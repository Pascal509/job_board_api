use actix_web::{web, get, delete};
use crate::handlers::user_handler::{
    get_current_user,
    get_all_users,
    delete_user_by_id,
};

#[get("/me")]
async fn get_me(
    req: actix_web::HttpRequest,
    db: web::Data<sqlx::MySqlPool>,
) -> impl actix_web::Responder {
    get_current_user(req, db).await
}

#[get("/")]
async fn list_users(
    req: actix_web::HttpRequest,
    db: web::Data<sqlx::MySqlPool>,
) -> impl actix_web::Responder {
    get_all_users(req, db).await
}

#[delete("/{id}")]
async fn delete_user(
    req: actix_web::HttpRequest,
    db: web::Data<sqlx::MySqlPool>,
    user_id: web::Path<i32>,
) -> impl actix_web::Responder {
    delete_user_by_id(req, db, user_id).await
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(get_me)
            .service(list_users)
            .service(delete_user),
    );
}
