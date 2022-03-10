create table "expenses"
(
  id uuid primary key default uuid_generate_v1mc(),

  event_id uuid not null references "events" (id) on delete cascade,

  group_id uuid not null references "groups" (id) on delete cascade,

  category category not null,

  description text,

  cost integer not null,

  created_at timestamptz not null default now(),

  updated_at timestamptz,

  constraint cost_nonnegative check (cost >= 0)
);

SELECT trigger_updated_at('"expenses"');
