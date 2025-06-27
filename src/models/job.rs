use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Job {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub tags: Option<String>,
    pub job_type: Option<String>, 
    pub views: Option<i32>,
    pub company_id: Option<i32>,
    pub user_id: Option<i32>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateJob {
    pub title: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub company_id: Option<i32>,
    pub job_type: Option<String>,
    pub tags: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct JobQueryParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub location: Option<String>,
    pub job_type: Option<String>,
    pub tags: Option<String>,
    pub company_id: Option<i32>,
}