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

// pub fn get_stocks(user_ids: &[i32], conn: &mut PgConnection) -> QueryResult<Vec<StocksEntity>> {
//     stocks::table
//         .filter(stocks::user_id.eq_any(user_ids))
//         .load::<StocksEntity>(conn)
// }

pub fn create(
    new_user: NewUserEntity,
    mut new_stocks_entity: NewStocksEntity,
    conn: &mut PgConnection,
) -> QueryResult<UserEntity> {
    use crate::persistence::schema::{stocks::dsl::*, users::dsl::*};

    let created_user: UserEntity = diesel::insert_into(users)
        .values(new_user)
        .get_result(conn)?;

    new_stocks_entity.user_id = created_user.id;

    diesel::insert_into(stocks)
        .values(new_stocks_entity)
        .execute(conn)?;

    Ok(created_user)
}
