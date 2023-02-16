use sqlx::MySqlPool;
pub mod router;

pub struct ServerContext {
    pub db: MySqlPool,
}
