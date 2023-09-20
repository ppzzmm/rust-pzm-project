use std::env;

use actix_web::{test, web, App};
use jsonpath_lib as jsonpath;
use serde::{Deserialize, Serialize};
use serde_json::Map;
use testcontainers::clients::Cli;

use stocks_service::{configure_service, create_schema_with_context};

mod common;

#[actix_rt::test]
async fn test_create_user() {
    let docker = Cli::default();
    let (_pg_container, pool) = common::setup(&docker);

    let service = test::init_service(
        App::new()
            .configure(configure_service)
            .app_data(web::Data::new(create_schema_with_context(pool))),
    )
    .await;

    let mutation = r#"
        mutation(
            $name: String!
            $email: String!
        ) {
            createUser(
                user: {
                    name: $name
                    email: $email
                }
            ) {
                id
                name
                email
            }
        }
        "#
    .to_string();

    let mut variables = Map::new();
    variables.insert("name".to_string(), "Test PZM".into());
    variables.insert("email".to_string(), "pablo.zuniga.mata@gmail.com".into());

    let request_body = GraphQLCustomRequest {
        query: mutation,
        variables,
    };

    let request = test::TestRequest::post()
        .uri("/")
        .set_json(&request_body)
        .to_request();

    let response: GraphQLCustomResponse = test::call_and_read_body_json(&service, request).await;

    let response_data = response.data.expect("Response doesn't contain data");

    let created_user_json = jsonpath::select(&response_data, "$.createUser")
        .expect("Can't get created user by JSON path")[0];

    common::check_user(created_user_json, 9, "Test PZM", "pablo.zuniga.mata@gmail.com");
}

#[derive(Serialize)]
struct GraphQLCustomRequest {
    query: String,
    variables: Map<String, serde_json::Value>,
}

#[derive(Deserialize)]
struct GraphQLCustomResponse {
    data: Option<serde_json::Value>,
    #[allow(dead_code)]
    errors: Option<serde_json::Value>,
}