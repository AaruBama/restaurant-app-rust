use std::collections::HashMap;
use crate::models::{Order, Items};
use uuid::Uuid;
use rand::Rng;

pub trait RestaurantService: Send + Sync {
    fn add_item(&mut self, item_name: String, quantity: u32) -> Order;
}

pub struct MyRestaurantService {
    tables: HashMap<String, Order>, // order ID -> OrderItem
}

impl MyRestaurantService {
    pub fn new() -> Self {
        MyRestaurantService {
            tables: HashMap::new(),
        }
    }
}

impl RestaurantService for MyRestaurantService {
    fn add_item(&mut self, item_name: String, quantity: u32) -> Order {
        let cooking_time = rand::thread_rng().gen_range(5..=15);
        let item_id = Uuid::new_v4().to_string();
        let item = Items {
            id: item_id.clone(),
            name: item_name,
            cooking_time,
        };
        let order_item = Order { menu_item: item.clone(), quantity };
        self.tables.insert(item_id.clone(), order_item.clone());
        order_item
    }
}