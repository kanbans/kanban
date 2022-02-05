use actix_web::{get, web, HttpResponse};
use serde_json::json;

#[get("/")]
async fn root() -> HttpResponse {
    web::HttpResponse::Ok().json(json!({
        "success": true,
        "message": "Kanbans REST API"
    }))
}
