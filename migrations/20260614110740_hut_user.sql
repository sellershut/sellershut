-- https://www.w3.org/TR/activitystreams-vocabulary/#actor-types
create type user_kind as enum (
  'application',
  'group',
  'organisation',
  'person',
  'service'
);

create type user_role as enum ('sys-admin', 'user');

create table "hut_user" (
  id uuid primary key,
  -- user role
  role user_role not null default 'user',
  -- user type
  kind user_kind not null default 'person',
  -- profile picture
  avatar text,
  -- profile description
  description text,
  -- profile creation
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create or replace function update_updated_at_column()
returns trigger as $$
begin
    new.updated_at = now();
    return new;
end;
$$ language 'plpgsql';

create trigger update_hut_user_updated_at
before update on hut_user
for each row
execute procedure update_updated_at_column();
