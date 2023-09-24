use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_connection_pool() -> PgPool {
    #[allow(unused_assignments)]
    let mut db_url = "".to_string();
    match env::var("DATABASE_URL") {
        Ok(stream) => {
            db_url = format!("{}", stream);
        }
        Err(_e) => {
            db_url = "postgres://postgres:postgres@localhost:5432/postgres".to_string();
        }
    };
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
