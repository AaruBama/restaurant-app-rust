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
    const MAX_TABLES: usize = 100;
    if *table_number > MAX_TABLES as u32 {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": format!("Max tables allowed is {}", MAX_TABLES)
        }));
    }
    let mut restaurant = restaurant_service.lock().unwrap();
    let mut order_items = Vec::new();
    for item in &item_info.items {
        match restaurant.add_item(*table_number, item.item_name.clone(), item.quantity) {
            Ok(order_item) => order_items.push(order_item),
            Err(error_message) => {
                return HttpResponse::BadRequest().json(serde_json::json!({"error": error_message}));
            }
        }
    }
    HttpResponse::Created().json(order_items)
}

pub async fn remove_item_handler(
    restaurant_service: web::Data<Arc<Mutex<RestaurantService>>>,
    params: web::Path<(u32, String)>
) -> impl Responder {
    let (table_number, item_id) = params.into_inner();
    let mut restaurant = restaurant_service.lock().unwrap();
    let success = restaurant.remove_item(table_number, &item_id);
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

pub async fn query_specific_items_handler(
    restaurant_service: web::Data<Arc<Mutex<RestaurantService>>>,
    params: web::Path<(u32, String)>
) -> impl Responder {
    let restaurant = restaurant_service.lock().unwrap();
    let (table_number, item_id) = params.into_inner();
    if let Some(table) = restaurant.tables.get(&table_number) {
        // Check if the item exists in the table's order_items
        if let Some(order_item) = table.order_items.get(item_id.as_str()) {
            // Item found, return it
            return HttpResponse::Ok().json(order_item);
        }
    }

    // Item not found, return an error response
    HttpResponse::NotFound().json(serde_json::json!({"error": "Item not found"}))
}

pub async fn get_menu_handler(
    restaurant_service: web::Data<Arc<Mutex<RestaurantService>>>,
) -> impl Responder {
    let restaurant = restaurant_service.lock().unwrap();
    let menu_items = restaurant.get_menu();
    HttpResponse::Ok().json(menu_items)
}