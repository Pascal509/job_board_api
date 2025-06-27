use actix_web::web;
use crate::handlers::application_handler::{apply_to_job, list_applications, get_my_applications, get_applications_for_job};


pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(apply_to_job);
    cfg.service(list_applications);
    cfg.service(get_my_applications);
    cfg.service(get_applications_for_job);
}
