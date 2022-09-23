use rocket::{
    serde::json::{
        Value, Json, json
    },
    post 
};

use pbkdf2::{
    password_hash::{
        rand_core::OsRng,
        SaltString
    },
    pbkdf2
};

use sha2::Sha256;
use hmac::Hmac;

use serde_derive::{Deserialize, Serialize};

use std::fs::write;
use std::fs::File;
use std::io::Error as ioError;


use crate::{models::Master, tools::generate_cipher};
use crate::KEY;

// use this to get our salt string
#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct MySalt {
    pub str: String,
    pub length: usize,
}

impl MySalt {
    fn new(salt: &SaltString) -> Self {
        MySalt { 
            str: String::from(salt.as_str()),
            length: salt.len()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyFile {
    pub salt: MySalt,
    pub count: u32,
    pub varify: Vec<String>,
}

impl KeyFile {
    fn new(salt: MySalt, count: u32, varify: Vec<String>) -> Self {
        KeyFile {
            salt,
            count,
            varify
        }
    }
}

// master password
#[post("/register", format="json", data="<master>")]
pub async fn register_password(master: Json<Master>) -> Value {
    // check if we have register
    if match File::open("keyFile") {
        Ok(_) => true,
        _ => false
    } {
        return json!("You have registered!");
    }
    // hash the masterkey
    let salt = SaltString::generate(&mut OsRng);
    let count:u32 = 10000;

    let master = master.into_inner();
    generate_key(&master.password, &salt, count);

    match generate_key_file(master, &salt, count) {
        Err(_) =>  return json!("Error, writing File failure!"),
        Ok(_) => (),
   } 
    // let password_hash = Pbkdf2.hash_password(master_password.as_bytes(), &salt).unwrap().to_string();
    // I must come up a fxxking solution for the storage of basic auth.
    json!("success!")
}

fn generate_key_file(master: Master, salt: &SaltString, count: u32) -> Result<(), ioError>{
    std::fs::File::create("keyFile").expect("create failed");
    let my_salt = MySalt::new(salt);
    let varify = generate_cipher(master.username).unwrap();
    let key_file = KeyFile::new(my_salt.clone(), count, varify);  
    
    write(
        "keyFile",
        serde_json::to_string_pretty(&key_file).unwrap()
    )?; 
    Ok(())
}

pub fn generate_key(master_password:&String, salt: &SaltString, count: u32) {
    let password = master_password.as_bytes();
    let mut res = [0_u8; 16]; // should in the heap
    pbkdf2::<Hmac<Sha256>>(password, salt.as_bytes(), count, &mut res);
    
    let res_key = Box::new(res); 

    // we need free this when logout!
    unsafe {
        KEY = Some(Box::leak(res_key));   
        println!("{:?}", KEY);
    }
} 