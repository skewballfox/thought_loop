-- Your SQL goes here

create table users (
    id serial primary key,
    first_name varchar(255) not null,
    last_name varchar(255) not null,
    email varchar(255) unique not null,
    password varchar(255) not null,
    created_at timestamp default current_timestamp
);

create table posts (
    id serial primary key,
    user_id int references users(id) not null,
    title varchar(255) not null,
    body text not null,
    published boolean not null default 'f',
    created_at timestamp default current_timestamp,
    updated_at timestamp default current_timestamp
);
