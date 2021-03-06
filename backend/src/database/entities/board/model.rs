use serde::{Deserialize, Serialize};

use crate::database::schema::boards;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Board {
    pub id: String,
    pub name: String,
    pub belongs_to: String,
}

#[derive(Insertable)]
#[table_name = "boards"]
pub struct NewBoard<'a> {
    pub id: String,
    pub name: &'a String,
    pub belongs_to: &'a String,
}
