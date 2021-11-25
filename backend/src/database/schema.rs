table! {
    tag (id) {
        id -> Integer,
        text -> Text,
        color -> Text,
    }
}

table! {
    user (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(tag, user,);
