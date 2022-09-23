use rocket::{
    serde::json::{
        Value, Json, json
    },
    post
};

use crate::models::Master;
use std::fs::File;

use super::key::init_key;

#[post("/login", format="json", data="<master>")]
pub async fn login_master(master: Json<Master>) -> Result<Value, Value>{
    // check if we have register
    if match File::open("keyFile") {
        Ok(_) => false,
        _ => true 
    } {
        return Err(json!("You have not registered!"));
    }

    let master = master.into_inner();

    if let Err(_) = init_key(&master) {
        println!("Error!");
        Err(json!("login failure"))
    } else {
        Ok(json!(master))
    }
}