use crate::models::User;
use crate::models::NewUser;
use crate::schema::users;

use diesel::{QueryResult, SqliteConnection, QueryDsl};
use diesel::{RunQueryDsl,ExpressionMethods};

pub struct PwdRepository;

impl PwdRepository {
    pub fn find_all(c: &SqliteConnection) -> QueryResult<Vec<User>> {
        users::table
            .limit(100)
            .load::<User>(c)
    }
    
    pub fn find(c: &SqliteConnection, id: i32) -> QueryResult<User> {
        users::table
            .find(id)
            .get_result::<User>(c)
    } 

    pub fn find_pwd(c: &SqliteConnection, site_addr: String) -> QueryResult<Vec<User>>{
        users::table
            .filter(users::website.eq(site_addr))
            .load::<User>(c)
    }
    
    pub fn create(c: &SqliteConnection, new_user: NewUser) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(new_user)
            .execute(c)?;
        let last_id = Self::last_id(c)?;
        Self::find(c, last_id) 
    }
    
    fn last_id(c: &SqliteConnection) -> QueryResult<i32> {
        users::table
            .select(users::id)
            .order(users::id.desc())
            .first(c)
    }
} 