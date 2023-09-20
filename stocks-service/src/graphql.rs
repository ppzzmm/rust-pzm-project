use std::fmt::{self, Formatter, LowerExp};
use std::iter::Iterator;
use std::str::FromStr;
use std::sync::{Mutex};

use async_graphql::*;
use bigdecimal::{BigDecimal, ToPrimitive};
use futures::{Stream, StreamExt};
use rdkafka::{producer::FutureProducer, Message};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::get_conn_from_ctx;
use crate::kafka;
use crate::persistence::model::{StocksEntity, NewStocksEntity, UserEntity};
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
    async fn buy_stocks(&self, ctx: &Context<'_>, stock: StocksInput) -> Result<Stock> {
        let new_stocks = NewStocksEntity {
            symbol: stock.symbol,
            share_held: stock.amount,
            user_id: 1,
        };

        let created_stock_entity = repository::create_stock(new_stocks, &mut get_conn_from_ctx(ctx))?;

        let producer = ctx
            .data::<FutureProducer>()
            .expect("Can't get Kafka producer");
        let message = serde_json::to_string(&Stock::from(&created_stock_entity))
            .expect("Can't serialize a user");
        kafka::send_message(producer, &message).await;

        Ok(Stock::from(&created_stock_entity))
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
    amount: i32,
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
