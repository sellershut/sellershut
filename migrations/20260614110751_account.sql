create extension if not exists citext;

create table account (
    -- oauth provider name
    provider_id text not null,
    -- oauth provider's user id
    provider_user_id text not null,
    -- email linked to said acc
    email citext not null,
    -- the upstream user data
    user_data jsonb not null,
    -- user
    user_id uuid not null references hut_user(id) on delete cascade,
    primary key (provider_id, provider_user_id)
);

create index account_email_idx on account (email);
