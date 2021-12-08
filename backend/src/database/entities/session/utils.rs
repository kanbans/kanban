use super::model::{NewSession, Session};
use crate::database::entities::user::model::User;
use crate::database::schema;
use diesel::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn create_session<'a>(
    conn: &SqliteConnection,
    session_token: &'a String,
    belongs_to: &'a String,
) -> Result<usize, DbError> {
    let new_session = NewSession {
        session_token,
        belongs_to,
    };

    let result = diesel::insert_into(schema::sessions::table)
        .values(&new_session)
        .execute(conn)?;

    Ok(result)
}

pub fn find_user_with_session<'a>(
    conn: &SqliteConnection,
    token: &'a String,
) -> Result<User, DbError> {
    use crate::database::schema::sessions::dsl::*;
    use crate::database::schema::users::dsl::*;

    //let session = sessions.filter(email.eq(user_email)).first::<User>(conn)?;
    let session = sessions
        .filter(session_token.eq(token))
        .first::<Session>(conn)?;

    // TODO: log error when this doesn't succeed
    // possibly need to clean up invalid data
    let user = users
        .filter(id.eq(session.belongs_to))
        .first::<User>(conn)?;

    Ok(user)
}
