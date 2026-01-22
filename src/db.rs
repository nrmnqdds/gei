use anyhow::{Context, Result};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePool},
    Row,
};
use std::str::FromStr;

/// Database handler for schedule storage
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    /// Create a new database connection
    pub async fn new(database_url: &str) -> Result<Self> {
        // Parse the connection options and enable auto-creation of database
        let options = SqliteConnectOptions::from_str(database_url)?.create_if_missing(true);

        let pool = SqlitePool::connect_with(options)
            .await
            .context("Failed to connect to database")?;

        Ok(Self { pool })
    }

    /// Initialize the database schema
    pub async fn init_schema(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS schedules (
                username TEXT PRIMARY KEY NOT NULL,
                schedule BLOB NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .context("Failed to create schedules table")?;

        // Create index on username for faster lookups (though it's already primary key)
        Ok(())
    }

    /// Store or update a schedule
    pub async fn store_schedule(&self, username: &str, schedule: &[u8]) -> Result<()> {
        let timestamp = chrono::Utc::now().timestamp();

        sqlx::query(
            r#"
            INSERT INTO schedules (username, schedule, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?3)
            ON CONFLICT(username) DO UPDATE SET
                schedule = ?2,
                updated_at = ?3
            "#,
        )
        .bind(username)
        .bind(schedule)
        .bind(timestamp)
        .execute(&self.pool)
        .await
        .context("Failed to store schedule")?;

        Ok(())
    }

    /// Retrieve a schedule by username
    pub async fn get_schedule(&self, username: &str) -> Result<Option<Vec<u8>>> {
        let result = sqlx::query(
            r#"
            SELECT schedule FROM schedules WHERE username = ?1
            "#,
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .context("Failed to retrieve schedule")?;

        Ok(result.map(|row| row.get::<Vec<u8>, _>("schedule")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_operations() {
        let db = Database::new("sqlite::memory:")
            .await
            .expect("Failed to create database");
        db.init_schema().await.expect("Failed to init schema");

        // Test store
        let test_data = b"encrypted test data";
        db.store_schedule("testuser", test_data)
            .await
            .expect("Failed to store");

        // Test retrieve
        let retrieved = db
            .get_schedule("testuser")
            .await
            .expect("Failed to retrieve");
        assert_eq!(retrieved, Some(test_data.to_vec()));

        // Test non-existent user
        let retrieved = db
            .get_schedule("nonexistent")
            .await
            .expect("Failed to retrieve");
        assert_eq!(retrieved, None);

        // Test update
        let updated_data = b"updated encrypted data";
        db.store_schedule("testuser", updated_data)
            .await
            .expect("Failed to update");

        let retrieved = db
            .get_schedule("testuser")
            .await
            .expect("Failed to retrieve");
        assert_eq!(retrieved, Some(updated_data.to_vec()));
    }
}
