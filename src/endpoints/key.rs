use pbkdf2::password_hash::SaltString;
use rocket::{
    get, post, 
    serde::json::{Value, Json, json}, 
    response::status,
    http::Status,
};

use crate::{
    models::{NewUser, RetUser},
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
    init_key(auth)?;
    let plain_key = generate_plain_key();

    println!("{}", plain_key);

    let cipher_vec= if let Ok(kkey) = generate_cipher(plain_key) {
        kkey   
    } else {
        panic!("error")
    };
    
    // let json = r#"{ "name": "Sean", "hair_color": "Black" }"#;
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
                        website: user.website,
                        username: user.username,
                        password
                    })
                })
                .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
        }).await
} 


#[get("/")]
pub async fn get_all_key(conn: DbConn) -> Result<Value, status::Custom<Value>>{
    conn.run(|con| {
            PwdRepository::find_all(con)
                .map(|user| json!(user))
                .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
        }).await
}

#[get("/<site_addr>")]
pub async fn get_key(conn: DbConn, site_addr: String, auth: AuthStruct) -> Result<Value, status::Custom<Value>>{

    init_key(auth)?;

    conn.run(|con| {
            PwdRepository::find_pwd(con, site_addr)
                .map(|user| {
                    let mut ret_vec = Vec::<RetUser>::new();
                    for u in user {
                        let password = decrypt(u.password, u.iv).unwrap();
                        ret_vec.push(RetUser { website: u.website, username: u.username, password});
                    }
                    json!(ret_vec)
                })
                .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
        }).await
}

fn init_key(auth: AuthStruct) -> Result<(), status::Custom<Value>> {
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
    generate_key(auth.password, salt, key_file.count);
    Ok(())
} 