use crate::database::schema::tag;

#[derive(Queryable)]
pub struct Tag {
    pub id: i32,
    pub text: String,
    pub color: String,
}

#[derive(Insertable)]
#[table_name = "tag"]
pub struct NewTag<'a> {
    pub text: &'a String,
    pub color: &'a String,
}
