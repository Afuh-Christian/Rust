use actix_web::{get, post, web, Responder};
use entity::user::Model;
use sea_orm::{ActiveValue::Set};
use sea_orm::{ActiveModelTrait, Condition, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use crate::utils::api_response::ApiResponse;
use crate::utils::jwt::encode_jwt;
use crate::utils::{api_response, app_state::{self}};
use sea_orm::ColumnTrait;
use sha256::digest;

#[derive(Serialize, Deserialize)]
struct RegisterModel{
    name: String,
    email: String,
    password: String
    }

    #[derive(Serialize, Deserialize)]
    struct LoginModel{
        // name: String,
        email: String,
        password: String
        }
    
    #[post("/register")]
    pub async fn register(
    app_state: web::Data<app_state::AppState>,
    register_json: web::Json<RegisterModel>
    ) -> Result<ApiResponse, ApiResponse> {

    let user_model: Model = entity::user::ActiveModel {
    name: Set(register_json.name.clone()),
    email: Set(register_json.email.clone()),
    password: Set(digest(&register_json.password)),
    ..Default::default()
    }.insert(&app_state.db).await
    .map_err(|err| api_response::ApiResponse::new(500, format!("{}", err)))?;
    
    Ok(api_response::ApiResponse::new(200, format!("{}",user_model.id)))
    }
    



#[post("/login")]
pub async fn login(
app_state: web:: Data<app_state:: AppState>,
login_json: web:: Json<LoginModel>
) -> Result<ApiResponse, ApiResponse> {

let user_data = entity::user:: Entity::find()
.filter(
Condition::all()
.add(entity::user::Column:: Email.eq(&login_json.email))
.add(entity::user::Column:: Password.eq(digest(&login_json.password)))
).one(&app_state.db).await
.map_err(|err | ApiResponse::new(500,err.to_string()))?
.ok_or(ApiResponse::new(404, "User Not Found".to_owned()))?;


let token: String = encode_jwt(user_data.email, user_data.id)
.map_err(|err | ApiResponse::new(500,err.to_string()))?;

Ok(api_response::ApiResponse::new(200, format! ("{{ 'token':'{}' }}", token)))

}

    
    
    
    
    