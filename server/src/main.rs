use actix_web::{get, post, web, App, HttpServer, Responder, Result};
use std::collections::HashMap;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health)
            .service(order)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}

#[get("/health")]
async fn health() -> Result<impl Responder> {
    Ok("I am alive")
}

#[post("/order")]
async fn order() -> Result<impl Responder> {
    Ok("received")
}