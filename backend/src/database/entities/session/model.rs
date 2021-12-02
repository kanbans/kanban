use crate::database::schema::sessions;

#[derive(Queryable)]
pub struct Session {
    pub session_token: String,
    pub belongs_to: String,
}

#[derive(Insertable)]
#[table_name = "sessions"]
pub struct NewSession<'a> {
    pub session_token: &'a String,
    pub belongs_to: &'a String,
}
