use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use rand::Rng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};
use std::env;
use crate::middleware::auth_middleware::Claims;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginSuccessResponse {
    pub message: String,
    pub token: String,
}

// 🔹 LOGIC HANDLER for registration
pub async fn register_user_handler(
    db: web::Data<MySqlPool>,
    info: web::Json<RegisterRequest>,
) -> impl Responder {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(info.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let result = sqlx::query!(
        "INSERT INTO users (email, password_hash, role) VALUES (?, ?, 'user')",
        info.email,
        hash,
    )
    .execute(db.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().body("User registered"),
        Err(_) => HttpResponse::InternalServerError().body("Registration failed"),
    }
}

// 🔹 ROUTE HANDLER for registration
#[post("/register")]
pub async fn register_user(
    db: web::Data<MySqlPool>,
    info: web::Json<RegisterRequest>,
) -> impl Responder {
    register_user_handler(db, info).await
}

// 🔹 LOGIC HANDLER for login
pub async fn login_user_handler(
    db: web::Data<MySqlPool>,
    info: web::Json<LoginRequest>,
) -> impl Responder {
    let row = sqlx::query!(
        "SELECT id, password_hash, role FROM users WHERE email = ?",
        info.email
    )
    .fetch_one(db.get_ref())
    .await;

    match row {
        Ok(user) => {
            let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();
            let valid = Argon2::default()
                .verify_password(info.password.as_bytes(), &parsed_hash)
                .is_ok();

            if valid {
                let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
                let expiration = Utc::now()
                    .checked_add_signed(Duration::hours(24))
                    .expect("valid timestamp")
                    .timestamp() as usize;

                let claims = Claims {
                    sub: user.id,
                    role: user.role,
                    exp: expiration,
                };

                let token = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(secret.as_bytes()),
                )
                .unwrap();

                HttpResponse::Ok().json(LoginSuccessResponse {
                    message: "Login successful".to_string(),
                    token,
                })
            } else {
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
        Err(_) => HttpResponse::Unauthorized().body("Invalid credentials"),
    }
}

// 🔹 ROUTE HANDLER for login
#[post("/login")]
pub async fn login_user(
    db: web::Data<MySqlPool>,
    info: web::Json<LoginRequest>,
) -> impl Responder {
    login_user_handler(db, info).await
}
