use super::model::NewUser;
use crate::database::schema;
use diesel::*;
use uuid::Uuid;

pub fn create_user<'a>(
    conn: &SqliteConnection,
    name: &'a String,
    email: &'a String,
    password: &'a String,
) -> Result<usize, diesel::result::Error> {
    let id = Uuid::new_v4().to_string();

    let new_user = NewUser {
        id,
        name,
        email,
        password,
    };

    let result = diesel::insert_into(schema::users::table)
        .values(&new_user)
        .execute(conn)?;

    Ok(result)
}
