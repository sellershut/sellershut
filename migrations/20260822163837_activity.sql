create table activity (
    id uuid primary key,
    user_id uuid not null references actor(id) on delete cascade,
    actor_ap_id text not null,
    activity_type text not null, -- e.g., 'create', 'follow', 'like', 'delete'
    object_ap_id text,           -- the target of the activity (nullable for some activity types)
    payload jsonb not null,      -- the full activitypub json-ld object
    created_at timestamptz not null default now()
);

create index idx_activity_actor on activity (actor_ap_id);
create index idx_activity_type on activity (activity_type);
create index idx_activity_object on activity (object_ap_id);
create index idx_activity_payload_gin on activity using gin (payload);
