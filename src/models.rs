use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Items {
    pub id: String,
    pub name: String,
    pub cooking_time: u32, // in minutes
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub menu_item: Items,
    pub quantity: u32,
}