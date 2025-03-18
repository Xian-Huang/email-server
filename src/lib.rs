use std::net::TcpListener;
pub mod configurations;
use actix_web::{
    body::BoxBody,
    dev::Server,
    http::StatusCode,
    web::{self, Form},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use chrono::Utc;
use sqlx::MySqlPool;

#[derive(serde::Deserialize, Debug)]
struct FormData {
    name: String,
    email: String,
}

// 健康检查器
pub async fn health_check(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("world");
    format!("Hello {name}!")
}

async fn subscribe(form: Form<FormData>, connection: web::Data<MySqlPool>) -> HttpResponse {
    let now = format!("{}", Utc::now().format("%Y-%m-%d %H:%M:%S"));
    let result = sqlx::query!(
        r#"INSERT INTO subscriptions (email, name, subscribed_at) VALUES (?, ?, ?)"#,
        form.email,
        form.name,
        now
    )
    .execute(connection.get_ref())
    .await;
    match result {
        Ok(_) => {
            println!("success to insert");
            HttpResponse::new(StatusCode::from_u16(200).unwrap())
                .set_body(BoxBody::new("Success to insert"))
        }
        Err(e) => {
            println!("Fail to insert:{}", e);
            if e.as_database_error().unwrap().is_unique_violation() {
                HttpResponse::new(StatusCode::from_u16(200).unwrap())
                    .set_body(BoxBody::new("Success to insert"))
            } else {
                HttpResponse::new(StatusCode::from_u16(500).unwrap())
                    .set_body(BoxBody::new("Data exists,Fail to insert"))
            }
        }
    }
}

pub fn run(listener: TcpListener, pool: MySqlPool) -> Result<Server, std::io::Error> {
    let port = listener.local_addr().unwrap().port();

    let connection = web::Data::new(pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    println!("RUN IN:127.0.0.1:{}", port);
    Ok(server)
}
