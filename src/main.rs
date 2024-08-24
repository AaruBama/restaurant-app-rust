// src/main.rs
use actix_web::{web, App, HttpServer};
use std::sync::{Arc, Mutex};
use crate::services::{MyRestaurantService, RestaurantService};
use crate::api::add_item_handler;

mod models;
mod services;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let restaurant_service: Arc<Mutex<dyn RestaurantService>> = Arc::new(Mutex::new(MyRestaurantService::new()));

    HttpServer::new(move || {
        let restaurant_service_clone = restaurant_service.clone();
        App::new()
            .app_data(web::Data::new(restaurant_service_clone))
            .route("/items", web::post().to(add_item_handler))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}