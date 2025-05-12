use axum::{http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use entity::user;
use sea_orm::{ActiveValue::Set, Condition, Database, DatabaseConnection, EntityTrait};
use uuid::Uuid;

use crate::models::user_model::UserModel;
use crate::models::user_model::{CreateUser, LoginUser};
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;


pub async fn create_user_post(
    Json(payload): Json<CreateUser>, // payload of type Json of structure CreateUser
) -> impl IntoResponse {

    let database_url = "postgres://postgres:password@localhost:5432/axum_db?schema=public";

    let db: DatabaseConnection = Database::connect( database_url).await.unwrap();

    let user_model = user::ActiveModel {
        // id: Set(0),
        name: Set(payload.name.to_owned()) , //, You, 1 second ago * Uncommitted changes
        email: Set(payload.email.to_owned()),
        password: Set(payload.email.to_owned()),
        uuid: Set(Uuid::new_v4()),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };
    
   let user =  user_model.insert(&db).await.unwrap();

    db.close().await.unwrap();
   (StatusCode::CREATED , "User created"  ) //, user.name.to_string() , user.email.to_string() , user.password.to_string() , user.created_at.to_string() , user.uuid.to_string()


}



pub async fn login_user_post(
    Json(payload): Json<LoginUser>, // payload of type Json of structure CreateUser
) -> impl IntoResponse {

    let database_url = "postgres://postgres:password@localhost:5432/axum_db?schema=public";

    let db: DatabaseConnection = Database::connect( database_url).await.unwrap();

     let user_model = user::Entity::find()
     .filter( Condition::all()
        .add(user::Column::Email.eq(payload.email))
        .add(user::Column::Password.eq(payload.password))
     ).one(&db)
     .await.unwrap().unwrap();

    // let data: UserModel = UserModel{
    //     name: user_model.name,
    //     email: user_model.email,
    //     password: user_model.password,
    //     uuid: user_model.uuid,
    //     created_at: user_model.created_at,
    //     };

    let data: UserModel = UserModel::new(user_model);


   db.close().await.unwrap();

   (StatusCode::CREATED , Json(data))

}


