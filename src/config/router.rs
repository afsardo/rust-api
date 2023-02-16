use super::ServerContext;
use crate::handlers::{create_user, delete_user, read_user, root, update_user};
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

pub fn setup_router(ctx: Arc<ServerContext>) -> Router {
    let app: Router = Router::new()
        .route("/", get(root::handler))
        .route("/users", post(create_user::handler))
        .route("/users/:id", get(read_user::handler))
        .route("/users/:id", put(update_user::handler))
        .route("/users/:id", delete(delete_user::handler))
        .with_state(ctx);

    app
}
