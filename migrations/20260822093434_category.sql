create table category_scheme (
    id uuid primary key,
    ap_id text not null unique,
    name text not null,
    owner_ap_id text,
    is_local boolean not null default false,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create table category (
    -- ulid
    id varchar(26) primary key,
    ap_id text not null unique,

    scheme_id uuid not null
        references category_scheme(id)
        on delete cascade,

    name text not null,
    description text,
    image_url text,

    -- ulid
    parent_id varchar(26),

    is_local boolean not null default false,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    constraint category_id_scheme_unique unique (id, scheme_id),
    constraint category_parent_same_scheme_fk
        foreign key (parent_id, scheme_id)
        references category(id, scheme_id),

    check (parent_id is null or parent_id <> id)
);

create or replace function update_updated_at_column()
returns trigger
language plpgsql
as $$
begin
    new.updated_at = now();
    return new;
end;
$$;

create trigger update_category_scheme_updated_at
    before update on category_scheme
    for each row
    execute function update_updated_at_column();

