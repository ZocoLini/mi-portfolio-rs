use std::{env};

use sqlx::migrate::MigrateDatabase;

use crate::error::Error;

pub fn get_db_url() -> String {
    env::var("DATABASE_URL").expect("ENV var DATABASE_URL not set")
}

pub async fn prepare(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::Sqlite::create_database(&get_db_url()).await?;

    sqlx::migrate!("../migrations")
        .run(pool)
        .await
        .map_err(|e| e.into())
}

pub async fn connect() -> Result<sqlx::SqlitePool, Error> {
    sqlx::SqlitePool::connect(&get_db_url())
        .await
        .map_err(|e| e.into())
}
