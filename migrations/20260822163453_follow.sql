create table follow (
    user_id uuid not null references actor(id) on delete cascade,
    follower_ap_id text not null,
    following_ap_id text not null,
    created_at timestamptz not null default now(),
    constraint prevent_self_follow check (follower_ap_id <> following_ap_id),
    primary key (follower_ap_id, following_ap_id)
);

create index idx_follow_follower on follow (follower_ap_id);
create index idx_follow_following on follow (following_ap_id);
