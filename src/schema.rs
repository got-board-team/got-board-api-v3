table! {
    matches (id) {
        id -> Int4,
        name -> Varchar,
        players_count -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    matches_users (id) {
        id -> Int4,
        match_id -> Int4,
        user_id -> Int4,
        house_name -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    pieces (id) {
        id -> Int4,
        match_id -> Int4,
        piece_type -> Varchar,
        x -> Int4,
        y -> Int4,
        location -> Varchar,
        house_name -> Nullable<Varchar>,
        spec -> Nullable<Jsonb>,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    matches,
    matches_users,
    pieces,
    users,
);
