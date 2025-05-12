use axum::{
    extract::Path, http::StatusCode, response::IntoResponse, routing::{delete, get, post, put}, Json, Router
};
use entity::user::{self, ActiveModel, Model};
use models::user_model::{ self, CreateUser, LoginUser, UpdateUser, UserModel};
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
mod routes;
mod handlers;


#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .merge(routes::auth_routes::auth_routes())
    .merge(routes::user_routes::user_routes())
    ;

    let database_url = "postgres://postgres:password@localhost:5432/axum_db?schema=public";

    // connect database ... 
    let db: DatabaseConnection = Database::connect(database_url).await.unwrap();
    // Migrator::up(&connection, None).await?;

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}



