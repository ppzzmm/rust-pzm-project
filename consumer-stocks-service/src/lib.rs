use bigdecimal::{BigDecimal, ToPrimitive};
use serde::{Deserialize, Serialize};
use async_graphql::*;

pub mod persistence;
use crate::persistence::model::{StocksEntity, NewStocksEntity};
use crate::persistence::repository;
use crate::persistence::connection::create_connection_pool;

pub fn buy_stocks(symbol: String, shares: String) {
    println!("Test PZM 01: {}", symbol.to_string());
    let result = common_utils::get_stock_from_nasdaq(symbol.to_string());
    if result.success == false {
        return;
    }
    let stock_from_nasdaq = result.stock.unwrap();
    let stock_data = stock_from_nasdaq.data;
    println!("PZM consumer 1: {}", stock_data.symbol);
    println!("PZM consumer 2: {}", stock_from_nasdaq.status.rCode);
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
        symbol: symbol.to_string(),
        shares: shares.parse::<i32>().unwrap(),
        price: price,
        percentage_change: percentage_change,
        action_type: "buy PZM 1".to_string(),
        user_id: 1,
    };
    let pool = create_connection_pool();
    repository::create_stock(
        new_stocks, &mut pool.get().expect("Can't get DB connection")
    );
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
