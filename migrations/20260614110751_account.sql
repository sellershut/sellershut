create extension if not exists citext;

create table account (
    -- oauth provider name
    provider_id text not null,
    -- oauth provider's user id
    provider_user_id text not null,
    -- email linked to said acc
    email citext not null,
    -- user
    user_id uuid not null references hut_user(id) on delete cascade,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    primary key (provider_id, provider_user_id)
);

create index if not exists account_email_idx on account (email);
-- One hut user can link at most one account per provider.
create unique index if not exists account_user_provider_idx
on account (user_id, provider_id);

create trigger update_account_updated_at
before update on account
for each row
execute procedure update_updated_at_column();
