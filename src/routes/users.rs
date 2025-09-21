use actix_web::web;
use crate::handlers;

pub fn config(cfg: &mut web::ServiceConfig){
    cfg.route("/users", web::get().to(handlers::users));
}