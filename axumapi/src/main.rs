use axum::{ routing::get, Extension, Router
};
use sea_orm::{  Database, DatabaseConnection};

mod models;
mod routes;
mod handlers;
mod utils;


#[tokio::main]
async fn main() {
    // build our application with a single route

    // connect database ... 
    let db_url = utils::constants::DATABASE_URL.clone();
    let db: DatabaseConnection = Database::connect(db_url).await.unwrap();
    // Migrator::up(&connection, None).await?;


    let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .merge(routes::auth_routes::auth_routes())
    .merge(routes::user_routes::user_routes())
    .layer(Extension(db));

  
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    

}



