use dotenv::dotenv;
use rust_api::{
    config::{router::setup_router, ServerContext},
    database::setup_database,
};
use std::sync::Arc;
use tracing::info;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let db = setup_database(dotenv::var("DATABASE_URL").unwrap()).await;

    let ctx = Arc::new(ServerContext { db });

    let router = setup_router(ctx);

    let addr = format!(
        "{}:{}",
        dotenv::var("HOST").unwrap_or("127.0.0.1".to_string()),
        dotenv::var("PORT").unwrap_or("3000".to_string())
    )
    .parse()
    .unwrap();

    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
