create table "schedules"
(
  id uuid primary key default uuid_generate_v1mc(),

  event_id uuid not null references "events" (id) on delete cascade,

  group_id uuid not null references "groups" (id) on delete cascade,

  name character(16) not null,

  description text,

  starts_at timestamptz not null,

  ends_at timestamptz not null,

  created_at timestamptz not null default now(),

  updated_at timestamptz
);

SELECT trigger_updated_at('"schedules"');
