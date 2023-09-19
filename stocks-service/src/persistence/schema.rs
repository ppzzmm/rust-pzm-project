diesel::table! {
    stocks (id) {
        id -> Int4,
        symbol -> Varchar,
        user_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar
    }
}

diesel::joinable!(stocks -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    stocks,
    users,
);

