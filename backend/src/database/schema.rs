table! {
    board (id) {
        id -> Text,
        name -> Text,
    }
}

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
joinable!(column -> board (belongs_to));
joinable!(sessions -> user (belongs_to));

allow_tables_to_appear_in_same_query!(
    board,
    card,
    card_tags,
    column,
    sessions,
    tag,
    user,
);
