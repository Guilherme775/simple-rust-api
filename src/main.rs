use std::{env, io};
use actix_web::{HttpServer, App};
use api::routes::{hello, index};

#[actix_web::main]
async fn main() -> io::Result<()> {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(index)
    })
    .bind(("0.0.0.0", port))
    .expect("Can not bind to port 8080")
    .run()
    .await
}
