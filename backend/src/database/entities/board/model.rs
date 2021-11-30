use crate::database::schema::board;

#[derive(Queryable)]
pub struct Board {
    pub id: String,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "board"]
pub struct NewBoard<'a> {
    pub id: String,
    pub name: &'a String,
}
