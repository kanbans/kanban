table! {
    card (id) {
        id -> Text,
        codename -> Text,
        title -> Text,
        description -> Text,
        priority -> Integer,
        column -> Text,
        created_at -> Timestamp,
        created_by -> Text,
        assigned_to -> Nullable<Text>,
    }
}

table! {
    card_tags (card_id) {
        card_id -> Text,
        tag_id -> Text,
    }
}

table! {
    column (id) {
        id -> Nullable<Text>,
        name -> Text,
    }
}

table! {
    tag (id) {
        id -> Text,
        text -> Text,
        color -> Text,
    }
}

table! {
    user (id) {
        id -> Text,
        name -> Text,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
    }
}

joinable!(card -> column (column));
joinable!(card_tags -> card (card_id));
joinable!(card_tags -> tag (tag_id));

allow_tables_to_appear_in_same_query!(
    card,
    card_tags,
    column,
    tag,
    user,
);
