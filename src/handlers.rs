use actix_web::{web, HttpResponse};
use uuid::Uuid;
use serde_json::json;
use crate::models::Item;

pub async fn create_item(item: web::Json<Item>) -> HttpResponse {
    HttpResponse::Ok().json(item.into_inner())
}
pub async fn get_items() -> HttpResponse {
    let items = vec![
        Item {
            id: Uuid::new_v4(),
            name: String::from("Item 1"),
            description: String::from("Description 1"),
        },
        Item {
            id: Uuid::new_v4(),
            name: String::from("Item 2"),
            description: String::from("Description 2"),
        },
    ];
    HttpResponse::Ok().json(items)
}
pub async fn get_item(item_id: web::Path<Uuid>) -> HttpResponse {
    let item = Item {
        id: *item_id,
        name: String::from("Item 1"),
        description: String::from("Description 1"),
    };
    HttpResponse::Ok().json(item)
}
pub async fn update_item(item_id: web::Path<Uuid>, item: web::Json<Item>) -> HttpResponse {
    let mut updated_item = item.into_inner();
    updated_item.id = *item_id;
    HttpResponse::Ok().json(updated_item)
}
pub async fn delete_item(item_id: web::Path<Uuid>) -> HttpResponse {
    HttpResponse::Ok().json(json!({ "deleted": item_id.to_string() }))
}
