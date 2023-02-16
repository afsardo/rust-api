use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, sqlx::FromRow, Debug)]
pub struct User {
    pub id: Option<u64>,
    pub username: String,
}

impl User {
    pub async fn save(&mut self, db: &sqlx::MySqlPool) {
        if self.id.is_some() {
            sqlx::query("UPDATE users SET username = ? WHERE id = ?")
                .bind(&self.username)
                .bind(&self.id.unwrap())
                .execute(db)
                .await
                .unwrap();

            return;
        }

        let result = sqlx::query("INSERT INTO users (username) VALUES (?)")
            .bind(&self.username)
            .execute(db)
            .await
            .unwrap();

        self.id = Some(result.last_insert_id());
    }

    pub async fn delete(db: &sqlx::MySqlPool, id: u64) {
        sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(db)
            .await
            .unwrap();
    }

    pub async fn find(db: &sqlx::MySqlPool, id: u64) -> Option<User> {
        let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_one(db)
            .await;

        match result {
            Ok(user) => Some(user),
            Err(_) => None,
        }
    }
}
