use crate::database::schema::user;
use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "user"]
pub struct NewUser<'a> {
    pub id: String,
    pub name: &'a String,
    pub email: &'a String,
    pub password: &'a String,
}
