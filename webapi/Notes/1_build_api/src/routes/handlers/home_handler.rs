use actix_web::{get, HttpResponse, Responder};

use crate::utils::api_response;

#[get("")]
pub async fn hello() -> impl Responder {
    // HttpResponse::Ok().body("Hello world!")
    // format!("Hello world")

    api_response::ApiResponse::new(200, "Hello".to_string())
}

#[get("/test")]
pub async fn test() -> impl Responder {
    // HttpResponse::Ok().body("Test Me!")
    api_response::ApiResponse::new(200, "Test me back".to_string())
}
