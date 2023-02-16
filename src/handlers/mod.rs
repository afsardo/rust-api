use serde::Serialize;
pub mod create_user;
pub mod delete_user;
pub mod read_user;
pub mod root;
pub mod update_user;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub error: Option<String>,
    pub message: Option<String>,
    pub data: Option<T>,
}
