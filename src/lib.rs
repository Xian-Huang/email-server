use std::net::TcpListener;

use actix_web::{body::BoxBody, dev::Server, http::StatusCode, web::{self, Form}, App, HttpRequest, HttpResponse, HttpServer, Responder};



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


async fn subscribe(form:Form<FormData>)->HttpResponse{
    // HttpResponse::with_body(StatusCode::OK, BoxBody::new(format!("{:?}",form)))
    HttpResponse::Ok().finish()

}

pub fn run(listener:TcpListener) -> Result<Server, std::io::Error> {
    let port= listener.local_addr().unwrap().port();

    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))

    })
    .listen(listener)?
    .run();
    println!("RUN IN:127.0.0.1:{}",port);
    Ok(server)
}
