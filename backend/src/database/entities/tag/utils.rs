use super::model::NewTag;
use crate::database::schema;
use uuid::Uuid;
use diesel::*;

pub fn create_tag<'a>(
    conn: &SqliteConnection,
    text: &'a String,
    color: &'a String,
) -> Result<usize, diesel::result::Error> {
    let id = Uuid::new_v4().to_string();
    let new_tag = NewTag { id, text, color };

    let result = diesel::insert_into(schema::tag::table)
        .values(&new_tag)
        .execute(conn)?;

    Ok(result)
}
