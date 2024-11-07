use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

// 健康检查器
pub async fn health_check(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("world");
    format!("Hello {name}!")
}

pub fn run(listener:TcpListener) -> Result<Server, std::io::Error> {
    let port= listener.local_addr().unwrap().port();

    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
    })
    .listen(listener)?
    .run();
    println!("RUN IN:127.0.0.1:{}",port);
    Ok(server)
}
