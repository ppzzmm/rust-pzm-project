diesel::table! {
    stocks (id) {
        id -> Int4,
        symbol -> Varchar,
        shares -> Integer,
        price -> Varchar,
        percentage_change -> Varchar,
        action_type -> Varchar,
        user_id -> Int4,
    }
}

diesel::table! {
    stocks_summary (id) {
        id -> Int4,
        symbol -> Varchar,
        shares -> Integer,
        total_value -> Varchar,
        lowest_price -> Varchar,
        highest_price -> Varchar,
        average_price -> Varchar,
        price_by_hours -> Varchar,
        profit_loss -> Varchar,
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

