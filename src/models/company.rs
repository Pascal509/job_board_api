use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub user_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCompany {
    pub name: String,
    pub description: Option<String>,
}
