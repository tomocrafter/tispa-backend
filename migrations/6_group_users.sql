create table "group_users"
(
  id uuid primary key default uuid_generate_v1mc(),

  group_id uuid not null references "groups" (id) on delete cascade,

  user_id uuid not null references "users" (id) on delete cascade,

  created_at timestamptz not null default now(),

  updated_at timestamptz
);

SELECT trigger_updated_at('"group_users"');
