use std::net::TcpListener;
pub mod configurations;
use actix_web::{
    body::BoxBody,
    dev::Server,
    http::StatusCode,
    middleware::Logger,
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
    // 查询是否已经存在相同email的数据
    let search_result = sqlx::query(r#"SELECT id FROM subscriptions where email=?"#).bind( &form.email)
        .fetch_one(connection.get_ref())
        .await;
    if let Err(_) = search_result {
        let result = sqlx::query(
            r#"INSERT INTO subscriptions (email, name, subscribed_at) VALUES (?, ?, ?)"#,
        ).bind(&form.email).bind(&form.name).bind(now)
        .execute(connection.get_ref())
        .await;
        match result {
            Ok(_) => {
                println!("success to insert");
                HttpResponse::new(StatusCode::from_u16(200).unwrap())
                    .set_body(BoxBody::new("success to insert"))
            }
            Err(e) => {
                println!("Fail to insert:{}", e);
                HttpResponse::new(StatusCode::from_u16(500).unwrap())
                    .set_body(BoxBody::new("Fail to insert"))
            }
        }
    } else {
        HttpResponse::new(StatusCode::from_u16(500).unwrap())
            .set_body(BoxBody::new("Data already exists"))
    }
}

pub fn run(listener: TcpListener, pool: MySqlPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
