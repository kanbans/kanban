use super::model::{NewUser, User};
use crate::database::schema;
use crate::database::utils::DbError;
use diesel::*;
use uuid::Uuid;

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

pub fn find_user<'a>(conn: &SqliteConnection, user_email: &'a String) -> Result<User, DbError> {
    use crate::database::schema::users::dsl::*;

    let user = users.filter(email.eq(user_email)).first::<User>(conn)?;

    Ok(user)
}
