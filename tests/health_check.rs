use std::net::TcpListener;

use emailserver::configurations::get_config;
use sqlx::{Connection, MySqlConnection, MySqlPool};


#[tokio::test]
async fn database_connect(){
    let connect_url = get_config().unwrap().database.connext_string();
    let mut connection = MySqlConnection::connect(&connect_url).await.expect("Faile to connect database");
    println!("connect to {connect_url} sccuess");
    let saved = sqlx::query!("SELECT email,name FROM subscriptions",).fetch_one(&mut connection).await.expect("query error");
    println!("{}-{}",saved.name,saved.email);
    assert_eq!(saved.name,"test");
    assert_eq!(saved.email,"test");
}





#[tokio::test]
async fn health_check(){

    let address = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client.get(format!("{}/health_check",&address))
    .send()
    .await
    .expect("Failed to send resquest!");
    assert!(response.status().is_success());
}



#[tokio::test]
async fn subscribe_returns_400(){

    let address = spawn_app().await;

    let client = reqwest::Client::new();

    let test_cases = [
        ("name=Li","missing the email"),
        ("email=649295818%40qq.com","missing the name"),
        ("","missing the name and email")
    ];
    for (body,msg) in test_cases{
        let response = client.post(format!("{}/subscribe",&address))
        .body(body)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await
        .expect("Failed to send resquest!");
        assert_eq!(response.status().as_u16(), 400,"The API failed(400) for {}",msg);
    }
}

#[tokio::test]
async fn subscribe_returns_200(){

    let address = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=Li%20Dingyi&email=649295818%40qq.com";

    let response = client.post(format!("{}/subscribe",&address))
    .body(body)
    .header("Content-Type", "application/x-www-form-urlencoded")
    .send()
    .await
    .expect("Failed to send resquest!");
    assert!(response.status().is_success());
}


//在后台启动应用程序
async fn spawn_app()->String{
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address!");
    let current_url = get_config().unwrap().database.connext_string();
    let port = listener.local_addr().unwrap().port();
    let connection = MySqlPool::connect(&current_url).await.expect("fail to connect to database");
    let server = emailserver::run(listener,connection).expect("Failed to start server");
    tokio::spawn(server);
    format!("http://127.0.0.1:{}",port)
}