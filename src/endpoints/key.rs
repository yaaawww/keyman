use pbkdf2::password_hash::SaltString;
use rocket::{
    get, post, delete, put,
    serde::json::{Value, Json, json}, 
    response::status,
    http::Status,
};

use crate::{
    models::{NewUser, RetUser, Master, UpdatedUser},
    repositories::PwdRepository,
    DbConn,
    tools::*,
    auth::*,
    endpoints::register::{KeyFile, generate_key},
};

use std::fs::File;
use std::io::Read;

#[post("/", format="json", data="<new_key>")]
pub async fn add_key(conn: DbConn, new_key: Json<NewUser>, auth: AuthStruct) -> Result<Value, status::Custom<Value>> {
    // here we crypto our key
    let cipher_vec = crypto_key();
    let mut new_record = new_key.into_inner();
    new_record.iv = cipher_vec[0].clone();
    new_record.password = cipher_vec[1].clone();

    // test result
    println!("{:?}", new_record);

    conn.run(
        |con| {
            PwdRepository::create(con, new_record)
                .map(|user| {
                    let password = decrypt(user.password, user.iv).unwrap();
                    json!(RetUser {
                        id: user.id,
                        website: user.website,
                        username: user.username,
                        password
                    })
                })
                .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
        }).await
} 


#[get("/")]
pub async fn get_all_key(conn: DbConn, auth: AuthStruct) -> Result<Value, status::Custom<Value>>{
    conn.run(|con| {
            PwdRepository::find_all(con)
                .map(|user| {
                    let mut ret_vec = Vec::<RetUser>::new();
                    for u in user {
                        let password = decrypt(u.password, u.iv).unwrap();
                        ret_vec.push(RetUser { id: u.id, website: u.website, username: u.username, password});
                    }
                    json!(ret_vec)
                })
                .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
        }).await
}

#[get("/<site_addr>")]
pub async fn get_key(conn: DbConn, site_addr: String, auth: AuthStruct) -> Result<Value, status::Custom<Value>>{
    conn.run(|con| {
            PwdRepository::find_pwd(con, site_addr)
                .map(|user| {
                    let mut ret_vec = Vec::<RetUser>::new();
                    for u in user {
                        let password = decrypt(u.password, u.iv).unwrap();
                        ret_vec.push(RetUser { id: u.id, website: u.website, username: u.username, password});
                    }
                    json!(ret_vec)
                })
                .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
        }).await
}

#[delete("/<id>")]
pub async fn delete_key(conn: DbConn, id: i32, auth: AuthStruct) -> Result<Value, status::Custom<Value>> {
    conn.run(move |con| {
        PwdRepository::delete(con, id)
        .map(|user| json!(user))
        .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))       
    })
    .await
}

#[put("/", format = "json", data = "<updated_key>")]
pub async fn update_key(conn: DbConn, updated_key: Json<UpdatedUser>, auth: AuthStruct) -> Result<Value, status::Custom<Value>> {
    let cipher_vec = crypto_key();

    let mut updated_key = updated_key.into_inner();
    updated_key.iv = cipher_vec[0].clone();
    updated_key.password = cipher_vec[1].clone();

    conn.run(move |con| {
        PwdRepository::save(con, updated_key)
            .map(|user| json!(user))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

// the help function
pub fn init_key(master: &Master) -> Result<(), status::Custom<Value>> {
    // recover the key_file struct
    let key_file = {
        if let Ok(mut file) = File::open("keyFile") {
            let mut key_file_str = String::new();
            file.read_to_string(&mut key_file_str).unwrap();
            serde_json::from_str::<KeyFile>(&key_file_str).unwrap()
        } else {
        return Err(status::Custom(Status::InternalServerError, json!("You have not registered!")));
        }
    };

    let salt = SaltString::new(&key_file.salt.str).unwrap();
    generate_key(&master.password, &salt, key_file.count);

    if let Ok(check_str) = decrypt(key_file.varify[1].clone(),key_file.varify[0].clone()) {
        println!("The check_str is: {}", check_str);
        println!("The username is: {}", master.username);
        if check_str != master.username {
            return Err(status::Custom(Status::InternalServerError, json!("login Failure!")));
        } else {
            return Ok(())
        }
    } else {
        return Err(status::Custom(Status::InternalServerError, json!("login Failure!")));
    }
} 

fn crypto_key() -> Vec<String> {
    let plain_key = generate_plain_key();

    println!("{}", plain_key);

    if let Ok(kkey) = generate_cipher(plain_key) {
        kkey   
    } else {
        panic!("error")
    }
}