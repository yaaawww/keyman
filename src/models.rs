use crate::schema::users;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Deserialize, Serialize, AsChangeset)]
#[serde(crate="rocket::serde")]
pub struct User{
    pub id: i32,
    pub name: String,
    pub password: String,
    #[serde(skip_deserializing)]
    pub create_at: String
} 

// query body
#[derive(Insertable, Deserialize)]
#[serde(crate="rocket::serde")]
#[table_name="users"]
pub struct NewUser {
    pub name: String,
    pub password: String,
} 