create extension if not exists citext;

create type user_kind as enum (
    'Person',
    'Service',
    'Organization',
    'Group',
    'Application'
);

create table "user" (
    id uuid primary key,
    ap_id text not null unique,
    username citext not null,
    name text,
    inbox citext not null,
    public_key text not null,
    private_key text,
    avatar text,
    kind user_kind not null default 'Person',
    last_refreshed_at timestamptz not null default now(),
    is_local boolean not null,
    created_at timestamptz not null default now(),
    -- private_key is NULL if and only if local is FALSE
    constraint check_local_private_key 
    check ((private_key is null) = (is_local = false))
);

create index idx_users_local_true on "user"(is_local) where is_local = true;
