drop table if exists courses;

create table courses
(
    id serial primary key,
    tutor_id INT not null,
    name varchar(140) not null,
    description varchar(2000),
    format varchar(30),
    structure varchar(200),
    language varchar(30),
    level varchar(30),
    duration INT,
    price real,
    created_at TIMESTAMP default now(),
    updated_at TIMESTAMP default now()
);