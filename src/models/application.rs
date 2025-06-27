use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;


#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Application {
    pub id: i32,
    pub user_id: i32,
    pub job_id: i32,
    pub resume_link: Option<String>,
    pub applied_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct ApplyRequest {
    pub job_id: i32,
    pub resume_link: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ApplicationRecord {
    pub id: i64,
    pub user_id: i64,
    pub job_id: i64,
    pub applied_at: Option<NaiveDateTime>,
    pub resume_link: Option<String>,
    pub user_email: String,
    pub job_title: String,
}

#[derive(Debug, Deserialize)]
pub struct ApplyJob {
    pub full_name: String,
    pub email: String,
    pub cover_letter: Option<String>,
    pub resume_link: Option<String>,
}

// This is different from the full application list
#[derive(Debug, Deserialize, Serialize)]
pub struct JobApplicationView {
    pub id: i32,
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub resume_link: Option<String>,
    pub cover_letter: Option<String>,
    pub applied_at: Option<NaiveDateTime>,
    pub job_id: i32,
}

