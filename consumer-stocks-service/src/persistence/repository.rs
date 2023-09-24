use diesel::prelude::*;

use crate::persistence::model::{StocksEntity, NewStocksEntity, StocksSummaryEntity, NewStocksSummaryEntity};


pub fn get_stocks_by_symbol(symbol_data: String, conn: &mut PgConnection) -> Vec<StocksEntity> {
    use crate::persistence::schema::{stocks::dsl::*};
    let result = stocks
        .filter(symbol.eq(symbol_data))
        .load::<StocksEntity>(conn)
        .expect("Error loading students");
    result
}

pub fn get_stock_summary_by_symbol(symbol_data: String, conn: &mut PgConnection) ->  Option<Vec<StocksSummaryEntity>> {
    use crate::persistence::schema::{stocks_summary::dsl::*};
    let result = stocks_summary
        .filter(symbol.eq(symbol_data))
        .load::<StocksSummaryEntity>(conn)
        .expect("Error loading students");
    if result.len() == 0 {
        return None;
    }
    Some(result)
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

pub fn create_stock_summary(
    new_stocks_summary: NewStocksSummaryEntity,
    conn: &mut PgConnection,
) -> QueryResult<StocksSummaryEntity> {
    use crate::persistence::schema::{stocks_summary::dsl::*};

    let created_stock_summary: StocksSummaryEntity = diesel::insert_into(stocks_summary)
        .values(new_stocks_summary)
        .get_result(conn)?;

    Ok(created_stock_summary)
}

pub fn update_stock_summary(
    new_stocks_summary: NewStocksSummaryEntity,
    conn: &mut PgConnection,
) -> QueryResult<StocksSummaryEntity> {
    use crate::persistence::schema::{stocks_summary::dsl::*};

    let created_stock_summary: StocksSummaryEntity = diesel::update(stocks_summary.filter(symbol.eq(new_stocks_summary.symbol)))
        .set((
            shares.eq(new_stocks_summary.shares),
            total_value.eq(new_stocks_summary.total_value),
            lowest_price.eq(new_stocks_summary.lowest_price),
            highest_price.eq(new_stocks_summary.highest_price),
            average_price.eq(new_stocks_summary.average_price),
            price_by_hours.eq(new_stocks_summary.price_by_hours),
        ))
        .get_result(conn)?;

    Ok(created_stock_summary)
}
