use super::model::NewBoard;
use crate::database::schema;
use diesel::*;
use uuid::Uuid;

pub fn create_board<'a>(
    conn: &SqliteConnection,
    name: &'a String,
) -> Result<usize, diesel::result::Error> {
    let id = Uuid::new_v4().to_string();

    let new_board = NewBoard { id, name };

    let result = diesel::insert_into(schema::board::table)
        .values(&new_board)
        .execute(conn)?;

    Ok(result)
}
