//! Domains is called by all modules.
#![forbid(unsafe_code)]

use anyhow::Result;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

pub mod posts;

// #[macro_use]
// extern crate tracing;

const DB_ROOT_URL: &str = "sqlite://../database/testing.db";
const DB_SUB_URL: &str = "sqlite://../../database/testing.db";

/// Database pool.
/// In case database type related code to spread around.
/// When database type changes, only need to change this code.
#[derive(Debug)]
pub struct DBPool(pub Pool<Sqlite>);

impl DBPool {
    /// Create a new database pool.
    pub async fn new(db_url: &str) -> Result<Self> {
        // specified the database type.
        // need to be changed to other database type.
        Ok(DBPool(
            SqlitePoolOptions::new()
                .max_connections(5)
                .connect(db_url)
                .await?,
        ))
    }

    /// Create a new database pool for testing.
    pub async fn new_test(is_root: bool) -> Result<Self> {
        if is_root {
            Self::new(DB_ROOT_URL).await
        } else {
            Self::new(DB_SUB_URL).await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dbpool_new() {
        assert!(DBPool::new(DB_SUB_URL).await.is_ok());
    }
}
