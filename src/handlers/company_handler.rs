use crate::middleware::auth_middleware::Claims;
use crate::models::company::{Company, CreateCompany};
use actix_web::{web, HttpRequest, HttpResponse, Responder, HttpMessage};
use sqlx::MySqlPool;
use chrono::Utc;

// ✅ Create Company with user_id from token claims
pub async fn create_company(
    req: HttpRequest,
    db: web::Data<MySqlPool>,
    info: web::Json<CreateCompany>,
) -> impl Responder {
    let extensions = req.extensions();
    let claims = match extensions.get::<Claims>() {
        Some(c) => c,
        None => return HttpResponse::Unauthorized().body("Unauthorized"),
    };

    let user_id = &claims.sub;

    let result = sqlx::query!(
        r#"
        INSERT INTO companies (name, description, user_id, created_at)
        VALUES (?, ?, ?, ?)
        "#,
        info.name,
        info.description,
        user_id,
        Utc::now().naive_utc()
    )
    .execute(db.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().body("Company created"),
        Err(e) => {
            eprintln!("DB Error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to create company")
        }
    }
}

// ✅ Get all companies (you can add pagination later)
pub async fn get_all_companies(req: HttpRequest, db: web::Data<MySqlPool>) -> impl Responder {
    let extensions = req.extensions();
    let claims = match extensions.get::<Claims>() {
        Some(c) => c,
        None => return HttpResponse::Unauthorized().body("Unauthorized"),
    };

    let companies = sqlx::query_as!(
        Company,
        r#"
        SELECT
            id,
            name,
            description,
            user_id,
            created_at
        FROM companies
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(db.get_ref())
    .await;

    match companies {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch companies"),
    }
}
