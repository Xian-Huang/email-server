use std::net::TcpListener;

use emailserver::{configurations::get_config, run};


#[tokio::main]
async fn main()->std::io::Result<()> {
    let settings = get_config().unwrap();
    let address = format!("127.0.0.1:{}",settings.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind address!");
    run(listener)?.await
}
