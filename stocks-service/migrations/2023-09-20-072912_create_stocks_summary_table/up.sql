create table stocks_summary (
    id serial primary key,
    symbol varchar not null unique,
    shares integer,
    total_value varchar(50),
    lowest_price varchar(50),
    highest_price varchar(50),
    average_price varchar(50),
    user_id integer references users not null
);