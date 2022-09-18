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
}

impl KeyFile {
    fn new(salt: MySalt, count: u32) -> Self {
        KeyFile {
            salt,
            count
        }
    }
}

// master password
#[post("/register", format="json", data="<master_password>")]
pub async fn register_password(master_password: Json<String>) -> Value {
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
    match generate_key_file(&salt, count) {
        Err(_) =>  return json!("Error, writing File failure!"),
        Ok(_) => (),
   } 
    // let password_hash = Pbkdf2.hash_password(master_password.as_bytes(), &salt).unwrap().to_string();

    generate_key(master_password.into_inner(), salt, count);
    json!("success!")
}

fn generate_key_file(salt: &SaltString, count: u32) -> Result<(), ioError>{
    std::fs::File::create("keyFile").expect("create failed");
    let my_salt = MySalt::new(salt);
    let key_file = KeyFile::new(my_salt.clone(), count);  
    
    write(
        "keyFile",
        serde_json::to_string_pretty(&key_file).unwrap()
    )?; 
    Ok(())
}

pub fn generate_key(master_password:String, salt: SaltString, count: u32) {
    let password = master_password.as_bytes();
    let mut res = [0_u8; 16]; // should in the heap
    pbkdf2::<Hmac<Sha256>>(password, salt.as_bytes(), count, &mut res);
    
    let res_key = Box::new(res); 

    unsafe {
        KEY = Some(Box::leak(res_key));   
        println!("{:?}", KEY);
    }
} 