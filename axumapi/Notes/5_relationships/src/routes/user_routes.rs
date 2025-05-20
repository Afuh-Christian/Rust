use axum::{http::Method, routing::{delete, get, post, put}, Router};
use tower_http::cors::{Any, CorsLayer};

use crate::handlers::{post_handlers::create_post_handler, user_handlers::{all_users, delete_user, update_user_post}};

pub fn user_routes() -> Router {

    let cors = CorsLayer::new()
    .allow_methods([Method::PUT ,Method::DELETE , Method::GET ])
    .allow_origin(Any);

    Router::new()
    .route("/user/all", get(all_users))
    .route("/user/update/{uuid}", put(update_user_post))
    .route("/user/delete/{uuid}", delete(delete_user))

    

    // posts routes 
    .route("/user/posts/create", post(create_post_handler))



    .layer(cors)

}