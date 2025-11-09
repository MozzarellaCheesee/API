use actix_web::{get, HttpResponse, Responder};

#[get("/v1/auth/ping")]
pub async fn pings()-> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "message": "pong" }))
}