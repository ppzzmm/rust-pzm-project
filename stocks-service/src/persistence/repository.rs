use diesel::prelude::*;

use crate::persistence::model::{StocksEntity, NewStocksEntity, NewUserEntity, UserEntity};
use crate::persistence::schema::{stocks, users};

pub fn get_all(conn: &mut PgConnection) -> QueryResult<Vec<UserEntity>> {
    use crate::persistence::schema::users::dsl::*;

    users.load(conn)
}

pub fn get(id: i32, conn: &mut PgConnection) -> QueryResult<UserEntity> {
    users::table.find(id).get_result(conn)
}

pub fn get_stock(symbol: String, conn: &mut PgConnection) -> QueryResult<StocksEntity> {
    stocks::table.filter(stocks::symbol.eq(symbol)).get_result(conn)
}

pub fn create_stock(
    new_stocks: NewStocksEntity,
    conn: &mut PgConnection,
) -> QueryResult<StocksEntity> {
    use crate::persistence::schema::{stocks::dsl::*};

    let created_stock: StocksEntity = diesel::insert_into(stocks)
        .values(new_stocks)
        .get_result(conn)?;

    Ok(created_stock)
}
