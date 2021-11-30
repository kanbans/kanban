use crate::database::schema::column;

#[derive(Queryable)]
pub struct Column {
    pub id: String,
    pub name: String,
    pub belongs_to: String
}

#[derive(Insertable)]
#[table_name = "column"]
pub struct NewColumn<'a> {
    pub id: String,
    pub name: &'a String,
    pub belongs_to: &'a String,
}
