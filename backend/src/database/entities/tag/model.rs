use crate::database::schema::tag;

#[derive(Queryable)]
pub struct Tag {
    pub id: String,
    pub text: String,
    pub color: String,
}

#[derive(Insertable)]
#[table_name = "tag"]
pub struct NewTag<'a> {
    pub id: String,
    pub text: &'a String,
    pub color: &'a String,
}
