use bigdecimal::{BigDecimal};
use serde::{Deserialize, Serialize};
use async_graphql::*;
use crate::persistence::model::{StocksEntity, NewStocksEntity, StocksSummaryEntity, NewStocksSummaryEntity};
use crate::persistence::repository;
use crate::persistence::connection::create_connection_pool;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::*;
use crate::persistence::schema::stocks_summary;

pub fn save_stock(symbol: String, shares: i32, price: String, percentage_change: String, action: String) {
    let new_stocks = NewStocksEntity {
        symbol: symbol,
        shares: shares,
        price: price,
        percentage_change: percentage_change,
        action_type: action,
        user_id: 1,
    };
    let pool = create_connection_pool();
    repository::create_stock(
        new_stocks, &mut pool.get().expect("Can't get DB connection")
    ).expect("Error to create a stock");
}

pub fn calculate_stock_summary(symbol: String, shares: i32, price: String, percentage_change: String) {
    let pool = create_connection_pool();
    let stocks_by_symbol = repository::get_stocks_by_symbol(
        symbol.to_string(), &mut pool.get().expect("Can't get DB connection")
    );
    let mut total_price:BigDecimal = "0.0".parse().unwrap();
    let mut total_percentage_change:BigDecimal = "0.0".parse().unwrap();
    let mut total_price_to_get_average_price:BigDecimal = "0.0".parse().unwrap();
    let mut total_shares:i32 = 0;
    let mut i:usize = 0;
    let mut prices: Vec<BigDecimal> = Vec::new();
    let mut average_price:BigDecimal = "0.0".parse().unwrap();
    let mut profit_loss:BigDecimal = "0.0".parse().unwrap();
    let mut prices_by_hour = "".to_string();
    if stocks_by_symbol.len() != 0 {
        while i<stocks_by_symbol.len()
        {
            prices.push(stocks_by_symbol[i].price.parse::<BigDecimal>().unwrap());
            if stocks_by_symbol[i].action_type == "buy" {
                total_price += &stocks_by_symbol[i].price.parse::<BigDecimal>().unwrap() *
                               &stocks_by_symbol[i].shares.to_string().parse::<BigDecimal>().unwrap();
                total_shares += stocks_by_symbol[i].shares;
            } else {
                total_price -= &stocks_by_symbol[i].price.parse::<BigDecimal>().unwrap() *
                               &stocks_by_symbol[i].shares.to_string().parse::<BigDecimal>().unwrap();
                total_shares -= stocks_by_symbol[i].shares;
            }
            total_percentage_change += &stocks_by_symbol[i].percentage_change.parse::<BigDecimal>().unwrap();
            total_price_to_get_average_price += &stocks_by_symbol[i].price.parse::<BigDecimal>().unwrap();
            i = i + 1;
        }
        average_price = total_price_to_get_average_price / stocks_by_symbol.len().to_string().parse::<BigDecimal>().unwrap();
        profit_loss = total_percentage_change / stocks_by_symbol.len().to_string().parse::<BigDecimal>().unwrap();
        prices_by_hour = calculate_prices_by_hour(symbol.to_string());
    } else {
        prices.push(price.parse::<BigDecimal>().unwrap());
        total_price = price.parse::<BigDecimal>().unwrap();
        total_price_to_get_average_price = price.parse::<BigDecimal>().unwrap();
        total_shares = shares;
        average_price = price.parse::<BigDecimal>().unwrap();
        profit_loss = percentage_change.parse::<BigDecimal>().unwrap();
    }  
    let total_price_result = total_price / "1".to_string().parse::<BigDecimal>().unwrap();    
    let new_stocks_summary = NewStocksSummaryEntity{
        total_value: total_price_result.with_prec(2).to_string(),
        symbol: symbol.to_string(),
        shares: total_shares,
        lowest_price: prices.iter().min().unwrap().with_prec(2).to_string(),
        highest_price: prices.iter().max().unwrap().with_prec(2).to_string(),
        average_price: average_price.with_prec(2).to_string(),
        profit_loss: profit_loss.to_string(),
        price_by_hours: prices_by_hour.to_string(),
        user_id: 1
    };
    let stock_summary_by_symbol = repository::get_stock_summary_by_symbol(
        symbol.to_string(), &mut pool.get().expect("Can't get DB connection")
    );
    match stock_summary_by_symbol {
        None => {
            repository::create_stock_summary(
                new_stocks_summary, &mut pool.get().expect("Can't get DB connection")
            ).expect("Error to create a stock summary");
        }
        _ => {
            repository::update_stock_summary(
                new_stocks_summary, &mut pool.get().expect("Can't get DB connection")
            ).expect("Error to update a stock summary");
        },
    }
}

#[derive(Debug, QueryableByName)]
#[table_name = "stocks_summary"]
pub struct StocksByHours {
   pub symbol: String,
   #[sql_type = "Text"]
   pub price : String,
   #[sql_type = "Text"]
   pub hour : String,
}

pub fn calculate_prices_by_hour(symbol: String) -> String {
    let pool = create_connection_pool();
    let query = format!(
        "
        SELECT symbol,
        ROUND(SUM(CAST (price AS DECIMAL))/COUNT(price),2) || '' AS price,
            substring(generate_series((TO_CHAR(MIN(created_at),'yyyy-mm-dd HH12:MI:SS'))::timestamp
                         ,(TO_CHAR(MAX(created_at),'yyyy-mm-dd HH12:MI:SS'))::timestamp
                         ,interval '1 hour')::text, 12, 5) AS hour
        FROM stocks
        WHERE symbol = '{}'
	    GROUP BY symbol",
        symbol
    );
    let results = sql_query(query)
        .load::<StocksByHours>(&mut pool.get().expect("Can't get DB connection"))
        .unwrap();
    println!("{:?}",results);
    let mut stocks_by_hour = "".to_string();
    let mut i:usize = 0;
    while i<results.len()
    {
        if i == 0 {
            stocks_by_hour = format!("{} - {}", results[i].hour.to_string(), results[i].price.to_string());
        } else {
            stocks_by_hour = format!("{}, {} - {}", stocks_by_hour, results[i].hour.to_string(), results[i].price.to_string());
        }
        i = i + 1;
    }
    stocks_by_hour
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

#[derive(Serialize, Deserialize)]
struct StockSummary {
    id: ID,
    symbol: String,
    shares: i32,
    total_value: String,
    lowest_price: String,
    highest_price: String,
    average_price: String,
    price_by_hours: String,
    profit_loss: String,
    user_id: ID,
}

#[Object]
impl StockSummary {
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn symbol(&self) -> &String {
        &self.symbol
    }

    async fn shares(&self) -> &i32 {
        &self.shares
    }

    async fn total_value(&self) -> &String {
        &self.total_value
    }

    async fn lowest_price(&self) -> &String {
        &self.lowest_price
    }

    async fn highest_price(&self) -> &String {
        &self.highest_price
    }

    async fn average_price(&self) -> &String {
        &self.average_price
    }

    async fn price_by_hours(&self) -> &String {
        &self.price_by_hours
    }

    async fn profit_loss(&self) -> &String {
        &self.profit_loss
    }

    async fn user_id(&self) -> &ID {
        &self.user_id
    }
}

impl From<&StocksSummaryEntity> for StockSummary {
    fn from(entity: &StocksSummaryEntity) -> Self {
        StockSummary {
            id: entity.id.into(),
            symbol: entity.symbol.clone(),
            shares: entity.shares.into(),
            total_value: entity.total_value.clone(),
            lowest_price: entity.lowest_price.clone(),
            highest_price: entity.highest_price.clone(),
            average_price: entity.average_price.clone(),
            price_by_hours: entity.price_by_hours.clone(),
            profit_loss: entity.profit_loss.clone(),
            user_id: entity.user_id.into(),
        }
    }
}
