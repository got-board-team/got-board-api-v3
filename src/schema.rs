table! {
    matches (id) {
        id -> Int4,
        name -> Varchar,
        players_count -> Int4,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
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
    posts,
    users,
);
