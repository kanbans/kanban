use crate::database::schema::card_tags;

#[derive(Queryable)]
pub struct CardTag {
    pub card_id: String,
    pub tag_id: String
}

#[derive(Insertable)]
#[table_name = "card_tags"]
pub struct NewCardTag<'a> {
    pub card_id: &'a String,
    pub tag_id: &'a String,
}
