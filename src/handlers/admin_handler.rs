use actix_web::{get, web, HttpResponse, HttpRequest};
use sqlx::MySqlPool;
use serde::Serialize;
use crate::middleware::auth_middleware::Claims;
use crate::models::admin::{DashboardStats, RecentJob, RecentApplication, AdminDashboardResponse};
use actix_web::HttpMessage;


#[get("/admin/dashboard")]
pub async fn get_dashboard_stats(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let extensions = req.extensions();
    let claims = match extensions.get::<Claims>() {
        Some(c) => c,
        None => return HttpResponse::Unauthorized().body("Missing auth claims"),
    };

    if claims.role != "admin" {
        return HttpResponse::Forbidden().body("Access denied");
    }

    let total_users = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or(0);

    let total_jobs = sqlx::query_scalar!("SELECT COUNT(*) FROM jobs")
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or(0);

    let total_companies = sqlx::query_scalar!("SELECT COUNT(*) FROM companies")
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or(0);

    let total_applications = sqlx::query_scalar!("SELECT COUNT(*) FROM applications")
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or(0);

    // let recent_jobs = sqlx::query_as!(
    //     RecentJob,
    //     r#"
    //     SELECT id, title, created_at as "created_at: chrono::NaiveDateTime"
    //     FROM jobs
    //     ORDER BY created_at DESC
    //     LIMIT 5
    //     "#
    // )
    // .fetch_all(pool.get_ref())
    // .await
    // .unwrap_or(vec![]);

    let raw_jobs = sqlx::query!(
        r#"
        SELECT id, title, created_at
        FROM jobs
        ORDER BY created_at DESC
        LIMIT 5
        "#
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap_or_default();

    let recent_jobs: Vec<RecentJob> = raw_jobs.into_iter().map(|job| {
        let created_at_pretty = job
            .created_at
            .map(|dt| dt.format("%d %B %Y").to_string())
            .unwrap_or("N/A".to_string());

        RecentJob {
            id: job.id,
            title: job.title,
            created_at_pretty,
        }
    }).collect();

    let raw_applications = sqlx::query!(
        r#"
        SELECT a.id, j.title as job_title, u.email as user_email, a.applied_at 
        FROM applications a
        JOIN jobs j ON a.job_id = j.id
        JOIN users u ON a.user_id = u.id
        ORDER BY a.applied_at DESC
        LIMIT 5
        "#
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap_or_default();

    let recent_applications: Vec<RecentApplication> = raw_applications.into_iter().map(|app| {
        let applied_at = app
            .applied_at
            .map(|dt| dt.format("%d %B %Y").to_string())
            .unwrap_or("N/A".to_string());

        RecentApplication {
            id: app.id,
            job_title: app.job_title,
            user_email: app.user_email,
            applied_at,
        }
    }).collect();

    let stats = DashboardStats {
        total_users,
        total_jobs,
        total_companies,
        total_applications,
    };

    let response = AdminDashboardResponse {
        stats,
        recent_jobs,
        recent_applications,
    };

    HttpResponse::Ok().json(response)
}
