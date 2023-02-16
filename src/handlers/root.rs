use super::ApiResponse;
use axum::{http::StatusCode, response::IntoResponse, Json};

pub async fn handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(ApiResponse::<()> {
            success: true,
            error: None,
            message: Some("It works!".to_string()),
            data: None,
        }),
    )
}
