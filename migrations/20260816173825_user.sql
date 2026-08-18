create extension if not exists citext;

create type user_kind as enum (
    'person',
    'service',
    'organization',
    'group',
    'application'
);

create table "user" (
    id uuid primary key,
    username citext not null,
    name text,
    inbox citext not null,
    public_key text not null,
    private_key text,
    kind user_kind not null default 'person',
    last_refreshed_at timestamptz not null,
    is_local boolean not null,
    created_at timestamptz not null default now(),
    -- private_key is NULL if and only if local is FALSE
    constraint check_local_private_key 
    check ((private_key is null) = (is_local = false))
);

create index idx_users_local_true on "user"(is_local) where is_local = true;
