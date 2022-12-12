use actix_web::{get, post, web::Json, App, HttpResponse, HttpServer, Responder, Result};
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

    let cook1 = model::Cook::new("John Doe");
    let cook2 = model::Cook::new("Jane Doe");

    //kitchen
    let foods = vec![food1, food2, food3];
    let cooks = vec![cook1, cook2];
    let _kitchen = model::Kitchen {
        foods: foods,
        cooks: cooks,
    };

    let (tx, rx) = tokio::sync::mpsc::channel(8);

    start_workers(rx);

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(tx.clone()))
            .service(health)
            .service(order)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}

fn start_workers(mut receiver: tokio::sync::mpsc::Receiver<model::Order>) {
    tokio::spawn(async move {
        while let Some(msg) = receiver.recv().await {
            println!("Got order from {:?}", msg.client);
        }
    });
}

#[get("/health")]
async fn health() -> Result<impl Responder> {
    Ok("I am alive")
}

#[post("/order")]
async fn order(
    payload: Json<model::Order>,
    sender: actix_web::web::Data<tokio::sync::mpsc::Sender<model::Order>>,
) -> HttpResponse {
    let order = model::Order {
        client: payload.client.clone(),
        xburger: payload.xburger,
        hotdog: payload.hotdog,
        omelette: payload.omelette,
    };
    let _ = sender.send(order).await;
    HttpResponse::Ok().json("received")
}
