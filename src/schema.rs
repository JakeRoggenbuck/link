table! {
    redirects (id) {
        id -> Int4,
        alias -> Varchar,
        url -> Varchar,
        count -> Int4,
    }
}

table! {
    tokens (id) {
        id -> Int4,
        token -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    redirects,
    tokens,
);
