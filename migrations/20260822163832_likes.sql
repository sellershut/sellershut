create table likes (
    user_id uuid not null references actor(id) on delete cascade,
    actor_ap_id text not null,
    object_ap_id text not null,
    created_at timestamptz not null default now(),
    primary key (user_id, object_ap_id)
);

create index idx_like_actor on likes (actor_ap_id);
create index idx_like_object on likes (object_ap_id);
