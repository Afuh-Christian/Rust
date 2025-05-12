use axum::{routing::{post, put, get , delete}, Router};

use crate::handlers::{auth_handlers::{create_user_post, login_user_post}, user_handlers::{all_users, delete_user, update_user_post}};


pub fn user_routes() -> Router {

    // let cors = CorsLayer ;

    Router::new()
    .route("/user/all", get(all_users))
    .route("/user/update/{uuid}", put(update_user_post))
    .route("/user/delete/{uuid}", delete(delete_user))

}