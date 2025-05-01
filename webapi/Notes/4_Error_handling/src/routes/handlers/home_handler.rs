use actix_web::{get, web, Responder};
use sea_orm::{ConnectionTrait, QueryResult, Statement};

use crate::utils::{api_response::{self, ApiResponse}, app_state::AppState};

#[get("")]
pub async fn hello() -> Result<ApiResponse, ApiResponse> {
    Ok(api_response::ApiResponse::new(200, "Hello".to_string()))
}

#[get("/test")]
pub async fn test(app_state : web::Data<AppState>) -> Result<ApiResponse, ApiResponse> {
    // HttpResponse::Ok().body("Test Me!")

    let res: Vec<QueryResult> = app_state.db
   .query_all(Statement::from_string(sea_orm::DatabaseBackend::Postgres, "Select * from user; ")).await
    .map_err(|err| ApiResponse::new(500, format!("{}", err)))?
   ;
    
    
    Ok(api_response::ApiResponse::new(200, "Test me back".to_string()))
}
