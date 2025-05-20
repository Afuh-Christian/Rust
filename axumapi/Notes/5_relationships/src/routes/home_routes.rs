use axum::{http::Method, routing::{get}, Router};
use tower_http::cors::{Any, CorsLayer};

use crate::handlers::post_handlers::{all_posts, single_post};


pub fn home_routes() -> Router {

    let cors = CorsLayer::new()
    .allow_methods([Method::GET ])
    .allow_origin(Any);

    Router::new()
    .route("/posts/all", get(all_posts))
    .route("/posts/{uuid}", get(single_post))
    .layer(cors)

}