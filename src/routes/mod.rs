use actix_web::web;
pub mod users;
pub mod posts;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(users::config)
            .configure(posts::config)
    );
}