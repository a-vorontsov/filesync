table! {
    files (id) {
        id -> Integer,
        file_name -> Text,
        date_created -> BigInt,
        date_modified -> BigInt,
        file_hash -> Text,
    }
}

table! {
    files_history (id) {
        id -> Integer,
        file_id -> BigInt,
        file_hash -> Text,
        date_modified -> BigInt,
    }
}

allow_tables_to_appear_in_same_query!(
    files,
    files_history,
);
