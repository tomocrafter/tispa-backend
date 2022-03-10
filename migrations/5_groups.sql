create table "groups"
(
  id uuid primary key default uuid_generate_v1mc(),

  event_id uuid not null references "events" (id) on delete cascade,

  name character(16) not null,

  created_at timestamptz not null default now(),

  updated_at timestamptz
);

SELECT trigger_updated_at('"groups"');
