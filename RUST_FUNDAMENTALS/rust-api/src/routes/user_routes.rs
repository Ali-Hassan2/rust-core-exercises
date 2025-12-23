use axum::{routing::{get,post,delete},Router};
use crate::handlers::user_handler::*;

pub fn routes()-> Router{
    Router::new()
    .route("/users",post(create_user))
    .route("/users",get(get_users))
    .route("/users/:id",delete(delete_users))
}