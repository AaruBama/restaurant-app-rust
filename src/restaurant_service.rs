use std::collections::HashMap;
use uuid::Uuid;
use crate::models::{MenuItem, OrderItem, Table};

pub struct RestaurantService {
    pub tables: HashMap<u32, Table>,
    pub menu: HashMap<String, MenuItem>,
}

impl RestaurantService {
    pub fn new() -> Self {
        let mut menu = HashMap::new();

        // Prepopulate menu items
        menu.insert(
            "Pasta".to_string(),
            MenuItem {
                id: Uuid::new_v4().to_string(),
                name: "Pasta".to_string(),
                cooking_time: 10,
            },
        );
        menu.insert(
            "Pizza".to_string(),
            MenuItem {
                id: Uuid::new_v4().to_string(),
                name: "Pizza".to_string(),
                cooking_time: 15,
            },
        );
        menu.insert(
            "Salad".to_string(),
            MenuItem {
                id: Uuid::new_v4().to_string(),
                name: "Salad".to_string(),
                cooking_time: 5,
            },
        );

        RestaurantService {
            tables: HashMap::new(),
            menu,
        }
    }

    pub fn add_item(&mut self, table_number: u32, item_name: String, quantity: u32) -> Result<OrderItem, String> {
        let table = self.tables
            .entry(table_number)
            .or_insert_with(|| Table::new(table_number));

        // Check if the menu item exists
        match self.menu.get(&item_name) {
            Some(menu_item) => {
                let item_id = Uuid::new_v4().to_string();
                let order_item = OrderItem {
                    order_item_id: item_id.clone(),
                    menu_item: menu_item.clone(),
                    quantity,
                };

                table.order_items.insert(item_id, order_item.clone());
                Ok(order_item) // Return the created order item
            },
            None => Err(format!("Menu item '{}' not found", item_name)), // Return an error if not found
        }
    }

    pub fn remove_item(&mut self, table_number: u32, item_id: &str) -> bool {
        if let Some(table) = self.tables.get_mut(&table_number) {
            return table.order_items.remove(item_id).is_some();
        }
        false
    }

    pub fn query_items(&self, table_number: u32) -> Vec<OrderItem> {
        if let Some(table) = self.tables.get(&table_number) {
            table.order_items.values().cloned().collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_menu(&self) -> Vec<MenuItem> {
        self.menu.values().cloned().collect()
    }
}