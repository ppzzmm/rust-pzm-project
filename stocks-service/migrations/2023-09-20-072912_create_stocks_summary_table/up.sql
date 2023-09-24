create table stocks_summary (
    id serial primary key,
    symbol varchar not null unique,
    shares integer,
    profit_loss varchar,
    total_value varchar,
    lowest_price varchar,
    highest_price varchar,
    average_price varchar,
    price_by_hours varchar,
    user_id integer references users not null
);