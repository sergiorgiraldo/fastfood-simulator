use crossbeam::channel::Receiver;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{thread, time::Duration};
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Order {
    pub id: String,
    pub client: String,
    pub xburger: bool,
    pub hotdog: bool,
    pub omelette: bool,
}
pub struct Cook {
    pub name: String,
    pub foods: Vec<Food>,
}

#[derive(Clone)]
pub struct Food {
    pub name: String,
    pub ingredients: HashMap<String, u64>, //name, time to cook
}

impl Cook {
    pub fn new(name: &str, foods: Vec<Food>) -> Cook {
        Cook {
            name: name.to_string(),
            foods: foods,
        }
    }

    pub fn start(self, orders: Receiver<Order>) {
        thread::spawn(move || loop {
            let order = orders.recv().unwrap();
            let mut ingredients = &HashMap::new();
            for f in &self.foods {
                if order.xburger && f.name == "xburger" {
                    ingredients = &f.ingredients;
                    break;
                }
                if order.hotdog && f.name == "hotdog" {
                    ingredients = &f.ingredients;
                    break;
                }
                if order.omelette && f.name == "omelette" {
                    ingredients = &f.ingredients;
                    break;
                }
            }

            Self::post_msg(format!(
                "I am {:?} doing {:?} from {}",
                self.name, order.id, order.client
            ));

            for (k, v) in ingredients.iter() {
                Self::post_msg(format!("{}::{} -> cooking {}", self.name, order.id, k));
                thread::sleep(Duration::from_secs(*v));
            }
            Self::post_msg(format!("{} DONE", order.id));
        });
    }

    fn post_msg(msg: String) {
        println!("{}", msg);
        let _ = ureq::post("http://localhost:8000/publish").send_json(ureq::json!({
            "user_id": 1,
            "topic": "order",
            "message": msg
        }));
    }
}

impl Food {
    pub fn new(name: &str) -> Food {
        Food {
            name: name.to_string(),
            ingredients: HashMap::new(),
        }
    }

    pub fn add_ingredient(&mut self, name: &str, time_to_cook: u64) {
        self.ingredients.insert(name.to_string(), time_to_cook);
    }
}
