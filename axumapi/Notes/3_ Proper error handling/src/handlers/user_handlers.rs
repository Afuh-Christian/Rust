use axum::extract::Path;
use axum::Extension;
use axum::{http::StatusCode, response::IntoResponse, Json};
use entity::user::{self, ActiveModel, Model};
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait};
use uuid::Uuid;

use crate::models::user_model::{UpdateUser, UserModel};
use crate::utils::api_errors::ApiError;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;




pub async fn update_user_post(
    Extension(db) : Extension<DatabaseConnection>,
    Path(uuid) : Path<Uuid> , 
    Json(payload): Json<UpdateUser>, // payload of type Json of structure CreateUser
) -> Result<Json<UserModel> , ApiError> {

     let mut user_model:ActiveModel = user::Entity::find()
     .filter( user::Column::Uuid.eq(uuid)
     ).one(&db)
     .await
    //  .map_err(|err|ApiError{message:err.to_string() , status_code:StatusCode::INTERNAL_SERVER_ERROR , error_code:Some(50)})?
    //  .ok_or(ApiError{message : String::from("Not Found") , status_code: StatusCode::NOT_FOUND , error_code:Some(44)})?

     .map_err(|err| ApiError{message:err.to_string() , status_code:StatusCode::INTERNAL_SERVER_ERROR , error_code:Some(50)})?
     .ok_or(ApiError{message: "Not Found".to_string() , status_code:StatusCode::NOT_FOUND , error_code:Some(44)})?
    
     .into();

    // .into converts the model to an activeModel 
    // add mut to change the fields .

    // it this form you can update ... 

    user_model.name = Set(payload.name); 
    let user_data: Model = user_model.clone()
    .try_into().map_err(|err| ApiError{message:"Error converting ActiveModel to model".to_string() , status_code:StatusCode::INTERNAL_SERVER_ERROR , error_code:Some(50)})?;

    let data: UserModel = UserModel::new(user_data);

    user_model.update(&db).await
    .map_err(|err|ApiError{message:err.to_string() , status_code:StatusCode::INTERNAL_SERVER_ERROR , error_code:Some(50)})?;

    db.close().await
    .map_err(|err|ApiError{message:err.to_string() , status_code:StatusCode::INTERNAL_SERVER_ERROR , error_code:Some(50)})?;

    Ok( Json(data))


}




pub async fn delete_user(
       Extension(db) : Extension<DatabaseConnection>,
    Path(uuid) : Path<Uuid>
) -> Result<String , ApiError> {
    let user_model  =  entity::user::Entity::find().filter(user::Column::Uuid.eq(uuid)).one(&db).await
     .map_err(|err| ApiError{message:err.to_string() , status_code:StatusCode::INTERNAL_SERVER_ERROR , error_code:Some(50)})?
     .ok_or(ApiError{message: "Not Found".to_string() , status_code:StatusCode::NOT_FOUND , error_code:Some(44)})?;
    
    // .unwrap().unwrap();
    user::Entity::delete_by_id(user_model.id).exec(&db).await
    .map_err(|err| ApiError{message:err.to_string() , status_code:StatusCode::INTERNAL_SERVER_ERROR , error_code:Some(50)})?;
    // WE could still use the name because only the ownership of id was transferred .. 
    
    db.close().await
    .map_err(|err|ApiError{message:err.to_string() , status_code:StatusCode::INTERNAL_SERVER_ERROR , error_code:Some(50)})?;

    
    Ok(format!("Deleted {}", user_model.name))
}


pub async fn all_users(
       Extension(db) : Extension<DatabaseConnection>
) -> Result<Json<Vec<UserModel>> , ApiError> {

    let all_user : Vec<UserModel> =  entity::user::Entity::find().all(&db).await
    .map_err(|err|ApiError{message: err.to_string() , status_code:StatusCode::INTERNAL_SERVER_ERROR , error_code:Some(50)})?
    .into_iter().map(|item|UserModel::new(item)).collect();

    db.close().await
    .map_err(|err|ApiError{message:err.to_string() , status_code:StatusCode::INTERNAL_SERVER_ERROR , error_code:Some(50)})?;


    Ok(Json(all_user))

}