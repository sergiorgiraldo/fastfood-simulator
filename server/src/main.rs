use actix_web::{get, post, web, App, HttpServer, HttpResponse, Responder, Result};
use crossbeam::channel;
use std::thread;
mod model;

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
async fn order(item: web::Json<model::Order>) -> HttpResponse {
    println!("order client: {:?}", &item.client);
    println!("order xburger: {:?}", &item.xburger);
    println!("order hotdog: {:?}", &item.hotdog);
    println!("order omelette: {:?}", &item.omelette);
    HttpResponse::Ok().json(item.0)
}