drop table if exists courses cascade;
drop table if exists tutors;

create table tutors (
    id serial primary key,
    name varchar(200) not null,
    photo_url varchar(200) not null,
    bio varchar(2000) not null
);

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
    updated_at TIMESTAMP default now(),
    CONSTRAINT fk_tutor
    FOREIGN KEY(tutor_id)
        REFERENCES tutors(id)
        ON DELETE cascade
);