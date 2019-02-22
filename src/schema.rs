table! {
    files (id) {
        id -> Nullable<Integer>,
        folder_id -> Integer,
        name -> Text,
        hash -> Text,
        size -> Integer,
        modified_date -> Text,
    }
}

table! {
    folders (id) {
        id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        name -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    files,
    folders,
);
