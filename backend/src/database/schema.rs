table! {
    card (id) {
        id -> Text,
        codename -> Text,
        title -> Text,
        description -> Text,
        priority -> Integer,
        column -> Text,
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

allow_tables_to_appear_in_same_query!(
    card,
    column,
    tag,
    user,
);
