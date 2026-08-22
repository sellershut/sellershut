create table category_scheme (
    id uuid primary key,
    ap_id text not null unique,
    name text not null,
    owner_ap_id text,
    top_concepts_ap_id text,
    is_local boolean not null default false,
    -- can be null for local records because local records use
    -- created_at and updated_at directly.
    ap_published_at timestamptz,
    ap_updated_at timestamptz,

    -- When we most recently fetched/refreshed said scheme
    last_refreshed_at timestamptz,

    -- for local ones
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),

    constraint category_scheme_ap_timestamps_valid check (
        ap_published_at is null
        or ap_updated_at is null
        or ap_updated_at >= ap_published_at
    )
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

    ap_published_at timestamptz,
    ap_updated_at timestamptz,
    last_refreshed_at timestamptz,
    
    -- Local persistence timestamps.
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),

    constraint category_id_scheme_unique
        unique (id, scheme_id),

    constraint category_parent_same_scheme_fk
        foreign key (parent_id, scheme_id)
        references category(id, scheme_id)
        on delete restrict,

    constraint category_not_own_parent
        check (parent_id is null or parent_id <> id),

    constraint category_ap_timestamps_valid check (
        ap_published_at is null
        or ap_updated_at is null
        or ap_updated_at >= ap_published_at
    )
);

create index category_scheme_id_idx
    on category(scheme_id);
-- so we can paginate direct children:
--
-- where scheme_id = $1
--   and parent_id = $2
--   and id > $3
-- order by id #ulid clutch
create index category_children_page_idx
    on category(scheme_id, parent_id, id);

-- Smaller index specifically for paginating root categories.
create index category_top_concepts_page_idx
    on category(scheme_id, id)
    where parent_id is null;


create trigger update_category_scheme_updated_at
    before update on category_scheme
    for each row
    execute function update_updated_at_column();

create trigger update_category_updated_at
    before update on category
    for each row
    execute function update_updated_at_column();
