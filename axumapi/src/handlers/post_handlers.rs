use axum::extract::Path;
use axum::http::StatusCode;
use axum::{Extension, Json};
use chrono::Utc;
use entity::{ post, user};
use sea_orm::{ActiveValue::Set, DatabaseConnection};
use sea_orm::{ActiveModelTrait, EntityTrait, JoinType, QueryFilter, QuerySelect};
use serde_json::Value;
use uuid::Uuid;
use crate::models::post_model::{CreatePostModel};
use crate::utils::api_errors::ApiError;
use sea_orm::ColumnTrait;


pub async fn create_post_handler(
    Extension(db): Extension<DatabaseConnection>, 
    Extension(identity) : Extension<user::Model> , 
    Json(payload) : Json<CreatePostModel>
)-> Result< Json<String> , ApiError >{

    let active_post_model = post::ActiveModel{
        image : Set(payload.image), 
        text: Set(payload.text) , 
        title: Set(payload.title) , 
        uuid: Set(Uuid::new_v4()) , 
        user_id: Set(identity.id) , 
        created_at: Set(Utc::now().naive_local()) , 
        ..Default::default() 
    } ; 

    // let post_model  = active_post_model.clone().try_into()
    // .map_err(|_|ApiError{message: "Failed to convert to model ".to_owned() , status_code: StatusCode::INTERNAL_SERVER_ERROR , error_code:Some(44)})?; 
    
    // let post : PostModel = PostModel::new(post_model);

    active_post_model.insert(&db).await.map_err(|err|ApiError{message: err.to_string() , status_code:StatusCode::INTERNAL_SERVER_ERROR , error_code: Some(50)})?;

    Ok(Json("Post Created ".to_string()))

}



pub async fn all_posts(
    Extension(db) : Extension<DatabaseConnection>
)-> Result<Json<Vec<Value>> , ApiError> {

    let all_posts = entity::post::Entity::find()
    .column_as(entity::user::Column::Name, "author")
    .column_as(entity::user::Column::Uuid , "author uuid")
    .join(JoinType::LeftJoin, 
    entity::post::Entity::belongs_to(entity::user::Entity)
    .from(entity::post::Column::UserId)
    .to(entity::user::Column::Id)
    .into())
    .into_json()
    .all(&db).await
    .map_err(|err|ApiError{message: err.to_string() , status_code: StatusCode::INTERNAL_SERVER_ERROR , error_code:Some(50)})?;
    
    Ok(Json(all_posts))

}




pub async fn single_post(
    Extension(db) : Extension<DatabaseConnection> , 
    Path(uuid) : Path<Uuid>
) -> Result<Json<serde_json::Value> , ApiError> {

    // Custom join
    let post_model: serde_json::Value = entity::post::Entity::find().filter(
        entity::post::Column::Uuid.eq(uuid)
    )
    .column_as(entity::user::Column::Name, "author")
    .column_as(entity::user::Column::Uuid, "author uuid")
    .join(JoinType::LeftJoin,
    entity::post::Entity::belongs_to(entity::user::Entity)
    .from(entity::post::Column::UserId)
    .to(entity::user::Column::Id)
    .into()
     )
     .into_json()
    .one(&db).await
    .map_err(|err|ApiError{message:err.to_string() , status_code: StatusCode::INTERNAL_SERVER_ERROR , error_code: Some(50)})?
    .ok_or(ApiError{message: "Failed to get post".to_owned() , status_code:StatusCode::INTERNAL_SERVER_ERROR , error_code: Some(54)})?;




    // built in join
    // let post_model = entity::post::Entity::find().filter(
    //     entity::post::Column::Uuid.eq(uuid)
    // )
    // .find_also_related(entity::user::Entity)
    // .one(&db).await
    // .map_err(|err|ApiError{message:err.to_string() , status_code: StatusCode::INTERNAL_SERVER_ERROR , error_code: Some(50)})?
    // .ok_or(ApiError{message: "Failed to get post".to_owned() , status_code:StatusCode::INTERNAL_SERVER_ERROR , error_code: Some(54)})?
    // .into();

    // let post : PostModel = PostModel::new(post_model) ; 

    

    Ok(Json(post_model))

}