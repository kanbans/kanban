use crate::database::schema::columns;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Column {
    pub id: String,
    pub name: String,
    pub belongs_to: String,
}

#[derive(Insertable)]
#[table_name = "columns"]
pub struct NewColumn<'a> {
    pub id: String,
    pub name: &'a String,
    pub belongs_to: &'a String,
}

#[derive(AsChangeset)]
#[table_name = "columns"]
pub struct UpdateColumn<'a> {
    pub name: Option<&'a String>,
    pub belongs_to: Option<&'a String>,
}
