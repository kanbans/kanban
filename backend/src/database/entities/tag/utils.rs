use super::model::NewTag;
use crate::database::schema;
use crate::database::utils::DbError;
use diesel::*;
use uuid::Uuid;

pub fn create_tag<'a>(
    conn: &SqliteConnection,
    text: &'a String,
    color: &'a String,
) -> Result<usize, DbError> {
    let id = Uuid::new_v4().to_string();
    let new_tag = NewTag { id, text, color };

    let result = diesel::insert_into(schema::tags::table)
        .values(&new_tag)
        .execute(conn)?;

    Ok(result)
}
