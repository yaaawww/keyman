#![allow(unused)]
#[macro_use] extern crate diesel;
use diesel::{RunQueryDsl, query_dsl::methods::{FindDsl, LimitDsl}};
use rocket::{
    catch, 
    catchers, 
    delete, 
    get, 
    post, 
    routes, 
    serde::json::{Value, serde_json::json},
};

use rocket::tokio::time::{sleep, Duration};
use rocket_sync_db_pools::database;

mod models;
mod schema;

use models::{NewUser, User};
use schema::users;

#[database("sqlite_path")]
struct DbConn(diesel::SqliteConnection);

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket::build()
        .mount("/", routes![index])
        .mount( "/api", routes![get_user, post_user, delay])
        .register("/", catchers!(not_found))
        .attach(DbConn::fairing())
        .launch().await?;

    Ok(())
}

// we use the route name as the function name in the website
// when we mount, we use the function name.

// routes
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/deley/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/")]
async fn get_user(conn: DbConn) -> Value {
    conn.run(
        |con| {
            let user = users::table.limit(100)
                .load::<User>(con)
                .expect("Error users list");
            json!(user)
        }).await
}

#[post("/")]
async fn post_user() -> Value {
    json!("post!")
} 

#[catch(404)]
async fn not_found() -> Value {
    json!("Not found!")
} 
