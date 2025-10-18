-- Add migration script here
-- Migrate qr_scan to v2

create table session (
    session_id uuid primary key not null,
    last_activity timestamp default current_timestamp not null
);

---

-- Creating the sessions before migrating qr_scan
insert into session (session_id, last_activity)
select 
    distinct session_id,
    max(created_at) as last_activity
from qr_scan
group by session_id;

create table qr_scan_copy (
    id integer primary key autoincrement not null,
    reference varchar(255) null,
    created_at timestamp default current_timestamp not null,
    session_id uuid not null,
    ip varchar(15) null,
    constraint fk_session foreign key (session_id) references session(session_id)
);

insert into qr_scan_copy (id, reference, created_at, session_id, ip)
select id, reference, created_at, session_id, ip from qr_scan;

drop table qr_scan;

alter table qr_scan_copy rename to qr_scan;

---

create table content_view (
  id integer primary key autoincrement not null,
  session_id uuid not null,
  content_id varchar(255) not null,
  constraint fk_session foreign key (session_id) references session(session_id)
);

