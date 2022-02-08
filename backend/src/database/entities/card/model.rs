use crate::database::schema::cards;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Card {
    pub id: String,
    pub codename: String,
    pub title: String,
    pub description: String,
    pub priority: i32,
    pub column: String,
    pub created_at: NaiveDateTime,
    pub created_by: String,
    pub assigned_to: Option<String>,
}

#[derive(Insertable)]
#[table_name = "cards"]
pub struct NewCard<'a> {
    pub id: String,
    pub codename: &'a String,
    pub title: &'a String,
    pub description: &'a String,
    pub priority: &'a i32,
    pub column: &'a String,
    pub created_by: &'a String,
    pub assigned_to: Option<String>,
}

#[derive(AsChangeset)]
#[table_name = "cards"]
pub struct UpdateCard<'a> {
    pub codename: Option<&'a String>,
    pub title: Option<&'a String>,
    pub description: Option<&'a String>,
    pub priority: Option<&'a i32>,
    pub column: Option<&'a String>,
    pub assigned_to: Option<&'a String>,
}
