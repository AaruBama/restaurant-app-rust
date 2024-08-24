use actix_web::{web, HttpResponse, Responder};
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use crate::services::RestaurantService;

#[derive(Serialize, Deserialize)]
pub struct AddItemRequest {
    pub item_name: String,
    pub quantity: u32,
}

pub async fn add_item_handler(
    restaurant_service: web::Data<Arc<Mutex<dyn RestaurantService>>>,
    item_info: web::Json<AddItemRequest>, // Change to use the new struct
) -> impl Responder {
    let mut restaurant = restaurant_service.lock().unwrap();
    let order_item = restaurant.add_item(item_info.item_name.clone(), item_info.quantity);
    HttpResponse::Created().json(order_item)
}