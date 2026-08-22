create table actor (
    id uuid primary key,
    ap_id text not null unique,

    preferred_username text not null,
    name text,
    summary text,

    inbox text not null,
    outbox text not null,

    following text, -- optional but required for local
    followers text, -- optional but required for local

    liked text, -- optional but required for local
    icon text,
    kind text not null,

    is_local boolean not null,

    last_refreshed_at timestamptz not null default now(),
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create index idx_user_local_true on actor (is_local) where is_local = true;
create unique index idx_user_local_username on actor(lower(preferred_username)) where is_local = true;
