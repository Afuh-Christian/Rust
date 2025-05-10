use axum::{
    http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router
};
use entity::user;
use models::user_model::{self, CreateUser, LoginUser, UserModel};
use sea_orm::{ sqlx::types::chrono::Utc, ActiveValue::Set, Condition, Database, DatabaseConnection};
use uuid::Uuid;
// use sea_orm::ActiveValue::Set;
// use uuid::Uuid;
// use chrono::Utc;
use sea_orm::ActiveModelTrait;
use sea_orm::EntityTrait; 
use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;

mod models;


#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .route("/user/create", post(create_user_post))
    .route("/user/login", post(login_user_post));

    let database_url = "postgres://postgres:password@localhost:5432/axum_db?schema=public";

    // connect database ... 
    let db: DatabaseConnection = Database::connect(database_url).await.unwrap();
    // Migrator::up(&connection, None).await?;

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}



async fn create_user_post(
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
async fn login_user_post(
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

// async fn create_user() -> impl IntoResponse {

//     let database_url = "postgres://postgres:password@localhost:5432/axum_db?schema=public";

//     let db: DatabaseConnection = Database::connect( database_url).await.unwrap();

//     let user_model = user::ActiveModel {
//         // id: Set(0),
//         name: Set("test".to_owned()) , //, You, 1 second ago * Uncommitted changes
//         email: Set("test@gmail.com".to_owned()),
//         password: Set("12345678".to_owned()),
//         uuid: Set(Uuid::new_v4()),
//         created_at: Set(Utc::now().naive_utc()),
//         ..Default::default()
//     };
    
//    let user =  user_model.insert(&db).await.unwrap();


//    (StatusCode::CREATED , "User created"  ) //, user.name.to_string() , user.email.to_string() , user.password.to_string() , user.created_at.to_string() , user.uuid.to_string()


// }
