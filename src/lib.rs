use std::net::TcpListener;
pub mod configurations;
use actix_web::{body::BoxBody, dev::Server, http::StatusCode, web::{self, Form}, App, HttpRequest, HttpResponse, HttpServer, Responder};
use sqlx::MySqlConnection;



#[derive(serde::Deserialize,Debug)]
struct FormData{
    name:String,
    email:String
}



// 健康检查器
pub async fn health_check(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("world");
    format!("Hello {name}!")
}


async fn subscribe(form:Form<FormData>,connection:web::Data<MySqlConnection>)->HttpResponse{
    HttpResponse::Ok().finish()

}

pub fn run(listener:TcpListener,dbconnection:MySqlConnection) -> Result<Server, std::io::Error> {
    let port= listener.local_addr().unwrap().port();

    let connection = web::Data::new(dbconnection);
    let server = HttpServer::new(move|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    println!("RUN IN:127.0.0.1:{}",port);
    Ok(server)
}
