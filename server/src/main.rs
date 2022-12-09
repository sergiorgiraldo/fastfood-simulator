use actix_web::{get, post, web, App, HttpServer, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct OrderObj {
    client: String,
    xburger: bool,
    hotdog: bool,
    omelette: bool,
}

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
async fn order(item: web::Json<OrderObj>) -> HttpResponse {
    println!("order: {:?}", &item.client);
    HttpResponse::Ok().json(item.0)
}