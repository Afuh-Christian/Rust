use axum::extract::Path;
use axum::{http::StatusCode, response::IntoResponse, Json};
use entity::user::{self, ActiveModel, Model};
use sea_orm::{ActiveValue::Set, Database, DatabaseConnection, EntityTrait};
use uuid::Uuid;

use crate::models::user_model::{UpdateUser, UserModel};
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;




pub async fn update_user_post(
    Path(uuid) : Path<Uuid> , 
    Json(payload): Json<UpdateUser>, // payload of type Json of structure CreateUser
) -> impl IntoResponse {

    let database_url = "postgres://postgres:password@localhost:5432/axum_db?schema=public";

    let db: DatabaseConnection = Database::connect( database_url).await.unwrap();

     let mut user_model:ActiveModel = user::Entity::find()
     .filter( user::Column::Uuid.eq(uuid)
     ).one(&db)
     .await.unwrap().unwrap().into();

    // .into converts the model to an activeModel 
    // add mut to change the fields .

    // it this form you can update ... 

    user_model.name = Set(payload.name); 

    let user_data: Model = user_model.clone().try_into().unwrap();

    let data: UserModel = UserModel::new(user_data);

    user_model.update(&db).await.unwrap();



   db.close().await.unwrap();

   (StatusCode::ACCEPTED , Json(data))

}




pub async fn delete_user(
    Path(uuid) : Path<Uuid>
) -> impl IntoResponse {


    let database_url = "postgres://postgres:password@localhost:5432/axum_db?schema=public";

    let db: DatabaseConnection = Database::connect( database_url).await.unwrap();


    let user_model  =  entity::user::Entity::find().filter(user::Column::Uuid.eq(uuid)).one(&db).await.unwrap().unwrap();

    
    user::Entity::delete_by_id(user_model.id).exec(&db).await.unwrap();

    // WE could still use the name because only the ownership of id was transferred .. 

    (StatusCode::ACCEPTED , format!("Deleted {}", user_model.name))

}


pub async fn all_users() -> impl IntoResponse {

    let database_url = "postgres://postgres:password@localhost:5432/axum_db?schema=public";

    let db: DatabaseConnection = Database::connect( database_url).await.unwrap();

    let all_user : Vec<UserModel> =  entity::user::Entity::find().all(&db).await.unwrap().into_iter().map(|item|UserModel::new(item)).collect();

    (StatusCode::ACCEPTED , Json(all_user))

}