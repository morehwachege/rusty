use actix_web::web;
use crate::handlers;

pub fn config(cfg: &mut web::ServiceConfig){
    cfg
        .route("/users", web::get().to(handlers::get_users))
        .route("/users", web::post().to(handlers::users));
}