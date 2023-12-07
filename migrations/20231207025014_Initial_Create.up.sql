-- Add up migration script here

create type itinerary_status as enum ('draft', 'published', 'archived');
create type itinerary_share_type as enum ('editor', 'viewer');
create type travel_leg_type as enum ('flight', 'train', 'bus', 'car', 'ferry', 'other');
-- create type 

CREATE TABLE IF NOT EXISTS users
(
    id         serial                                 not null
        constraint users_pk primary key,
    name       varchar(255)                           not null,
    email      varchar(255)                           not null
        constraint users_email_unique
            unique,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp with time zone default now() not null
);

create table if not exists itineraries
(
    id         serial                                 not null
        constraint itineraries_pk
            primary key,
    name       varchar(255)                           not null,
    user_id    integer                                not null
        constraint itineraries_users_id_fk
            references users
            on update cascade on delete cascade,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp with time zone default now() not null,
    start_date date                                   not null,
    end_date   date                                   not null
);


create table if not exists itinerary_shares
(
    id            serial               not null
        constraint itinerary_shares_pk
            primary key,
    itinerary_id  integer              not null
        constraint itinerary_shares_itineraries_id_fk
            references itineraries
            on update cascade on delete cascade,
    user_id       integer              not null
        constraint itinerary_shares_users_id_fk
            references users
            on update cascade on delete cascade,
    share_type    itinerary_share_type not null,
    share_message varchar(255)         not null
);


create table if not exists itinerary_items
(
    id           serial       not null
        constraint itinerary_items_pk
            primary key,
    itinerary_id integer      not null
        constraint itinerary_items_itineraries_id_fk
            references itineraries
            on update cascade on delete cascade,
    name         varchar(255) not null
);

create table if not exists stays
(
    id         serial                                 not null
        constraint stay_pk
            primary key,
    summary    varchar(255)                           not null,
    start_date timestamp with time zone default now() not null,
    end_date   timestamp with time zone default now() not null,
    location   varchar(255)                           not null,
    notes      varchar(255)                           not null
);

create table if not exists activities
(
    id         serial                                 not null
        constraint activities_pk
            primary key,
    summary    varchar(255)                           not null,
    start_date timestamp with time zone default now() not null,
    end_date   timestamp with time zone default now() not null,
    location   varchar(255)                           not null,
    notes      varchar(255)                           not null
);

create table if not exists travel_legs
(
    id                serial                                 not null
        constraint travel_legs_pk
            primary key,
    itinerary_item_id integer                                not null
        constraint travel_legs_itinerary_items_id_fk
            references itinerary_items
            on update cascade on delete cascade,
    travel_leg_type   travel_leg_type                        not null,
    start_date        timestamp with time zone default now() not null,
    end_date          timestamp with time zone default now() not null,
    start_location    varchar(255)                           not null,
    end_location      varchar(255)                           not null,
    notes             varchar(255)                           not null
);
