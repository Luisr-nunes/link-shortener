use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Link {
    pub id: i32,
    pub short_code: String,
    pub long_url: String,
    pub clicks: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateLinkRequest {
    pub long_url: String,
}