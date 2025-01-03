create table "user" (
    id varchar(21) primary key,
    ap_id varchar unique not null,
    username varchar not null,
    display_name varchar,
    avatar_url varchar,
    email varchar,
    inbox varchar not null,
    public_key varchar not null,
    private_key varchar,
    local boolean not null,
    followers varchar[] not null default '{}',
    created_at timestamptz default current_timestamp not null,
    last_refreshed_at timestamptz default current_timestamp not null,
    updated_at timestamptz default current_timestamp not null
);

create or replace function update_updated_at()
returns trigger as $$
begin
    new.updated_at = current_timestamp;
    return new;
end;
$$ language plpgsql;

create trigger set_updated_at
before update on "user"
for each row
execute function update_updated_at();
