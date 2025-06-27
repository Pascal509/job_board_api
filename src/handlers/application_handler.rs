use actix_web::{post, web, get, HttpResponse, HttpMessage, HttpRequest, Responder};
use sqlx::MySqlPool;
use crate::models::application::{Application, ApplyRequest, ApplicationRecord, ApplyJob, JobApplicationView};
use crate::middleware::auth_middleware::Claims;
use chrono::Utc;

#[post("/applications/jobs/{job_id}/apply")]
pub async fn apply_to_job(
    req: HttpRequest,
    db: web::Data<MySqlPool>,
    job_id: web::Path<i32>,
    form: web::Json<ApplyJob>,
) -> impl Responder {
    let extensions = req.extensions();
    let claims = match extensions.get::<Claims>() {
        Some(c) => c,
        None => return HttpResponse::Unauthorized().body("Unauthorized"),
    };

    let user_id = claims.sub;
    let job_id = *job_id;

    // 🔍 Check if job exists
    match sqlx::query!("SELECT id FROM jobs WHERE id = ?", job_id)
        .fetch_optional(db.get_ref())
        .await
    {
        Ok(None) => return HttpResponse::NotFound().body("Job not found"),
        Err(e) => {
            eprintln!("Database error checking job existence: {:?}", e);
            return HttpResponse::InternalServerError().body("Internal server error");
        }
        _ => {}
    }

    // 🔄 Check for duplicate application
    match sqlx::query!(
        "SELECT id FROM applications WHERE user_id = ? AND job_id = ?",
        user_id,
        job_id
    )
    .fetch_optional(db.get_ref())
    .await
    {
        Ok(Some(_)) => return HttpResponse::BadRequest().body("You have already applied to this job"),
        Err(e) => {
            eprintln!("Database error checking duplicate application: {:?}", e);
            return HttpResponse::InternalServerError().body("Internal server error");
        }
        _ => {}
    }

    // ✅ Insert application
    let result = sqlx::query!(
        r#"
        INSERT INTO applications (user_id, job_id, full_name, email, resume_link, cover_letter, applied_at)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
        user_id,
        job_id,
        form.full_name,
        form.email,
        form.resume_link,
        form.cover_letter,
        Utc::now().naive_utc()
    )
    .execute(db.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().body("Application submitted"),
        Err(e) => {
            eprintln!("Error applying to job: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to submit application")
        }
    }
}

#[get("/applications")]
pub async fn list_applications(
    req: HttpRequest,
    db: web::Data<MySqlPool>,
) -> impl Responder {
    let extensions = req.extensions();
    let claims = match extensions.get::<Claims>() {
        Some(c) => c,
        None => return HttpResponse::Unauthorized().body("Missing token"),
    };

    if claims.role != "admin" {
        return HttpResponse::Forbidden().body("Admins only");
    }

    let result = sqlx::query_as!(
        ApplicationRecord,
        r#"
        SELECT 
            a.id, a.user_id, a.job_id, a.applied_at,
            a.resume_link,
            u.email AS user_email,
            j.title AS job_title
        FROM applications a
        JOIN users u ON a.user_id = u.id
        JOIN jobs j ON a.job_id = j.id
        ORDER BY a.applied_at DESC
        "#
    )
    .fetch_all(db.get_ref())
    .await;

    match result {
        Ok(applications) => HttpResponse::Ok().json(applications),
        Err(e) => {
            eprintln!("Error fetching applications: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to load applications")
        }
    }
}

#[get("/applications/me")]
pub async fn get_my_applications(
    req: HttpRequest,
    db: web::Data<MySqlPool>,
) -> impl Responder {
    let extensions = req.extensions();
    let claims = match extensions.get::<Claims>() {
        Some(c) => c,
        None => return HttpResponse::Unauthorized().body("Unauthorized"),
    };

    let user_id = claims.sub;

    let result = sqlx::query_as!(
        ApplicationRecord,
        r#"
        SELECT 
            a.id, a.user_id, a.job_id, a.applied_at,
            a.resume_link,
            u.email AS user_email,
            j.title AS job_title
        FROM applications a
        JOIN users u ON a.user_id = u.id
        JOIN jobs j ON a.job_id = j.id
        WHERE a.user_id = ?
        ORDER BY a.applied_at DESC
        "#,
        user_id
    )
    .fetch_all(db.get_ref())
    .await;

    match result {
        Ok(applications) => HttpResponse::Ok().json(applications),
        Err(e) => {
            eprintln!("Error fetching user applications: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to fetch your applications")
        }
    }
}

#[get("/applications/jobs/{job_id}")]
pub async fn get_applications_for_job(
    req: HttpRequest,
    db: web::Data<MySqlPool>,
    job_id: web::Path<i32>,
) -> impl Responder {
    let extensions = req.extensions();
    let claims = match extensions.get::<Claims>() {
        Some(c) => c,
        None => return HttpResponse::Unauthorized().body("Unauthorized"),
    };

    // Only admin users allowed
    if claims.role != "admin" {
        return HttpResponse::Forbidden().body("Admins only");
    }

    let job_id = *job_id;

    // Fetch all applications for this job
    let result = sqlx::query_as!(
        JobApplicationView,
        r#"
        SELECT 
            a.id,
            a.full_name,
            a.email,
            a.resume_link,
            a.cover_letter,
            a.applied_at,
            a.job_id
        FROM applications a
        WHERE a.job_id = ?
        ORDER BY a.applied_at DESC
        "#,
        job_id
    )
    .fetch_all(db.get_ref())
    .await;

    match result {
        Ok(applications) => HttpResponse::Ok().json(applications),
        Err(e) => {
            eprintln!("Error fetching applications for job: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to load applications for this job")
        }
    }
}
