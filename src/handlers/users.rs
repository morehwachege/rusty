use serde_json::json;
use actix_web::{ HttpResponse, Responder};

pub async fn users() -> impl Responder{
    let data = json!({
        "hunter": "morgan",
        "kylian": "mbappe",
        "morgan": "freeman"
    });
    HttpResponse::Ok().json(data)
}