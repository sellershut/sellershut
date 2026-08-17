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
    email text not null,
    email_normalised text not null,
    expires_at timestamptz not null,
    created_at timestamptz not null default now(),
    constraint pending_oauth_logins_identity_key unique (provider, provider_subject),
    constraint pending_oauth_logins_token_hash_length check (octet_length(token_hash) = 32)
);

create index pending_oauth_logins_expires_at_idx on pending_oauth_login(expires_at);
