use actix_web::{get, post, HttpResponse, Responder, web};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Info {
    name: String,
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("/")]
pub async fn index(info: web::Json<Info>) -> HttpResponse {
    HttpResponse::Ok().json(Info {
        name: info.name.to_string(),
    })
}

