use super::model::{Column, NewColumn};
use crate::database::entities::column::model::UpdateColumn;
use crate::database::schema;
use crate::database::utils::DbError;
use diesel::*;
use uuid::Uuid;

pub fn get_columns(conn: &SqliteConnection, board_id: &String) -> Result<Vec<Column>, DbError> {
    use crate::database::schema::columns::dsl::*;
    let res = columns
        .filter(belongs_to.eq(board_id))
        .load::<Column>(conn)?;

    Ok(res)
}

pub fn create_column<'a>(
    conn: &SqliteConnection,
    name: &'a String,
    belongs_to: &'a String,
) -> Result<Column, DbError> {
    let id = Uuid::new_v4().to_string();

    let new_column = NewColumn {
        id: id.clone(),
        name,
        belongs_to,
    };
    
    diesel::insert_into(schema::columns::table)
        .values(&new_column)
        .execute(conn)?;

    Ok(Column{
        id: id.clone(),
        name: name.clone(), belongs_to: belongs_to.clone(),
    })
}

pub fn delete_column(conn: &SqliteConnection, cid: &String) -> Result<usize, DbError> {
    use crate::database::schema::columns::dsl::*;
    let res = diesel::delete(columns.filter(id.eq(cid))).execute(conn)?;

    Ok(res)
}

pub fn update_column(
    conn: &SqliteConnection,
    cid: &String,
    new_name: &Option<String>,
    new_belongs_to: &Option<String>,
) -> Result<usize, DbError> {
    use crate::database::schema::columns::dsl::*;

    let new_column = UpdateColumn {
        name: new_name.as_ref(),
        belongs_to: new_belongs_to.as_ref(),
    };

    let res = diesel::update(columns.filter(id.eq(cid)))
        .set(new_column)
        .execute(conn)?;

    Ok(res)
}

pub fn get_from_id(conn: &SqliteConnection, cid: &String) -> Result<Column, DbError> {
    use crate::database::schema::columns::dsl::*;
    let res = columns.filter(id.eq(cid)).first::<Column>(conn)?;

    Ok(res)
}
