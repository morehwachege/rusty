use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use env_logger::Env;
use diesel::PgConnection;
use dotenv::dotenv;

mod handlers;
mod routes;
mod models;
mod db;

#[actix_web::main]
async fn main() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file or environment variable");
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    HttpServer::new( move || { 
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::config)
    })
    .bind("0.0.0.0:8000")
    .unwrap()
    .run()
    .await
    .unwrap()
}