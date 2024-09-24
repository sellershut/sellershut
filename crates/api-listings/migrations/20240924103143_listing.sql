create type status as enum ('active', 'sold', 'inactive');

create table listing (
    id varchar(21) primary key,
    user_id varchar(21) not null,
    local boolean not null,
    title varchar not null,
    quantity int default 1,
    description varchar not null,
    expires_at timestamptz,
    created_at timestamptz default current_timestamp not null,
    updated_at timestamptz default current_timestamp not null,
    ap_id varchar not null,
    status status default 'active'
);

create or replace function update_updated_at()
returns trigger as $$
begin
    new.updated_at = current_timestamp;
    return new;
end;
$$ language plpgsql;

create trigger set_updated_at
before update on listing
for each row
execute function update_updated_at();
