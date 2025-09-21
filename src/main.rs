use actix_web::{App, HttpServer};
mod handlers;
mod routes;

#[actix_web::main]
async fn main() {
    HttpServer::new( || App::new().configure(routes::config))
    .bind("0.0.0.0:8000")
    .unwrap()
    .run()
    .await
    .unwrap()
}