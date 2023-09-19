create table users (
    id serial primary key,
    name varchar not null unique,
    email varchar(50) not null
);

create table stocks (
    id serial primary key,
    symbol varchar not null unique,
    unit_cost numeric,
    user_id integer references users not null
);

-- create table stocks (
--     id serial primary key,
--     symbol varchar not null unique,
--     share_held numeric(30) default 0,
--     profit_loss numeric(30) default 0,
--     unit_cost decimal default 0.0,
--     lowest_price decimal default 0.0,
--     highest_price decimal default 0.0,
--     average_price decimal default 0.0,
--     created_at datetime,
--     modified_at datetime,
--     user_id integer references users not null
-- );