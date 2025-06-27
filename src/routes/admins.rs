use actix_web::web;
use crate::handlers::admin_handler::get_dashboard_stats;

pub fn configure_admin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_dashboard_stats);
}
