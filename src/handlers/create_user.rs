use super::ApiResponse;
use crate::{config::ServerContext, entities::user::User};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct CreateUserPayload {
    pub username: String,
}

pub async fn handler(
    State(ctx): State<Arc<ServerContext>>,
    Json(payload): Json<CreateUserPayload>,
) -> impl IntoResponse {
    let mut user = User {
        id: None,
        username: payload.username.to_owned(),
    };

    user.save(&ctx.as_ref().db).await;

    (
        StatusCode::CREATED,
        Json(ApiResponse::<User> {
            success: true,
            error: None,
            message: Some("user created".to_string()),
            data: Some(user),
        }),
    )
}
