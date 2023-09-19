use bigdecimal::BigDecimal;
use diesel::prelude::*;

use crate::persistence::schema::{stocks, users};

#[derive(Identifiable, Queryable)]
#[diesel(table_name = users)]
pub struct UserEntity {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Identifiable, Queryable, Associations)]
#[diesel(table_name = stocks)]
#[diesel(belongs_to(StocksEntity, foreign_key = user_id))]
pub struct StocksEntity {
    pub id: i32,
    pub symbol: String,
    pub user_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUserEntity {
    pub name: String,
    pub email: String,
}

#[derive(Insertable)]
#[diesel(table_name = stocks)]
pub struct NewStocksEntity {
    pub symbol: String,
    pub user_id: i32,
}
