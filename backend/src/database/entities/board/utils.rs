use super::model::NewBoard;
use crate::database::entities::board::model::Board;
use crate::database::schema;
use crate::database::utils::DbError;
use diesel::*;
use uuid::Uuid;

pub fn create_board<'a>(conn: &SqliteConnection, name: &'a String) -> Result<usize, DbError> {
    let id = Uuid::new_v4().to_string();

    let new_board = NewBoard { id, name };

    let result = diesel::insert_into(schema::boards::table)
        .values(&new_board)
        .execute(conn)?;

    Ok(result)
}

pub fn get_boards<'a>(conn: &SqliteConnection) -> Result<Vec<Board>, DbError> {
    use crate::database::schema::boards::dsl::*;
    let res = boards.load::<Board>(conn)?;

    Ok(res)
}
