use crate::middleware::auth_middleware::Claims;
use crate::models::job::{CreateJob, Job, JobQueryParams};
use actix_web::{web, put, post, HttpRequest, HttpResponse, Responder, Error};
use sqlx::MySqlPool;
use actix_web::HttpMessage;
use chrono::Utc;
use actix_web::web::{Path, Json};
use serde_json::json;


/// Create a new job — only allowed for authenticated users
pub async fn create_job(
    req: HttpRequest,
    db: web::Data<MySqlPool>,
    info: web::Json<CreateJob>,
) -> Result<HttpResponse, Error> {
    let extensions = req.extensions();
    let claims = match extensions.get::<Claims>() {
        Some(c) => c,
        None => {
            return Ok(HttpResponse::Unauthorized().finish());
        }
    };
    let user_id = &claims.sub;
    let job_type = info.job_type.clone().unwrap_or("full-time".into());
    let tags = info.tags.clone().unwrap_or("".into());

    let result = sqlx::query!(
        r#"
        INSERT INTO jobs (title, description, location, company_id, user_id, job_type, tags, created_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        info.title,
        info.description,
        info.location,
        info.company_id,
        user_id,
        job_type,
        tags,
        Utc::now().naive_utc()  // Convert to NaiveDateTime here
    )
    .execute(db.as_ref())
    .await;

    match result {
        Ok(res) => Ok(HttpResponse::Created().json(format!("Job created with ID: {}", res.last_insert_id()))),
        Err(e) => {
            eprintln!("Error inserting job: {}", e);
            Ok(HttpResponse::InternalServerError().body(format!("Failed to create job: {}", e)))
        }
    }
}

/// Retrieve all jobs — supports filtering & pagination
pub async fn get_all_jobs(
    req: HttpRequest,
    db: web::Data<MySqlPool>,
    query: web::Query<JobQueryParams>,
) -> Result<HttpResponse, Error> {
    let extensions = req.extensions();
    let claims = match extensions.get::<Claims>() {
        Some(c) => c,
        None => return Ok(HttpResponse::Unauthorized().body("Unauthorized: No valid token")),
    };

    let user_id = &claims.sub;
    let role = &claims.role;

    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let search = query.search.clone().unwrap_or_default();
    let location = query.location.clone().unwrap_or_default();
    let job_type = query.job_type.clone().unwrap_or_default();
    let tags = query.tags.clone().unwrap_or_default();
    let company_id = query.company_id;

    let mut sql = String::from(
        "SELECT id, title, description, location, company_id, user_id, job_type, tags, views, created_at FROM jobs WHERE 1=1",
    );
    let mut args: Vec<String> = vec![];

    // ⛔ Restrict regular users to their own jobs
    if role != "admin" {
        sql += " AND user_id = ?";
        args.push(user_id.to_string());
    }

    if !search.is_empty() {
        sql += " AND (title LIKE ? OR description LIKE ?)";
        let like = format!("%{}%", search);
        args.push(like.clone());
        args.push(like);
    }

    if !location.is_empty() {
        sql += " AND location LIKE ?";
        args.push(format!("%{}%", location));
    }

    if !job_type.is_empty() {
        sql += " AND job_type LIKE ?";
        args.push(format!("%{}%", job_type));
    }

    if let Some(cid) = company_id {
        sql += " AND company_id = ?";
        args.push(cid.to_string());
    }

    sql += " ORDER BY created_at DESC LIMIT ? OFFSET ?";

    let mut query_builder = sqlx::query_as::<_, Job>(&sql);
    for val in args {
        query_builder = query_builder.bind(val);
    }
    query_builder = query_builder.bind(limit).bind(offset);

    match query_builder.fetch_all(db.as_ref()).await {
        Ok(jobs) => Ok(HttpResponse::Ok().json(jobs)),
        Err(e) => {
            eprintln!("Error fetching jobs: {}", e);
            Ok(HttpResponse::InternalServerError().body("Failed to fetch jobs"))
        }
    }
}

/// Delete a job — only allowed for admin users
pub async fn delete_job(
    req: HttpRequest,
    db: web::Data<MySqlPool>,
    job_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    // Fix: Store extensions in a variable so the reference lives long enough
    let extensions = req.extensions();
    let claims = match extensions.get::<Claims>() {
        Some(c) => c,
        None => return Ok(HttpResponse::Unauthorized().body("Unauthorized: No token")),
    };

    // Check if the role is admin
    if claims.role != "admin" {
        return Ok(HttpResponse::Forbidden().body("Forbidden: Admins only"));
    }

    let result = sqlx::query!(
        "DELETE FROM jobs WHERE id = ?",
        *job_id
    )
    .execute(db.as_ref())
    .await;

    match result {
        Ok(_) => Ok(HttpResponse::Ok().body("Job deleted successfully")),
        Err(e) => {
            eprintln!("Error deleting job: {}", e);
            Ok(HttpResponse::InternalServerError().body("Failed to delete job"))
        }
    }
}

#[put("/view/{id}")]
pub async fn increment_job_view(
    db: web::Data<MySqlPool>,
    job_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let job_id = *job_id;

    let res = sqlx::query!(
        "UPDATE jobs SET views = views + 1 WHERE id = ?",
        job_id
    )
    .execute(db.as_ref())
    .await;

    match res {
        Ok(_) => Ok(HttpResponse::Ok().body("View counted")),
        Err(e) => {
            eprintln!("View update error: {}", e);
            Ok(HttpResponse::InternalServerError().body("Failed to update view"))
        }
    }
}

/// Get single job by ID and increment views
pub async fn get_job_by_id(
    db: web::Data<MySqlPool>,
    job_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let id = *job_id;

    // Increment views
    let _ = sqlx::query!("UPDATE jobs SET views = views + 1 WHERE id = ?", id)
        .execute(db.as_ref())
        .await;

    // Fetch job
    let job = sqlx::query_as::<_,Job>(
        r#"
        SELECT id, title, description, location, company_id, user_id, created_at,
               job_type, tags, views
        FROM jobs WHERE id = ?
        "#
    )
    .bind(id)
    .fetch_optional(db.as_ref())
    .await;

    match job {
        Ok(Some(job)) => Ok(HttpResponse::Ok().json(job)),
        Ok(None) => Ok(HttpResponse::NotFound().body("Job not found")),
        Err(e) => {
            eprintln!("Error fetching job: {}", e);
            Ok(HttpResponse::InternalServerError().body("Failed to fetch job"))
        }
    }
}

// #[post("/jobs/{id}/apply")]
// pub async fn apply_to_job(
//     req: HttpRequest,
//     db: web::Data<MySqlPool>,
//     job_id: web::Path<i32>,
// ) -> Result<HttpResponse, Error> {
//     let job_id = *job_id;

//     // Get the authenticated user from request extensions
//     let extensions = req.extensions();
//     let claims = match extensions.get::<Claims>() {
//         Some(c) => c,
//         None => return Ok(HttpResponse::Unauthorized().body("Unauthorized: Missing claims")),
//     };

//     // NOTE: If claims.sub is already an i32, use directly
//     let user_id = claims.sub;

//     // 1. Check if the job exists
//     let job_exists: i64 = match sqlx::query_scalar!(
//         "SELECT EXISTS(SELECT 1 FROM jobs WHERE id = ?)",
//         job_id
//     )
//     .fetch_one(db.as_ref())
//     .await
//     {
//         Ok(val) => val,
//         Err(_) => return Ok(HttpResponse::InternalServerError().body("Failed to check job")),
//     };

//     if job_exists == 0 {
//         return Ok(HttpResponse::NotFound().body("Job not found"));
//     }

//     // 2. Check if user already applied
//     let already_applied: i64 = match sqlx::query_scalar!(
//         "SELECT EXISTS(SELECT 1 FROM applications WHERE job_id = ? AND user_id = ?)",
//         job_id,
//         user_id
//     )
//     .fetch_one(db.as_ref())
//     .await
//     {
//         Ok(val) => val,
//         Err(_) => return Ok(HttpResponse::InternalServerError().body("Failed to check application")),
//     };

//     if already_applied == 1 {
//         return Ok(HttpResponse::Conflict().body("You already applied to this job"));
//     }

//     // 3. Insert the application
//     let res = sqlx::query!(
//         "INSERT INTO applications (user_id, job_id) VALUES (?, ?)",
//         user_id,
//         job_id
//     )
//     .execute(db.as_ref())
//     .await;

//     match res {
//         Ok(_) => Ok(HttpResponse::Ok().json(json!({ "message": "Application submitted" }))),
//         Err(e) => {
//             eprintln!("Error inserting application: {}", e);
//             Ok(HttpResponse::InternalServerError().body("Failed to apply to job"))
//         }
//     }
// }