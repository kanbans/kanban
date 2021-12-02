use crate::database::schema::tags;

#[derive(Queryable)]
pub struct Tag {
    pub id: String,
    pub text: String,
    pub color: String,
}

#[derive(Insertable)]
#[table_name = "tags"]
pub struct NewTag<'a> {
    pub id: String,
    pub text: &'a String,
    pub color: &'a String,
}
