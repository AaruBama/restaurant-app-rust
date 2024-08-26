use actix_web::{web, App, HttpServer};
use std::sync::{Arc, Mutex};
use restaurant_service::RestaurantService;

mod api;
mod restaurant_service;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let restaurant_service = Arc::new(Mutex::new(RestaurantService::new()));

    HttpServer::new(move || {
        let restaurant_service_clone = restaurant_service.clone();
        App::new()
            .app_data(web::Data::new(restaurant_service_clone))
            .route("/tables/{table_number}/items", web::post().to(api::add_item_handler))
            .route("/tables/{table_number}/items/{item_id}", web::delete().to(api::remove_item_handler))
            .route("/tables/{table_number}/items", web::get().to(api::query_items_handler))
            .route("/tables/{table_number}/items/{item_id}", web::get().to(api::query_specific_items_handler))
            .route("/menu", web::get().to(api::get_menu_handler))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}