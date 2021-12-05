use super::model::NewCardTag;
use crate::database::schema;
use crate::database::utils::DbError;
use diesel::*;

pub fn create_card_tag<'a>(
    conn: &SqliteConnection,
    card_id: &'a String,
    tag_id: &'a String,
) -> Result<usize, DbError> {
    let new_card_tag = NewCardTag { card_id, tag_id };

    let result = diesel::insert_into(schema::card_tags::table)
        .values(&new_card_tag)
        .execute(conn)?;

    Ok(result)
}
