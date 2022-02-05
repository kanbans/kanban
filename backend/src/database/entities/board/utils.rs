use super::model::NewBoard;
use crate::database::entities::board::model::Board;
use crate::database::schema;
use crate::database::utils::DbError;
use diesel::*;
use uuid::Uuid;

pub fn create_board<'a>(
    conn: &SqliteConnection,
    name: &'a String,
    belongs_to: &'a String,
) -> Result<usize, DbError> {
    let id = Uuid::new_v4().to_string();

    let new_board = NewBoard {
        id,
        name,
        belongs_to,
    };

    let result = diesel::insert_into(schema::boards::table)
        .values(&new_board)
        .execute(conn)?;

    Ok(result)
}

pub fn get_from_id(conn: &SqliteConnection, cid: String) -> Result<Board, DbError> {
    use crate::database::schema::boards::dsl::*;
    let res = boards.filter(id.eq(cid)).first::<Board>(conn)?;

    Ok(res)
}

pub fn delete(conn: &SqliteConnection, cid: String) -> Result<usize, DbError> {
    use crate::database::schema::boards::dsl::*;
    let res = diesel::delete(boards.filter(id.eq(cid))).execute(conn)?;

    Ok(res)
}

pub fn update_name(
    conn: &SqliteConnection,
    cid: String,
    new_name: String,
) -> Result<usize, DbError> {
    use crate::database::schema::boards::dsl::*;
    let res = diesel::update(boards.filter(id.eq(cid)))
        .set(name.eq(new_name))
        .execute(conn)?;

    Ok(res)
}

pub fn get_boards(conn: &SqliteConnection, user_id: String) -> Result<Vec<Board>, DbError> {
    use crate::database::schema::boards::dsl::*;
    let res = boards.filter(belongs_to.eq(user_id)).load::<Board>(conn)?;

    Ok(res)
}
