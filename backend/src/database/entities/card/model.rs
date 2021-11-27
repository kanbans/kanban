use crate::database::schema::card;
use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Card {
    pub id: String,
    pub codename: String,
    pub title: String,
    pub description: String,
    pub priority: i32,
    pub column: String,
    pub created_at: NaiveDateTime,
    pub created_by: String,
    pub assigned_to: String,
}

#[derive(Insertable)]
#[table_name = "card"]
pub struct NewCard<'a> {
    pub id: String,
    pub codename: &'a String,
    pub title: &'a String,
    pub description: &'a String,
    pub priority: &'a i32,
    pub column: &'a String,
    pub created_by: &'a String,
    pub assigned_to: &'a String,
}
