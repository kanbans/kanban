use super::model::{Card, NewCard, UpdateCard};
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

pub fn delete_card(conn: &SqliteConnection, cid: &String) -> Result<usize, DbError> {
    use crate::database::schema::cards::dsl::*;
    let res = diesel::delete(cards.filter(id.eq(cid))).execute(conn)?;

    Ok(res)
}

pub fn get_from_id(conn: &SqliteConnection, cid: &String) -> Result<Card, DbError> {
    use crate::database::schema::cards::dsl::*;
    let res = cards.filter(id.eq(cid)).first::<Card>(conn)?;

    Ok(res)
}

pub fn update_card<'a>(
    conn: &SqliteConnection,
    cid: &String,
    codename: &'a Option<String>,
    title: &'a Option<String>,
    description: &'a Option<String>,
    priority: &'a Option<i32>,
    column: &'a Option<String>,
    assigned_to: &'a Option<String>,
) -> Result<usize, DbError> {
    use crate::database::schema::cards::dsl::{cards, id};

    let new_card = UpdateCard {
        codename: codename.as_ref(),
        title: title.as_ref(),
        description: description.as_ref(),
        priority: priority.as_ref(),
        column: column.as_ref(),
        assigned_to: assigned_to.as_ref(),
    };

    let result = diesel::update(cards.filter(id.eq(cid)))
        .set(new_card)
        .execute(conn)?;

    Ok(result)
}
