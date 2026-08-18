create extension if not exists citext;

create table oauth_flow (
    state_hash bytea primary key,
    provider text not null,
    pkce_verifier text not null,
    expires_at timestamptz not null,
    created_at timestamptz not null default now(),
    constraint oauth_flows_state_hash_length check (octet_length(state_hash) = 32)
);

create index oauth_flows_expires_at_idx on oauth_flow(expires_at);

create table pending_oauth_login (
    token_hash bytea primary key,
    provider text not null,
    provider_subject text not null,
    email citext not null,
    expires_at timestamptz not null,
    created_at timestamptz not null default now(),
    constraint pending_oauth_logins_identity_key unique (provider, provider_subject),
    constraint pending_oauth_logins_token_hash_length check (octet_length(token_hash) = 32)
);

create index pending_oauth_logins_expires_at_idx on pending_oauth_login(expires_at);

create table "user" (
    id uuid primary key,
    email citext not null,
    username citext not null,
    created_at timestamptz not null default now()
);

create table oauth_identity (
    provider text not null,
    provider_id text not null,
    user_id uuid not null references "user"(id) on delete cascade,
    provider_email citext not null,
    created_at timestamptz not null default now(),
    last_login_at timestamptz not null default now(),
    primary key (provider, provider_id)
);

create index oauth_identities_user_id_idx on oauth_identity(user_id);

create table auth_session (
    token_hash bytea primary key,
    user_id uuid not null references "user"(id) on delete cascade,
    expires_at timestamptz not null,
    created_at timestamptz not null default now(),
    constraint auth_sessions_token_hash_length check (octet_length(token_hash) = 32)
);

create index auth_sessions_user_id_idx on auth_session(user_id);
create index auth_sessions_expires_at_idx on auth_session(expires_at);
