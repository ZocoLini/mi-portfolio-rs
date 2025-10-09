use std::{env, io, path::PathBuf};

use sqlx::migrate::MigrateDatabase;

use crate::error::Error;

pub fn get_db_path() -> PathBuf {
    let db_dir = env::current_exe().unwrap();
    let db_name = "portfolio.sqlite";
    db_dir.join(db_name)
}

pub async fn prepare(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::Sqlite::create_database(
        get_db_path()
            .to_str()
            .ok_or(io::Error::new(io::ErrorKind::NotFound, "invalid db path"))?,
    )
    .await?;

    sqlx::migrate!("../migrations")
        .run(pool)
        .await
        .map_err(|e| e.into())
}

pub async fn connect() -> Result<sqlx::SqlitePool, Error> {
    let db_path = get_db_path();
    sqlx::SqlitePool::connect(&format!("sqlite://{}", db_path.display()))
        .await
        .map_err(|e| e.into())
}
