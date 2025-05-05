use axum::{
    routing::get,
    Router,
};
use sea_orm::{Database, DatabaseConnection};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let database_url = "";

    // connect database ... 
    let db: DatabaseConnection = Database::connect(database_url).await.unwrap();



    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}