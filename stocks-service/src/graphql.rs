use std::collections::HashMap;
use std::fmt::{self, Formatter, LowerExp};
use std::iter::Iterator;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use async_graphql::dataloader::{DataLoader, Loader};
use async_graphql::*;
use bigdecimal::{BigDecimal, ToPrimitive};
use futures::{Stream, StreamExt};
use rdkafka::{producer::FutureProducer, Message};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::get_conn_from_ctx;
use crate::kafka;
use crate::persistence::connection::PgPool;
use crate::persistence::model::{StocksEntity, NewStocksEntity, NewUserEntity, UserEntity};
use crate::persistence::repository;

pub type AppSchema = Schema<Query, Mutation, Subscription>;

pub struct Query;

#[Object]
impl Query {
    async fn get_users(&self, ctx: &Context<'_>) -> Vec<User> {
        repository::get_all(&mut get_conn_from_ctx(ctx))
            .expect("Can't get users")
            .iter()
            .map(User::from)
            .collect()
    }

    async fn get_user(&self, ctx: &Context<'_>, id: ID) -> Option<User> {
        find_user_by_id_internal(ctx, id)
    }

    #[graphql(entity)]
    async fn find_user_by_id(&self, ctx: &Context<'_>, id: ID) -> Option<User> {
        find_user_by_id_internal(ctx, id)
    }
}

fn find_user_by_id_internal(ctx: &Context<'_>, id: ID) -> Option<User> {
    let id = id
        .to_string()
        .parse::<i32>()
        .expect("Can't get id from String");
    repository::get(id, &mut get_conn_from_ctx(ctx))
        .ok()
        .map(|p| User::from(&p))
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_user(&self, ctx: &Context<'_>, user: UserInput) -> Result<User> {
        let new_user = NewUserEntity {
            name: user.name,
            email: user.email,
        };

        let stocks = user.stocks;
        let new_user_stocks = NewStocksEntity {
            symbol: stocks.symbol,
            user_id: 0,
        };

        let created_user_entity =
            repository::create(new_user, new_user_stocks, &mut get_conn_from_ctx(ctx))?;

        let producer = ctx
            .data::<FutureProducer>()
            .expect("Can't get Kafka producer");
        let message = serde_json::to_string(&User::from(&created_user_entity))
            .expect("Can't serialize a user");
        kafka::send_message(producer, &message).await;

        Ok(User::from(&created_user_entity))
    }
}

pub struct Subscription;

#[Subscription]
impl Subscription {
    async fn latest_user<'ctx>(
        &self,
        ctx: &'ctx Context<'_>,
    ) -> impl Stream<Item = User> + 'ctx {
        let kafka_consumer_counter = ctx
            .data::<Mutex<i32>>()
            .expect("Can't get Kafka consumer counter");
        let consumer_group_id = kafka::get_kafka_consumer_group_id(kafka_consumer_counter);
        let consumer = kafka::create_consumer(consumer_group_id);

        async_stream::stream! {
            let mut stream = consumer.stream();

            while let Some(value) = stream.next().await {
                yield match value {
                    Ok(message) => {
                        let payload = message.payload().expect("Kafka message should contain payload");
                        let message = String::from_utf8_lossy(payload).to_string();
                        serde_json::from_str(&message).expect("Can't deserialize a user")
                    }
                    Err(e) => panic!("Error while Kafka message processing: {}", e)
                };
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct User {
    id: ID,
    name: String,
    email: String,
}

#[Object]
impl User {
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn name(&self) -> &String {
        &self.name
    }

    async fn email(&self) -> &String {
        &self.email
    }

    // #[graphql(deprecation = "Now it is not in doubt. Do not use this field")]
    // async fn is_rotating_around_sun(&self) -> bool {
    //     true
    // }

    // async fn stocks(&self, ctx: &Context<'_>) -> Result<Stocks> {
    //     let data_loader = ctx
    //         .data::<DataLoader<StocksLoader>>()
    //         .expect("Can't get data loader");
    //     let user_id = self
    //         .id
    //         .to_string()
    //         .parse::<i32>()
    //         .expect("Can't convert id");
    //     let stocks = data_loader.load_one(user_id).await?;
    //     stocks.ok_or_else(|| "Not found".into())
    // }
}


#[derive(Serialize, Deserialize)]
struct Stock {
    id: ID,
    symbol: String,
    user_id: ID,
}

#[Object]
impl Stock {
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn symbol(&self) -> &String {
        &self.symbol
    }

    async fn user_id(&self) -> &ID {
        &self.user_id
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Enum, Display, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
enum UserType {
    TerrestrialUser,
    GasGiant,
    IceGiant,
    DwarfUser,
}

// #[derive(Interface, Clone)]
// #[graphql(
//     field(name = "unit_cost", ty = "&CustomBigDecimal"),
// )]
// pub enum Stocks {
//     InhabitedUserStocks(InhabitedUserStocks),
//     UninhabitedUserStocks(UninhabitedUserStocks),
// }

// #[derive(SimpleObject, Clone)]
// pub struct InhabitedUserStocks {
//     unit_cost: CustomBigDecimal,
// }

// #[derive(SimpleObject, Clone)]
// pub struct UninhabitedUserStocks {
//     unit_cost: CustomBigDecimal,
// }

#[derive(Clone)]
pub struct CustomBigInt(BigDecimal);

#[Scalar(name = "BigInt")]
impl ScalarType for CustomBigInt {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(s) => {
                let parsed_value = BigDecimal::from_str(&s)?;
                Ok(CustomBigInt(parsed_value))
            }
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(format!("{:e}", &self))
    }
}

impl LowerExp for CustomBigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let val = &self.0.to_f64().expect("Can't convert BigDecimal");
        LowerExp::fmt(val, f)
    }
}

#[derive(Clone)]
pub struct CustomBigDecimal(BigDecimal);

#[Scalar(name = "BigDecimal")]
impl ScalarType for CustomBigDecimal {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(s) => {
                let parsed_value = BigDecimal::from_str(&s)?;
                Ok(CustomBigDecimal(parsed_value))
            }
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}

#[derive(InputObject)]
struct UserInput {
    name: String,
    email: String,
    stocks: StocksInput,
}

#[derive(InputObject)]
struct StocksInput {
    symbol: String,
}

impl From<&UserEntity> for User {
    fn from(entity: &UserEntity) -> Self {
        User {
            id: entity.id.into(),
            name: entity.name.clone(),
            email: entity.email.clone(),
        }
    }
}

impl From<&StocksEntity> for Stock {
    fn from(entity: &StocksEntity) -> Self {
        Stock {
            id: entity.id.into(),
            symbol: entity.symbol.clone(),
            user_id: entity.user_id.into(),
        }
    }
}

// pub struct StocksLoader {
//     pub pool: Arc<PgPool>,
// }

// #[async_trait::async_trait]
// impl Loader<i32> for StocksLoader {
//     type Value = Stocks;
//     type Error = Error;

//     async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
//         let mut conn = self.pool.get()?;
//         let stocks = repository::get_stocks(keys, &mut conn)?;

//         Ok(stocks
//             .iter()
//             .map(|stocks_entity| (stocks_entity.user_id, Stocks::from(stocks_entity)))
//             .collect::<HashMap<_, _>>())
//     }
// }
