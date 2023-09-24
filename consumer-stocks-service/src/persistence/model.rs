use diesel::prelude::*;

use crate::persistence::schema::{stocks, stocks_summary};

#[derive(Identifiable, Queryable, Associations)]
#[diesel(table_name = stocks)]
#[diesel(belongs_to(StocksEntity, foreign_key = user_id))]
pub struct StocksEntity {
    pub id: i32,
    pub symbol: String,
    pub shares: i32,
    pub price: String,
    pub percentage_change: String,
    pub action_type: String,
    pub user_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = stocks)]
pub struct NewStocksEntity {
    pub symbol: String,
    pub shares: i32,
    pub price: String,
    pub percentage_change: String,
    pub action_type: String,
    pub user_id: i32,
}

#[derive(Identifiable, Queryable, Associations)]
#[diesel(table_name = stocks_summary)]
#[diesel(belongs_to(StocksSummaryEntity, foreign_key = user_id))]
pub struct StocksSummaryEntity {
    pub id: i32,
    pub symbol: String,
    pub shares: i32,
    pub total_value: String,
    pub lowest_price: String,
    pub highest_price: String,
    pub average_price: String,
    pub price_by_hours: String,
    pub profit_loss: String,
    pub user_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = stocks_summary)]
pub struct NewStocksSummaryEntity {
    pub symbol: String,
    pub shares: i32,
    pub total_value: String,
    pub lowest_price: String,
    pub highest_price: String,
    pub average_price: String,
    pub profit_loss: String,
    pub price_by_hours: String,
    pub user_id: i32,
}
