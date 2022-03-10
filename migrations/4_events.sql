create table "events"
(
  id uuid primary key default uuid_generate_v1mc(),

  name character(16) not null,

  starts_at timestamptz not null,

  ends_at timestamptz not null,

  created_at timestamptz not null default now(),

  updated_at timestamptz
);

SELECT trigger_updated_at('"events"');
