use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Order {
    pub client: String,
    pub xburger: bool,
    pub hotdog: bool,
    pub omelette: bool,
}
pub struct Cook {
    pub name: String,
}

pub struct Food {
    pub name: String,
    pub ingredients: HashMap<String, u8>, //name, time to cook
}

pub struct Kitchen {
    pub foods: Vec<Food>,
    pub cooks: Vec<Cook>,
}

impl Cook {
    pub fn new(name: &str) -> Cook {
        Cook {
            name: name.to_string(),
        }
    }

    pub fn start(&self, order: &Order) {
        println!("Cooking order: {}", order.client);
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
