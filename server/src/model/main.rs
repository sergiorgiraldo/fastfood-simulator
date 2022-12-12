use crossbeam::channel::Receiver;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Order {
    pub client: String,
    pub xburger: bool,
    pub hotdog: bool,
    pub omelette: bool,
}
#[derive(Clone)]
pub struct Cook {
    pub name: String,
}

pub struct Food {
    pub name: String,
    pub ingredients: HashMap<String, u8>, //name, time to cook
}

pub struct Kitchen {
    pub foods: Vec<Food>,
}

impl Cook {
    pub fn new(name: &str) -> Cook {
        Cook {
            name: name.to_string(),
        }
    }

    pub fn start(self, orders: Receiver<Order>) {
        thread::spawn(move ||{
            println!("{:?}", orders.recv().unwrap());
        });
        // for o in orders {
        //     println!("This is {:?}.Got order from {:?}", self.name, o.client);
        // }
        // thread::sleep(Duration::from_secs_f32(0.2));
    }

    // pub fn start(self, mut receiver: tokio::sync::mpsc::Receiver<Order>) {
    //     tokio::spawn(async move {
    //         while let Some(msg) = receiver.recv().await {
    //             println!("This is {:?}.Got order from {:?}", self.name, msg.client);
    //         }
    //     });
    // }
}

impl Food {
    pub fn new(name: &str) -> Food {
        Food {
            name: name.to_string(),
            ingredients: HashMap::new(),
        }
    }

    pub fn add_ingredient(&mut self, name: &str, time_to_cook: u8) {
        self.ingredients.insert(name.to_string(), time_to_cook);
    }
}
