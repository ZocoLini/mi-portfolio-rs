-- Migration script for version 1
-- Creating tables needed to track qr scans

create table qr_scan (
    id integer primary key autoincrement not null,
    reference varchar(255) null,
    created_at timestamp default current_timestamp not null,
    session_id integer not null,
    ip varchar(15) null
)
