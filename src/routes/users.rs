use actix_web::web;
use crate::handlers;

pub fn config(cfg: &mut web::ServiceConfig){
    cfg
        .route("/users", web::get().to(handlers::get_users))
        .route("/users", web::post().to(handlers::users))
        .route("/users/{id}", web::get().to(handlers::get_user_by_id))
        .route("/users/{id}", web::put().to(handlers::update_user))
        .route("/users/{id}", web::delete().to(handlers::delete_user));
}