table! {
    hierarchy_node (id) {
        id -> Text,
        name -> Text,
        parent -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    hierarchy_node,
    users,
);
