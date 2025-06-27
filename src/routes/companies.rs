use actix_web::{web, get, post, HttpRequest, Responder};
use actix_web::HttpMessage;
use crate::middleware::auth_middleware::Claims;
use crate::handlers::company_handler::{create_company, get_all_companies};
use crate::models::company::CreateCompany;

#[get("/")]
pub async fn list_companies(req: HttpRequest, db: web::Data<sqlx::MySqlPool>) -> impl Responder {
    let req_clone = req.clone();
    let extensions = req_clone.extensions();
    let claims = extensions.get::<Claims>().unwrap();

    get_all_companies(req, db).await
}

#[post("/")]
pub async fn create_company_route(
    req: HttpRequest,
    db: web::Data<sqlx::MySqlPool>,
    body: web::Json<CreateCompany>,
) -> impl Responder {
    let req_clone = req.clone();
    let extensions = req_clone.extensions();
    let claims = extensions.get::<Claims>().unwrap();

    create_company(req, db, body).await
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/companies")
            .service(list_companies)
            .service(create_company_route),
    );
}
