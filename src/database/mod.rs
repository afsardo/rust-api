use sqlx::{Executor, MySqlPool};

pub async fn setup_database(database_url: String) -> MySqlPool {
    let db = MySqlPool::connect(&database_url).await.unwrap();

    db.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
            username VARCHAR(255) NOT NULL,
            PRIMARY KEY (id)
        )",
    )
    .await
    .unwrap();

    db
}
