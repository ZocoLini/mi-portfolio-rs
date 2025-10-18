use getset::Getters;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use typed_builder::TypedBuilder;

#[derive(FromRow, Serialize, Deserialize, TypedBuilder, Getters, Clone, PartialEq)]
#[getset(get = "pub")]
pub struct Session {
    session_id: String,
    last_activity: chrono::DateTime<chrono::Utc>,
}

#[derive(FromRow, Serialize, Deserialize, TypedBuilder, Getters, Clone, PartialEq)]
#[getset(get = "pub")]
pub struct QrScan {
    #[builder(default)]
    id: u64,
    #[builder(default)]
    reference: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    session_id: String,
    #[builder(default)]
    ip: Option<String>,
}

#[derive(FromRow, Serialize, Deserialize, TypedBuilder, Getters, Clone, PartialEq)]
#[getset(get = "pub")]
pub struct ContentView {
    #[builder(default)]
    id: u64,
    content_id: String,
    session_id: String,
}
