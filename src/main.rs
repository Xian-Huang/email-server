use std::net::TcpListener;

use emailserver::{configurations::get_config, run};
use sqlx::MySqlPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let settings = get_config().unwrap();
    let connect_url = settings.database.connection_string();
    let dbconpool = MySqlPool::connect(&connect_url).await.unwrap();

    let address = format!("127.0.0.1:{}", settings.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind address!");
    run(listener, dbconpool)?.await
}
