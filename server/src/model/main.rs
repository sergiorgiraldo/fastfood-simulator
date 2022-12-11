use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::thread;
use crossbeam::channel::Receiver;
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Order {
    pub client: String,
    pub xburger: bool,
    pub hotdog: bool,
    pub omelette: bool,
}
pub struct Cook {
    pub name: String,
    pub kitchen: Kitchen
}

#[derive(Clone)]
pub struct Food{
    pub name: String,
    pub ingredients: HashMap<String, u8>, //name, time to cook
}

#[derive(Clone)]
    pub struct Kitchen {
    pub foods: Vec<Food>,
}


impl Cook {
    pub fn new(name: &str, kitchen: Kitchen) -> Cook {
        Cook {
            name: name.to_string(),
            kitchen: kitchen
        }
    }

	pub fn start(&self, receiver: &Receiver<Order>) {
        for order in receiver {
            println!("Cooking order: {}", order.client);
        }
    }
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