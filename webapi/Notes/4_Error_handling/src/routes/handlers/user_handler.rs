use actix_web:: {get, web};
use crate::utils:: {api_response::{self, ApiResponse}, app_state};

#[get("")]
pub async fn user(
 app_state: web:: Data<app_state:: AppState>
)-> Result<ApiResponse, ApiResponse> {
Ok(api_response:: ApiResponse:: new(200, "Verifyed user".to_string()))
}