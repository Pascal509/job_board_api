use actix_web::{web, get, post, delete, put, HttpRequest, Responder};
use crate::middleware::auth_middleware::Claims;
use crate::handlers::job_handler::{create_job, get_all_jobs, delete_job, increment_job_view, get_job_by_id};
use crate::models::job::CreateJob;

/// Route for GET /api/jobs — with query parameters (page, limit, search)
#[get("/")]
async fn list_jobs(
    req: HttpRequest,
    db: web::Data<sqlx::MySqlPool>,
    query: web::Query<crate::models::job::JobQueryParams>,
) -> impl Responder {
    get_all_jobs(req, db, query).await
}

/// Route for POST /api/jobs
#[post("/")]
async fn create_job_route(
    req: HttpRequest,
    db: web::Data<sqlx::MySqlPool>,
    body: web::Json<CreateJob>,
) -> impl Responder {
    create_job(req, db, body).await
}

/// Route for DELETE /api/jobs/{id}
#[delete("/{id}")]
async fn delete_job_route(
    req: HttpRequest,
    db: web::Data<sqlx::MySqlPool>,
    job_id: web::Path<i32>,
) -> impl Responder {
    delete_job(req, db, job_id).await
}

/// Route for GET /api/jobs/{id}
#[get("/{id}")]
async fn get_job_route(
    db: web::Data<sqlx::MySqlPool>,
    job_id: web::Path<i32>,
) -> impl Responder {
    get_job_by_id(db, job_id).await
}

// /// PUT /api/jobs/{id}/view — increment job view count manually (if needed)
// #[put("/{id}/view")]
// async fn increment_job_views_route(
//     db: web::Data<sqlx::MySqlPool>,
//     job_id: web::Path<i32>,
// ) -> impl Responder {
//     increment_job_view(db, job_id).await
// }

/// Register all /jobs routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/jobs")
            .service(list_jobs)
            .service(create_job_route)
            .service(delete_job_route)
            .service(get_job_route)
            .service(increment_job_view)
            // .service(apply_to_job)
    );
}
