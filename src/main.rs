use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, MySqlPool};
use std::sync::Arc;
use tracing::{debug, info};

struct ServerContext {
    db: MySqlPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let db = MySqlPool::connect(dotenv::var("DATABASE_URL").unwrap().as_str())
        .await
        .unwrap();

    db.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INT NOT NULL AUTO_INCREMENT,
            username VARCHAR(255) NOT NULL,
            PRIMARY KEY (id)
        )",
    )
    .await
    .unwrap();

    let ctx = Arc::new(ServerContext { db });

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .with_state(ctx);

    let addr = format!(
        "{}:{}",
        dotenv::var("HOST").unwrap_or("127.0.0.1".to_string()),
        dotenv::var("PORT").unwrap_or("3000".to_string())
    )
    .parse()
    .unwrap();

    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

#[derive(Deserialize)]
struct CreateUserPayload {
    username: String,
}

async fn create_user(
    State(ctx): State<Arc<ServerContext>>,
    Json(payload): Json<CreateUserPayload>,
) -> impl IntoResponse {
    let username = payload.username.to_string();

    let result = sqlx::query("INSERT INTO users (username) VALUES (?)")
        .bind(&username)
        .execute(&ctx.as_ref().db)
        .await
        .unwrap();

    let user = User {
        id: result.last_insert_id(),
        username: username.to_owned(),
    };

    debug!("created: {:?}", &user);

    (StatusCode::CREATED, Json(user))
}

#[derive(Serialize, sqlx::FromRow, Debug)]
struct User {
    id: u64,
    username: String,
}
