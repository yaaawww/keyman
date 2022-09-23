use crate::schema::users;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Deserialize, Serialize, AsChangeset)]
#[serde(crate="rocket::serde")]
pub struct User{
    pub id: i32,
    pub website: String,
    pub username: String,
    pub password: String,
    pub iv: String,
    #[serde(skip_deserializing)]
    pub create_at: String
} 

// query body
#[derive(Insertable, Deserialize, Debug)]
#[serde(crate="rocket::serde")]
#[table_name="users"]
pub struct NewUser {
    pub website: String,
    pub username: String,
    pub password: String,
    pub iv: String,
} 

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate="rocket::serde")]
pub struct Master {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RetUser {
    pub id: i32,
    pub website: String,
    pub username: String,
    pub password: String,
}

#[derive(Insertable, Deserialize, Debug)]
#[serde(crate="rocket::serde")]
#[table_name="users"]
pub struct UpdatedUser {
    pub id: i32,
    pub website: String,
    pub username: String,
    pub password: String,
    pub iv: String,
} 