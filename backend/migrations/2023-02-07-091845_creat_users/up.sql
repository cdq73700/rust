-- Your SQL goes here
create table users (
    id varchar(36) primary key,
    name varchar(255) not null
) character set utf8mb4 collate utf8mb4_bin;