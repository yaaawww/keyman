use rocket::{catch, serde::json::{Value, json}};

#[catch(404)]
pub async fn not_found() -> Value {
    json!("Not found!")
} 