use super::model::NewColumn;
use crate::database::schema;
use diesel::*;
use uuid::Uuid;

pub fn create_column<'a>(
    conn: &SqliteConnection,
    name: &'a String,
    belongs_to: &'a String,
) -> Result<usize, diesel::result::Error> {
    let id = Uuid::new_v4().to_string();

    let new_column = NewColumn {
        id,
        name,
        belongs_to,
    };

    let result = diesel::insert_into(schema::column::table)
        .values(&new_column)
        .execute(conn)?;

    Ok(result)
}
