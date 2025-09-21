use actix_web::web;
use crate::handlers;

pub fn config(cfg: &mut web::ServiceConfig){
    cfg.route("/posts", web::get().to(handlers::get_posts));
    cfg.route("/posts", web::post().to(handlers::make_post));
}