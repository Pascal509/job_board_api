// src/handlers/user_handler.rs
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use sqlx::MySqlPool;
use crate::middleware::auth_middleware::Claims;
use crate::models::user::User;
use actix_web::HttpMessage;

/// Get current logged-in user (GET /users/me)
pub async fn get_current_user(req: HttpRequest, db: web::Data<MySqlPool>) -> impl Responder {
    if let Some(claims) = req.extensions().get::<Claims>() {
        let user_id = claims.sub;

        let result = sqlx::query_as!(
            User,
            r#"SELECT id, email, password_hash, role, created_at, updated_at
               FROM users WHERE id = ?"#,
            user_id
        )
        .fetch_one(db.get_ref())
        .await;

        match result {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(_) => HttpResponse::NotFound().body("User not found"),
        }
    } else {
        HttpResponse::Unauthorized().body("Unauthorized")
    }
}

/// Get all users (admin only)
pub async fn get_all_users(req: HttpRequest, db: web::Data<MySqlPool>) -> impl Responder {
    if let Some(claims) = req.extensions().get::<Claims>() {
        if claims.role != "admin" {
            return HttpResponse::Forbidden().body("Access denied");
        }

        let result = sqlx::query_as!(
            User,
            r#"SELECT id, email, password_hash, role, created_at, updated_at FROM users"#
        )
        .fetch_all(db.get_ref())
        .await;

        match result {
            Ok(users) => HttpResponse::Ok().json(users),
            Err(_) => HttpResponse::InternalServerError().body("Failed to fetch users"),
        }
    } else {
        HttpResponse::Unauthorized().body("Unauthorized")
    }
}

/// Delete a user by ID (admin only)
pub async fn delete_user_by_id(
    req: HttpRequest,
    db: web::Data<MySqlPool>,
    user_id: web::Path<i32>,
) -> impl Responder {
    if let Some(claims) = req.extensions().get::<Claims>() {
        if claims.role != "admin" {
            return HttpResponse::Forbidden().body("Access denied");
        }

        let result = sqlx::query!("DELETE FROM users WHERE id = ?", *user_id)
            .execute(db.get_ref())
            .await;

        match result {
            Ok(_) => HttpResponse::Ok().body("User deleted successfully"),
            Err(_) => HttpResponse::InternalServerError().body("Failed to delete user"),
        }
    } else {
        HttpResponse::Unauthorized().body("Unauthorized")
    }
}
