create table oauth_flow (
    state_hash bytea primary key,
    provider text not null,
    pkce_verifier text not null,
    expires_at timestamptz not null,
    created_at timestamptz not null default now(),
    constraint oauth_flows_state_hash_length check (octet_length(state_hash) = 32)
);

create index oauth_flows_expires_at_idx on oauth_flow(expires_at);
