use actix_web::{get, post, web::Json, App, HttpResponse, HttpServer, Responder, Result};
use crossbeam::channel::{self};
mod model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (tx_a, rx_a) = channel::unbounded();
    let rx_a2 = rx_a.clone();

    //menu
    let mut food1 = model::Food::new("xburger");
    food1.add_ingredient("burger", 3);
    food1.add_ingredient("cheese", 2);
    food1.add_ingredient("onion", 1);

    let mut food2 = model::Food::new("hotdog");
    food2.add_ingredient("dog", 2);
    food2.add_ingredient("fries", 3);

    let mut food3 = model::Food::new("omelette");
    food3.add_ingredient("omelette", 2);
    food3.add_ingredient("salad", 2);

    let cook1 = model::Cook::new("John Doe", vec![food1.clone(), food2.clone(), food3.clone()]);
    cook1.start(rx_a);

    let cook2 = model::Cook::new("Jane Doe", vec![food1.clone(), food2.clone(), food3.clone()]);
    cook2.start(rx_a2);

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(tx_a.clone()))
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
async fn order(
    payload: Json<model::Order>,
    sender: actix_web::web::Data<crossbeam::channel::Sender<model::Order>>,
) -> HttpResponse {
    let order = model::Order {
        client: payload.client.clone(),
        xburger: payload.xburger,
        hotdog: payload.hotdog,
        omelette: payload.omelette,
    };
    let _ = sender.send(order);
    HttpResponse::Ok().json("received")
}
