use rocket::{routes, catchers};
use rocket_sync_db_pools::database;
#[macro_use] extern crate diesel;

mod models;
mod repositories;
mod schema;
mod auth;
mod endpoints;
mod tools;

#[cfg(test)]
mod testres;

use endpoints::*;

#[database("sqlite_path")]
pub struct DbConn(diesel::SqliteConnection);

pub static mut KEY: Option<&mut [u8; 16]> = None;

// Error
#[derive(Debug)]
pub enum ApiError {
    MasterKeyMissing,
    ConventFail,
}

#[allow(unused)]
#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket::build()
        .mount( "/api", routes![
            add_key,
            get_key,
            get_all_key,
            update_key,
            delete_key,
            register_password,
            login_master,
            all_options,
        ])
        .register("/", catchers!(not_found))
        .attach(CORS)
        .attach(DbConn::fairing())
        .launch().await?;
    Ok(())
}