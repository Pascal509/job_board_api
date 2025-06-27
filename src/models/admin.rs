use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Serialize)]
pub struct DashboardStats {
    pub total_users: i64,
    pub total_jobs: i64,
    pub total_companies: i64,
    pub total_applications: i64,
}

// #[derive(Serialize)]
// pub struct RecentJob {
//     pub id: i32,
//     pub title: String,
//     pub created_at: Option<chrono::NaiveDateTime>,
// }

#[derive(Serialize)]
pub struct RecentJob {
    pub id: i32,
    pub title: String,

    #[serde(rename = "created_at")]
    pub created_at_pretty: String,
}

#[derive(Serialize)]
pub struct RecentApplication {
    pub id: i32,
    pub job_title: String,
    pub user_email: String,
    pub applied_at: String,
}

#[derive(Serialize)]
pub struct AdminDashboardResponse {
    pub stats: DashboardStats,
    pub recent_jobs: Vec<RecentJob>,
    pub recent_applications: Vec<RecentApplication>,
}
