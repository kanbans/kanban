use super::model::NewCard;
use crate::database::schema;
use crate::database::utils::DbError;
use diesel::*;
use uuid::Uuid;

pub fn create_card<'a>(
    conn: &SqliteConnection,
    codename: &'a String,
    title: &'a String,
    description: &'a String,
    priority: &'a i32,
    column: &'a String,
    created_by: &'a String,
    assigned_to: Option<String>,
) -> Result<usize, DbError> {
    let id = Uuid::new_v4().to_string();

    let new_card = NewCard {
        id,
        codename,
        title,
        description,
        priority,
        column,
        created_by,
        assigned_to,
    };

    let result = diesel::insert_into(schema::cards::table)
        .values(&new_card)
        .execute(conn)?;

    Ok(result)
}
