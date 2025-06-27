use actix_web::{App, HttpServer, web, HttpRequest, Responder};
use actix_web::HttpMessage;
use dotenvy::dotenv;
use std::env;
use sqlx::mysql::MySqlPoolOptions;

use routes::{jobs, companies, users, applications, admins};

mod handlers;
mod routes;
mod models;
mod middleware;

use middleware::auth_middleware::{AuthMiddleware, Claims};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let db_pool = MySqlPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    println!("Server running at http://{}", addr);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .configure(routes::auth::configure) // Public auth routes
            .service(
                web::scope("/api")
                    .wrap(AuthMiddleware) // 👈 Middleware applied to protected routes
                    .configure(jobs::configure)
                    .configure(companies::configure)
                    .configure(users::configure)
                    .configure(applications::configure)
                    .configure(admins::configure_admin_routes)
                    .route("/dashboard", web::get().to(protected_dashboard))
            )
    })
    .bind(addr)?
    .run()
    .await
}

// Example protected handler
async fn protected_dashboard(req: HttpRequest) -> impl Responder {
    if let Some(claims) = req.extensions().get::<Claims>() {
        format!(
            "Welcome user with ID: {}, role: {}",
            claims.sub, claims.role
        )
    } else {
        "Unauthorized - No token claims found".to_string()
    }
}
