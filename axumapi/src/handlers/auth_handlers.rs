use axum::Extension;
use axum::{http::StatusCode, Json};
use chrono::Utc;
use entity::user;
use sea_orm::{ActiveValue::Set, Condition, DatabaseConnection, EntityTrait};
use uuid::Uuid;

use crate::models::user_model::{LoginUserResponseModel, UserModel};
use crate::models::user_model::{CreateUser, LoginUser};
use crate::utils::api_errors::ApiError;
use crate::utils::jwts::encode_jwt;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;


pub async fn create_user_post(
    Extension(db) : Extension<DatabaseConnection>,
    Json(payload): Json<CreateUser>, // payload of type Json of structure CreateUser
) -> Result<String  , ApiError> {


    // check for database errors ... 
    let user_exist = entity::user::Entity::find().filter(user::Column::Email.eq(payload.email.clone())).one(&db).await
    .map_err(|err| ApiError{message:err.to_string() , status_code:StatusCode::INTERNAL_SERVER_ERROR , error_code:Some(50)})?;


    // check if user exists 
    if user_exist != None {
        return Err(ApiError{message: String::from("User Already Exists") , status_code:StatusCode::CONFLICT , error_code:Some(40)})
    }


    let user_model = user::ActiveModel {
        // id: Set(0),
        name: Set(payload.name.to_owned()) , //, You, 1 second ago * Uncommitted changes
        email: Set(payload.email.to_owned()),
        password: Set(payload.email.to_owned()),
        uuid: Set(Uuid::new_v4()),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };



    
    user_model.insert(&db).await
   .map_err(|err| ApiError{message:err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR , error_code: Some(50)})?;

    // db.close().await
    // .map_err(|err|ApiError{message:err.to_string() , status_code:StatusCode::INTERNAL_SERVER_ERROR , error_code: Some(40)})?;
   
   Ok("User created".to_string()) //, user.name.to_string() , user.email.to_string() , user.password.to_string() , user.created_at.to_string() , user.uuid.to_string()

}






pub async fn login_user_post(
    Extension(db) : Extension<DatabaseConnection>,
    Json(payload): Json<LoginUser>, // payload of type Json of structure CreateUser
) -> Result<Json<LoginUserResponseModel> , ApiError> {


     let user_model = user::Entity::find()
     .filter( Condition::all()
        .add(user::Column::Email.eq(payload.email))
        .add(user::Column::Password.eq(payload.password))
     ).one(&db)
     .await.
     map_err(|err| ApiError{message:err.to_string() , status_code:StatusCode::INTERNAL_SERVER_ERROR , error_code:Some(50)})?
     .ok_or(ApiError{message: "Not Found".to_string() , status_code:StatusCode::NOT_FOUND , error_code:Some(44)})?;
    //  .unwrap();

   
    let data: UserModel = UserModel::new(user_model);

   
    let encoded_token  = encode_jwt(data.email).map_err(|err|ApiError{status_code:StatusCode::INTERNAL_SERVER_ERROR , message: "Failed to encode credentials".to_owned() , error_code : Some(50)})?;
    
     let login_model = LoginUserResponseModel{
        token : encoded_token
    }; 
    
    //   db.close().await
    //       .map_err(|err| ApiError{message:err.to_string() , status_code:StatusCode::INTERNAL_SERVER_ERROR , error_code:Some(50)})?;


   Ok(Json(login_model))

}


