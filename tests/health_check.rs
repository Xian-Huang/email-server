use std::net::TcpListener;




#[tokio::test]
async fn health_check(){

    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client.get(format!("{}/health_check",&address))
    .send()
    .await
    .expect("Failed to send resquest!");
    assert!(response.status().is_success());
}



#[tokio::test]
async fn subscribe_returns_400(){

    let address = spawn_app();

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

    let address = spawn_app();
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
fn spawn_app()->String{
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address!");
    let port = listener.local_addr().unwrap().port();
    let server = emailserver::run(listener).expect("Failed to start server");
    tokio::spawn(server);
    format!("http://127.0.0.1:{}",port)
}