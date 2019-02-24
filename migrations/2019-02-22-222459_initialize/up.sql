create table if not exists folders (
    id integer primary key not null,
    parent_id integer not null,
    name text not null
);

create table if not exists files (
    id integer primary key not null,
    folder_id integer not null,
    name text not null,
    hash text not null,
    size integer not null,
    modified_date text not null
);
