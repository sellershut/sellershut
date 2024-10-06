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

create table if not exists session (
  id serial primary key,
  user_id varchar(21) not null unique,
  session_id varchar not null,
  expires_at timestamptz not null,
  foreign key (user_id) references "user"(id)
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
