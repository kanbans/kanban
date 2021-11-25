use diesel::*;
use super::model::NewUser;
use crate::database::schema;

pub fn create_user<'a>(
	conn: &SqliteConnection,
	name: &'a String,
	email: &'a String,
	password: &'a String,
) -> Result<usize, diesel::result::Error> {
	let new_user = NewUser {
		name,
		email,
		password,
	};

	let result = diesel::insert_into(schema::user::table)
		.values(&new_user)
		.execute(conn)?;

	Ok(result)
}