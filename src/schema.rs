table! {
    matches (id) {
        id -> Int4,
        name -> Varchar,
        players_count -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        match_id -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    matches,
    users,
);
