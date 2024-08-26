use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub id: String,
    pub name: String,
    pub cooking_time: u32, // in minutes
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub order_item_id: String,
    pub menu_item: MenuItem,
    pub quantity: u32,
    pub created_at: DateTime<Utc>
}

#[derive(Debug)]
pub struct Table {
    pub table_number: u32,
    pub order_items: HashMap<String, OrderItem>,
}

impl Table {
    pub fn new(table_number: u32) -> Self {
        Table {
            table_number,
            order_items: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AddItemRequest {
    pub items: Vec<OrderItemRequest>, // Change to a vector of OrderItemRequest
}

#[derive(Serialize, Deserialize)]
pub struct OrderItemRequest {
    pub item_name: String,
    pub quantity: u32,
}
