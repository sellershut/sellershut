create table "user" (
    id varchar(21) primary key,
    username varchar(20) not null,
    inbox text not null,
    public_key_pem text not null,
    private_key_pem text,
    last_refreshed_at timestamptz not null,
    followers varchar[] not null default '{}',
    local boolean not null,
    ap_id varchar not null,
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
before update on "user"
for each row
execute function update_updated_at();
