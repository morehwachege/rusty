use serde_json::json;
use serde::{ Serialize, Deserialize};
use actix_web::{web, HttpResponse, Responder};

#[derive(Debug, Deserialize, Serialize)]
pub struct Post{
    title: String,
    content: String,
}

pub async fn get_posts() -> impl Responder{
    let data = json!({
        "id": "12",
        "caption": "This is my first movie",
        "likes": 12,
        "replies":231,
    });
    HttpResponse::Ok().json(data)
}

pub async fn make_post(post: web::Json<Post>) -> impl Responder{
    println!("{:?}",post);
    HttpResponse::Created().json(post)
}