use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use emailserver::health_check;


async fn greet(req:HttpRequest)->impl Responder{
    let name = req.match_info().get("name").unwrap_or("world");
    format!("Hello {name}!")
}

#[tokio::main]
pub async fn main()->std::io::Result<()> {
    HttpServer::new(||{
        App::new()
        .route("/", web::get().to(greet))
        .route("/{name}", web::get().to(greet))
        .route("/health_check", web::get().to(health_check))

    }).bind("127.0.0.1:8000")?
    .run()
    .await
}


