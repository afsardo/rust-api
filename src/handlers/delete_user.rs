use super::ApiResponse;
use crate::{config::ServerContext, entities::user::User};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

pub async fn handler(
    State(ctx): State<Arc<ServerContext>>,
    Path(id): Path<u64>,
) -> impl IntoResponse {
    let user = User::find(&ctx.as_ref().db, id).await;

    if user.is_none() {
        return (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<User> {
                success: false,
                error: Some("user not found".to_string()),
                message: None,
                data: None,
            }),
        );
    }

    User::delete(&ctx.as_ref().db, id).await;

    (
        StatusCode::OK,
        Json(ApiResponse::<User> {
            success: true,
            error: None,
            message: Some("user deleted".to_string()),
            data: user,
        }),
    )
}
