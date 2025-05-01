use actix_web::{get, web, Responder};
use sea_orm::{ConnectionTrait, QueryResult, Statement};

use crate::utils::{api_response, app_state::AppState};

#[get("")]
pub async fn hello() -> impl Responder {
    // HttpResponse::Ok().body("Hello world!")
    // format!("Hello world")

    api_response::ApiResponse::new(200, "Hello".to_string())
}

#[get("/test")]
pub async fn test(app_state : web::Data<AppState>) -> impl Responder {
    // HttpResponse::Ok().body("Test Me!")

    let res: Vec<QueryResult> = app_state.db
   .query_all(Statement::from_string(sea_orm::DatabaseBackend::Postgres, "Select * from user; ")).await.unwrap();
    
    
    api_response::ApiResponse::new(200, "Test me back".to_string())
}
