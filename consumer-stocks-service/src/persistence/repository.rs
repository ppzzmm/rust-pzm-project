use diesel::prelude::*;

use crate::persistence::model::{StocksEntity, NewStocksEntity};
use crate::persistence::schema::{stocks};

// pub fn get_stocks(symbol: String, conn: &mut PgConnection) -> QueryResult<Vec<StocksEntity>> {
//     stocks::table.filter(stocks::symbol.eq(symbol)).get_result(conn)
// }

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
