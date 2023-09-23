use diesel::prelude::*;

use crate::persistence::schema::{stocks};

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
