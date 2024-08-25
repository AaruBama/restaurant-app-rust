use actix_web::{web, HttpResponse, Responder};
use std::sync::{Arc, Mutex};
use serde_json::json;
use crate::restaurant_service::{RestaurantService};
use crate::models::{AddItemRequest};


pub async fn add_item_handler(
    restaurant_service: web::Data<Arc<Mutex<RestaurantService>>>,
    table_number: web::Path<u32>,
    item_info: web::Json<AddItemRequest>, // Change to use the new struct
) -> impl Responder {
    let mut restaurant = restaurant_service.lock().unwrap();
    match restaurant.add_item(*table_number, item_info.item_name.clone(), item_info.quantity) {
        Ok(order_item) => HttpResponse::Created().json(order_item),
        Err(error_message) => HttpResponse::BadRequest().json(serde_json::json!({"error": error_message})),
    }
    // HttpResponse::Created().json(order_item)
}

pub async fn remove_item_handler(
    restaurant_service: web::Data<Arc<Mutex<RestaurantService>>>,
    table_number: web::Path<u32>,
    item_id: web::Path<String>,
) -> impl Responder {
    let mut restaurant = restaurant_service.lock().unwrap();
    let success = restaurant.remove_item(*table_number, &item_id);
    if success {
        HttpResponse::Ok().json(json!({"success": true, "message": "Item removed successfully."}))
    } else {
        HttpResponse::NotFound().json(json!({"success": false, "message": "Item not found."}))
    }
}

pub async fn query_items_handler(
    restaurant_service: web::Data<Arc<Mutex<RestaurantService>>>,
    table_number: web::Path<u32>,
) -> impl Responder {
    let restaurant = restaurant_service.lock().unwrap();
    let items = restaurant.query_items(*table_number);
    HttpResponse::Ok().json(items)
}

pub async fn get_menu_handler(
    restaurant_service: web::Data<Arc<Mutex<RestaurantService>>>,
) -> impl Responder {
    let restaurant = restaurant_service.lock().unwrap();
    let menu_items = restaurant.get_menu();
    HttpResponse::Ok().json(menu_items)
}