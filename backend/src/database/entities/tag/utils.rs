use diesel::*;
use super::model::NewTag;
use crate::database::schema;

pub fn create_tag<'a>(
	conn: &SqliteConnection,
	text: &'a String,
	color: &'a String,
) -> Result<usize, diesel::result::Error> {
	let new_tag = NewTag {
		text,
		color,
	};

	let result = diesel::insert_into(schema::tag::table)
		.values(&new_tag)
		.execute(conn)?;

	Ok(result)
}