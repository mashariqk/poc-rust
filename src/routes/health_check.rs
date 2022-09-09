use actix_web::{HttpResponse, Responder};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct HealthStatus {
    status: String,
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthStatus {
        status: String::from("UP"),
    })
}
