use std::net::TcpListener;

use emailserver::configurations::{self, get_config, DatabaseSettings};
use sqlx::Executor;
use sqlx::{Connection, MySql, MySqlConnection, MySqlPool, Pool};
use uuid::Uuid;

#[tokio::test]
async fn database_connect() {
    let connect_url = get_config().unwrap().database.connection_string();
    let mut connection = MySqlConnection::connect(&connect_url)
        .await
        .expect("Faile to connect database");
    println!("connect to {connect_url} sccuess");
    let saved = sqlx::query!("SELECT email,name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("query error");
    println!("{}-{}", saved.name, saved.email);
    assert_eq!(saved.name, "test");
    assert_eq!(saved.email, "test");
}

#[tokio::test]
async fn health_check() {
    let testapp = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", &testapp.address))
        .send()
        .await
        .expect("Failed to send resquest!");
    assert!(response.status().is_success());
}

#[tokio::test]
async fn subscribe_returns_400() {
    let testapp = spawn_app().await;

    let client = reqwest::Client::new();

    let test_cases = [
        ("name=Li", "missing the email"),
        ("email=649295818%40qq.com", "missing the name"),
        ("", "missing the name and email"),
    ];
    for (body, msg) in test_cases {
        let response = client
            .post(format!("{}/subscribe", &testapp.address))
            .body(body)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .send()
            .await
            .expect("Failed to send resquest!");
        assert_eq!(
            response.status().as_u16(),
            400,
            "The API failed(400) for {}",
            msg
        );
    }
}

#[tokio::test]
async fn subscribe_returns_200() {
    let testapp = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=Li%20Dingyi&email=649295818%40qq.com";

    let response = client
        .post(format!("{}/subscribe", &testapp.address))
        .body(body)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await
        .expect("Failed to send resquest!");
    assert!(response.status().is_success());
}

struct TestApp {
    pub address: String,
    pub db_pool: Pool<MySql>,
}

//在后台启动应用程序
async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address!");
    let mut configuration = get_config().unwrap();
    configuration.database.dbname = Uuid::new_v4().as_simple().to_string();
    let address = format!("http://127.0.0.1:{}", listener.local_addr().unwrap().port());
    let connection = configure_database(&configuration.database).await;
    let server = emailserver::run(listener, connection.clone()).expect("Failed to start server");
    tokio::spawn(server);
    TestApp {
        address: address,
        db_pool: connection,
    }
}

async fn configure_database(config: &DatabaseSettings) -> MySqlPool {
    let mut connection = MySqlConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Fail to connect to database");

    connection
        .execute(format!(r#"CREATE DATABASE {}"#, config.dbname.as_str()).as_str())
        .await
        .expect("Failed to create database");

    let conn_pool = MySqlPool::connect(&config.connection_string())
        .await
        .expect("fail to connect to database");
    sqlx::migrate!("./migrations")
        .run(&conn_pool)
        .await
        .expect("Failed to migrate the database");
    conn_pool
}
