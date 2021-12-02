use crate::utils::models::AppError;
use actix_web::{get, web, HttpResponse};
use serde_json::json;

#[get("/")]
async fn root() -> Result<HttpResponse, AppError> {
    Ok(web::HttpResponse::Ok().json(json!({
        "success": true,
        "message": "Kanbans REST API"
    })))
}
