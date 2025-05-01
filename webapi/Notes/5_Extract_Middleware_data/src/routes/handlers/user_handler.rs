use actix_web:: {get, post, web};
use sea_orm::{ActiveValue::Set, EntityTrait};
use serde::{Deserialize, Serialize};
use crate::utils:: {api_response::{self, ApiResponse}, app_state, jwt::Claims};
use sea_orm::{IntoActiveModel , ActiveModelTrait};

#[derive(Serialize, Deserialize)]
struct UpdateUserModel {
name: String
}



#[get("")]
pub async fn user(
 app_state: web:: Data<app_state:: AppState> , 
 claims_data : Claims
)-> Result<api_response::ApiResponse, api_response::ApiResponse> {

let user_model = entity::user::Entity::find_by_id(claims_data.id)
.one(&app_state.db).await 
.map_err(|err | api_response:: ApiResponse:: new(500, err.to_string()))?
.ok_or(api_response:: ApiResponse:: new(404, "User not found".to_owned()))?;


Ok(api_response:: ApiResponse:: new(200, format!("{{ 'namej' : {} , 'emailh' : {} }}" , user_model.name, user_model.email)))
}




#[post("update")]  
pub async fn update_user(  
app_state: web:: Data<app_state:: AppState>,  
user_data: web:: Json<UpdateUserModel>,  
claim_data: Claims  
) -> Result<api_response::ApiResponse, api_response:: ApiResponse>  {
let mut user_model = entity:: user:: Entity:: find_by_id(claim_data.id)  
.one(&app_state.db).await 
.map_err(|err| api_response:: ApiResponse:: new (500, err.to_string()))? 
.ok_or(api_response:: ApiResponse:: new (404, "User Not Found".to_owned()))?   
.into_active_model();  
user_model.name = Set(user_data.name.clone());  
user_model.update(&app_state.db).await  
.map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;  
Ok (api_response:: ApiResponse:: new (200, "User Updated".to_owned()))  
}  
