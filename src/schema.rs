table! {
    files (id) {
        id -> Integer,
        folder_id -> Integer,
        name -> Text,
        hash -> Text,
        size -> Integer,
        modified_date -> Text,
    }
}

table! {
    folders (id) {
        id -> Integer,
        parent_id -> Integer,
        name -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    files,
    folders,
);
