use actix_web::{get, post, web, App, HttpServer, HttpResponse, Responder, Result};
use crossbeam::channel;
mod model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

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

    let foods = vec![food1, food2, food3];
    let kitchen = model::Kitchen{foods: foods};

    //kitchen
    let mut cook1 = model::Cook::new("John Doe", kitchen.clone());
    let mut cook2 = model::Cook::new("Jane Doe", kitchen.clone());
    let (sender, receiver) = channel::unbounded();
    cook1.start(&receiver);
    cook2.start(&receiver);

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