use axum::{routing::post, Router};

use crate::handlers::auth_handlers::{create_user_post, login_user_post};


pub fn auth_routes() -> Router {

    // let cors = CorsLayer ;
    

    Router::new()
        .route("/user/create", post(create_user_post))
        .route("/user/login", post(login_user_post))
}