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
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
