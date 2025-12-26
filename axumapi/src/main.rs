use axum::{ middleware, routing::get, Extension, Router };
use axum_prometheus::{PrometheusMetricLayer};
use sea_orm::{  Database, DatabaseConnection};

mod models;
mod routes;
mod handlers;
mod utils;

#[tokio::main]
async fn main() {
    // build our application with a single route

    // connect database ... 
    let db_url = (*utils::constants::DATABASE_URL).clone();
    let db: DatabaseConnection = Database::connect(db_url).await.expect("Failed to connect to db");
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
    
    // routes .. 
    let app = Router::new()
    .route("/", get(|| async {"Hello, World!"}))
    .merge(routes::user_routes::user_routes())
    .merge(routes::home_routes::home_routes())
    .route_layer(middleware::from_fn(utils::guard::guard)) // Apply this middleware here so it only affects the routes above ...
    .merge(routes::auth_routes::auth_routes())
    .layer(Extension(db.clone())) // database
    .route("/metrics", get(||async move {metric_handle.render()}))
    .layer(prometheus_layer); // this layer will collect metrics for all routes

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3005").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    
}