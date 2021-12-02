use super::model::NewSession;
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
