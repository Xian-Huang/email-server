use actix_web::{HttpRequest, HttpResponse, Responder};

// 健康检查器
pub async fn health_check(_req:HttpRequest)->impl Responder{
    HttpResponse::Ok().finish()
}
