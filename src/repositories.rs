use crate::models::{NewUser, User, UpdatedUser};
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
    
    pub fn save(c: &SqliteConnection, user: UpdatedUser) -> QueryResult<User> {
        diesel::update(users::table.find(user.id))
            .set((
                users::website.eq(user.website.to_owned()),
                users::username.eq(user.username.to_owned()),
                users::password.eq(user.password.to_owned()), 
                users::iv.eq(user.iv.to_owned())
            ))
            .execute(c)?;
        Self::find(c, user.id)
    }
    
    pub fn delete(c: &SqliteConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(users::table.find(id))
        .execute(c)
    }

    fn last_id(c: &SqliteConnection) -> QueryResult<i32> {
        users::table
            .select(users::id)
            .order(users::id.desc())
            .first(c)
    }
} 