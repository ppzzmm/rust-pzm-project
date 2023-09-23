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
use crate::kafka_sockets;
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

    // async fn get_stock_by_symbol(&self, ctx: &Context<'_>, simbol: String) -> Option<Stock> {
    //     repository::get_stock(simbol, &mut get_conn_from_ctx(ctx))
    //     .ok()
    //     .map(|p| Stock::from(&p))
    // }
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
        let resul = common_utils::get_stock_from_nasdaq(stock.symbol.to_string());
        if !resul.success {
            return Err(Error{
                message: resul.message,
                source: None,
                extensions: None,
            });
        }
        let stock_by_symbol = repository::get_stock(stock.symbol.to_string(), &mut get_conn_from_ctx(ctx))
            .ok()
            .map(|p| Stock::from(&p));

        if !stock_by_symbol.is_none() {
            println!("PZM 0: {}", stock_by_symbol.unwrap().percentage_change);
            // println!("PZM 110: {}", stock_by_symbol.unwrap().action_type);
        }
        let stock_from_nasdaq = resul.stock.unwrap();
        let stock_data = stock_from_nasdaq.data;
        println!("PZM 1: {}", stock_data.symbol);
        println!("PZM 2: {}", stock_from_nasdaq.status.rCode);
        let last_sale_price = stock_data.primaryData.lastSalePrice.replace("$", "");
        let bid_price = stock_data.primaryData.bidPrice.replace("$", "");
        // let cien: BigDecimal = "100".parse().unwrap();
        let price = if bid_price != "N/A" {
            bid_price
        } else {
            // last_sale_price.parse::<BigDecimal>().unwrap()// * cien
            // BigDecimal::from_str(&last_sale_price.0.to_string()).expect("Can't get BigDecimal from string")// * cien
            last_sale_price
        };
        let percentage_change = stock_data.primaryData.percentageChange
            .replace("%", "")
            .replace("+", "")
            .replace("-", "");
        let new_stocks = NewStocksEntity {
            symbol: stock.symbol,
            shares: stock.shares,
            price: price,
            percentage_change: percentage_change,
            action_type: "buy".to_string(),
            user_id: 1,
        };

        common_utils::send_message_to_consumer(new_stocks.symbol.to_string(), stock.shares);
        let created_stock_entity = repository::create_stock(new_stocks, &mut get_conn_from_ctx(ctx))?;
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
        let consumer_group_id = kafka_sockets::get_kafka_consumer_group_id(kafka_consumer_counter);
        let consumer = kafka_sockets::create_consumer(consumer_group_id);

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
    shares: i32,
    price: String,
    percentage_change: String,
    action_type: String,
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

    async fn shares(&self) -> &i32 {
        &self.shares
    }

    async fn price(&self) -> &String {
        &self.price
    }

    async fn percentage_change(&self) -> &String {
        &self.percentage_change
    }

    async fn action_type(&self) -> &String {
        &self.action_type
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
    shares: i32,
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
            shares: entity.shares.into(),
            price: entity.price.clone(),
            percentage_change: entity.percentage_change.clone(),
            action_type: entity.action_type.clone(),
            user_id: entity.user_id.into(),
        }
    }
}
