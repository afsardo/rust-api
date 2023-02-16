use super::ApiResponse;
use crate::{config::ServerContext, entities::user::User};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct UpdateUserPayload {
    pub username: String,
}

pub async fn handler(
    State(ctx): State<Arc<ServerContext>>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateUserPayload>,
) -> impl IntoResponse {
    let user_result = User::find(&ctx.as_ref().db, id).await;

    if user_result.is_none() {
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

    let mut user = user_result.unwrap();
    user.username = payload.username.to_owned();
    user.save(&ctx.as_ref().db).await;

    (
        StatusCode::OK,
        Json(ApiResponse::<User> {
            success: true,
            error: None,
            message: None,
            data: Some(user),
        }),
    )
}
