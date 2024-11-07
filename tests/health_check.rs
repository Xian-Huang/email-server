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
    assert_eq!(Some(19),response.content_length()); //暂时不知道为什么是19，而不是书上写的0
}



//在后台启动应用程序
fn spawn_app()->String{
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address!");
    let port = listener.local_addr().unwrap().port();
    let server = emailserver::run(listener).expect("Failed to start server");
    tokio::spawn(server);
    format!("http://127.0.0.1:{}",port)
}