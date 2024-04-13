drop table if exists tutor_storefront;

create table tutor_storefront
(
    id serial primary key,
    tutor_id INT not null,
    name varchar(140) not null,
    created_at TIMESTAMP default now()
);