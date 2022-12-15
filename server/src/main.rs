use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use warp::{ws::Message, Filter, Rejection};
use crossbeam::channel::{self};

mod handler;
mod ws;
mod model;

type Result<T> = std::result::Result<T, Rejection>;
type Clients = Arc<RwLock<HashMap<String, Client>>>;

#[derive(Debug, Clone)]
pub struct Client {
    pub user_id: usize,
    pub topics: Vec<String>,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

#[tokio::main]
async fn main() {
    //kitchen
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

    //socket and api
    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

    let health_route = warp::path!("health").and_then(handler::health_handler);

    let register = warp::path("register");
    let register_routes = register
        .and(warp::post())
        .and(warp::body::json())
        .and(with_clients(clients.clone()))
        .and_then(handler::register_handler)
        .or(register
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_clients(clients.clone()))
            .and_then(handler::unregister_handler));
    
    let order_route = warp::path!("order")
    .and(warp::post())
    .and(warp::body::json())
    .and(with_data(tx_a.clone()))
    .and_then(handler::order_handler);

    let publish = warp::path!("publish")
        .and(warp::body::json())
        .and(with_clients(clients.clone()))
        .and_then(handler::publish_handler);

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::path::param())
        .and(with_clients(clients.clone()))
        .and_then(handler::ws_handler);

    let routes = health_route
        .or(register_routes)
        .or(ws_route)
        .or(order_route)
        .or(publish)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

fn with_data(tx: crossbeam::channel::Sender<model::Order>) -> impl Filter<Extract = (crossbeam::channel::Sender<model::Order>,), Error = Infallible> + Clone {
    warp::any().map(move || tx.clone())
}