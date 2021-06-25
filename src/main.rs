use std::io;
use actix_web::{HttpServer, App};
use api::routes::{hello, index};

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(index)
    })
    .bind("https://murmuring-caverns-23223.herokuapp.com")?
    .run()
    .await
}
