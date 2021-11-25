table! {
    user (id) {
        id -> Nullable<Integer>,
        name -> Text,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
    }
}
