use std::env;

use dotenv::dotenv;
use jsonpath_lib as jsonpath;
use testcontainers::clients::Cli;
use testcontainers::images::postgres::Postgres;
use testcontainers::{Container, RunnableImage};

use stocks_service::persistence::connection::{create_connection_pool, PgPool};
use stocks_service::run_migrations;

pub fn setup(docker: &Cli) -> (Container<Postgres>, PgPool) {
    dotenv().ok();
    let pg_container = setup_database(docker);
    let pool = create_connection_pool();
    run_migrations(&mut pool.get().expect("Can't get DB connection"));
    (pg_container, pool)
}

fn setup_database(docker: &Cli) -> Container<Postgres> {
    let pg_container = docker.run(get_pg_image());
    let pg_port = pg_container.get_host_port_ipv4(5432);
    env::set_var(
        "DATABASE_URL",
        format!("postgres://postgres:postgres@localhost:{}/postgres", pg_port),
    );
    pg_container
}

fn get_pg_image() -> RunnableImage<Postgres> {
    RunnableImage::from(Postgres::default())
        .with_env_var(("POSTGRES_DB", "postgres"))
        .with_env_var(("POSTGRES_PASSWORD", "postgres"))
}

pub fn check_user(
    user_json: &serde_json::Value,
    id: i32,
    name: &str,
    email: &str,
) {
    fn check_property(
        user_json: &serde_json::Value,
        property_name: &str,
        property_expected_value: &str,
    ) {
        let json_path = format!("$.{}", property_name);
        assert_eq!(
            property_expected_value,
            jsonpath::select(user_json, &json_path).expect("Can't get property")[0]
                .as_str()
                .expect("Can't get property as str")
        );
    }
    check_property(user_json, "id", &id.to_string());
    check_property(user_json, "name", name);
    check_property(user_json, "email", email);
}