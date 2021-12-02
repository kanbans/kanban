use serde_json::json;
use actix_web::{web, HttpResponse, get, Error};

#[get("/")]
async fn root() -> Result<HttpResponse, Error> {
    Ok(web::HttpResponse::Ok().json(json!({
        "success": true,
        "message": "Kanbans REST API"
    })))
}
