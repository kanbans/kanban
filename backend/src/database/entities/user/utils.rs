use super::model::NewUser;
use crate::database::schema;
use diesel::*;
use uuid::Uuid;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn create_user<'a>(
    conn: &SqliteConnection,
    name: &'a String,
    email: &'a String,
    password: &'a String,
) -> Result<NewUser<'a>, DbError> {
    let id = Uuid::new_v4().to_string();

    let new_user = NewUser {
        id,
        name,
        email,
        password,
    };

    diesel::insert_into(schema::users::table)
        .values(&new_user)
        .execute(conn)?;

    Ok(new_user)
}
