create table "federated_user" (
    ap_id varchar primary key,
    username varchar(20) not null,
    last_refreshed_at timestamptz not null,
    private_key text,
    public_key text not null,
    inbox text not null unique,
    followers varchar[] not null default '{}',
    local boolean not null,
    created_at timestamptz default current_timestamp not null,
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
before update on "federated_user"
for each row
execute function update_updated_at();

