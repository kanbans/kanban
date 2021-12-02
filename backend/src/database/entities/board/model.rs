use crate::database::schema::boards;

#[derive(Queryable)]
pub struct Board {
    pub id: String,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "boards"]
pub struct NewBoard<'a> {
    pub id: String,
    pub name: &'a String,
}
