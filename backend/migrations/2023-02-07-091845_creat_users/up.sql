-- Your SQL goes here
create table users (
    id binary(16) primary key not null default (UUID_TO_BIN(UUID(), TRUE)),
    email varchar(255) not null,
    name varchar(255) not null,
    password varchar(255) not null,
    created_at DATETIME not null default NOW(),
    created_by varchar(255) not null,
    updated_at DATETIME not null default NOW(),
    updated_by varchar(255) not null,
    deleted_at DATETIME default null,
    deleted_by varchar(255) default null
) character set utf8mb4 collate utf8mb4_bin;