create table if not exists folders (
    id integer primary key,
    parent_id integer null,
    name text not null
);

create table if not exists files (
    id integer primary key,
    folder_id integer not null,
    name text not null,
    hash text not null,
    size integer not null,
    modified_date text not null
);
