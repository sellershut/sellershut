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
  created_at timestamptz not null default now()
);
