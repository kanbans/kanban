table! {
    boards (id) {
        id -> Text,
        name -> Text,
        belongs_to -> Text,
    }
}

table! {
    card_tags (card_id) {
        card_id -> Text,
        tag_id -> Text,
    }
}

table! {
    cards (id) {
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
    columns (id) {
        id -> Text,
        name -> Text,
        belongs_to -> Text,
    }
}

table! {
    sessions (session_token) {
        session_token -> Text,
        belongs_to -> Text,
    }
}

table! {
    tags (id) {
        id -> Text,
        text -> Text,
        color -> Text,
    }
}

table! {
    users (id) {
        id -> Text,
        name -> Text,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
    }
}

joinable!(boards -> users (belongs_to));
joinable!(card_tags -> cards (card_id));
joinable!(card_tags -> tags (tag_id));
joinable!(cards -> columns (column));
joinable!(columns -> boards (belongs_to));
joinable!(sessions -> users (belongs_to));

allow_tables_to_appear_in_same_query!(
    boards,
    card_tags,
    cards,
    columns,
    sessions,
    tags,
    users,
);
