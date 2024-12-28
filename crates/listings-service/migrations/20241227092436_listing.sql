create type status as enum ('active', 'sold', 'inactive');
create type condition as enum ('unspecified', 'new', 'used', 'refurbished', 'like_new', 'for_parts', 'damaged');

create table listing (
    id varchar(21) primary key,
    user_ap_id varchar not null,
    local boolean not null,
    negotiable boolean default false,
    title varchar not null,
    quantity int default 1,
    description varchar not null,
    expires_at timestamptz,
    created_at timestamptz default current_timestamp not null,
    updated_at timestamptz default current_timestamp not null,
    ap_id varchar not null,
    status status default 'active',
    currency_code char(3) not null,  -- iso 4217 3-character currency code
    attachments varchar[],
    condition condition default 'unspecified',
    condition_details varchar,
    category_ap_id varchar not null,
    curr_units bigint not null,           -- whole units of the amount (e.g., 1 usd, 100 jpy)
    curr_nanos int not null               -- nano units, must be between -999,999,999 and +999,999,999
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
