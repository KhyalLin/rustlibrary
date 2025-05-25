use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc, NaiveDateTime};
use sqlx::types::chrono::NaiveDateTime as SqlxNaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: Option<String>,
    pub r#type: String,
    pub quantity: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBook {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: Option<String>,
    pub r#type: String,
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateBook {
    pub title: Option<String>,
    pub author: Option<String>,
    pub isbn: Option<String>,
    pub description: Option<String>,
    pub r#type: Option<String>,
    pub quantity: Option<i32>,
} 