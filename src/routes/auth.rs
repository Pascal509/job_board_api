use actix_web::web;
use crate::handlers::auth_handler::{register_user, login_user};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(register_user)
            .service(login_user)
    );
}
