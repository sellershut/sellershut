create table "user" (
    id varchar(21) primary key,
    username varchar(20) not null,
    email varchar(255) unique not null,
    avatar_url varchar,
    followers varchar[] not null default '{}',
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
