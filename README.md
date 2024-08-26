
# Restaurant Order Management System

## Description

The Restaurant Order Management System is a server-based Rust application designed to facilitate the management of menu items in a restaurant. It allows restaurant staff to add, remove, and query menu items for specific tables. The application efficiently handles multiple simultaneous requests and provides a quick snapshot of the current orders.

## Features

- **Add Menu Items**: Accepts one or more items along with a table number and a static cooking time.
- **Remove Menu Items**: Allows staff to remove specific items from a specified table.
- **Query Menu Items**: Provides the ability to view all items for a specified table or a specific item for a table.
- **Concurrency**: Supports at least 10 simultaneous incoming requests for adding, removing, and querying items.
- **Random Cooking Time**: Assigns a random cooking time between 5-15 minutes for each item upon creation.
- **Table Management**: Limits the number of specific tables to a finite set (at least 100).
- **View Menu**: Allows staff to view menu items.

## Requirements

- The application must store the item, table number, and cooking time upon creation.
- It must allow for the removal of specified items for a specified table number.
- It must provide a snapshot of all items for a specified table number or a specific item for a table.
- The application must handle at least 10 simultaneous requests.
- The cooking time for items can be generated randomly and does not need to be updated in real-time.

## Installation

To run the application locally, follow these steps:

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/restaurant-order-management.git
   cd restaurant-order-management
   ```
2. Install the necessary dependencies:
   ```bash
   cargo build
   ```
   
3. Run the server:
   ```bash
   cargo run
   ```

## API Endpoints

### Add Item
- **Endpoint**: `POST /tables/{table_number}/items`
  - **Request Body**:
      ```json
      {
         "items": [
             {
                 "item_name": "Salad",
                 "quantity": 2
             },
             {
                 "item_name": "Pasta",
                 "quantity": 1
   
             }
         ]
      }
      ```

### Remove Item
- **Endpoint**: `DELETE /tables/{table_number}/items/{order_item_id}`

### Query All Items for a Table
- **Endpoint**: `GET /tables/{table_number}/items`

### Query Specific Item for a Table
- **Endpoint**: `GET /tables/{table_number}/items/{order_item_id}`

### View menu items:
- **Endpoint**: `GET /menu`

## Example Requests

### Adding an Item
```bash
curl --location --request POST 'http://localhost:8080/tables/1/items' \
--header 'Content-Type: application/json' \
--data '{
    "items": [
        {
            "item_name": "Salad",
            "quantity": 2
        },
        {
            "item_name": "Pasta",
            "quantity": 1

        }
    ]
}'
```

## For Testing concurrency

Additional file siege.conf is added to test concurrency of various endpoints.

```siege -c10 -r10 -v -H "Content-Type: application/json" -f concurrency_testing/siege.conf```


## Operation Complexities:

| Operation                         | Time Complexity |
|-----------------------------------|-----------------|
| Adding an item to a table         | O(1) (average)  |
| Querying an item for a table      | O(1) (average)  |
| Querying all items for a table    | O(n)            |
| Adding multiple items to one table | O(k)            |
| Senu available menu items         | O(m)            |

** n,k is number of items on a table and number of items to be added to an order.
** m is the number of menu items available to be ordered.

### Future Enhancements:
1. Authorization
2. Error Management
3. Test Cases
4. Table Status and non-static cooking time for items.
5. More APIs like table initiation, free tables can be added.
6. Decouple the menu initiation logic.