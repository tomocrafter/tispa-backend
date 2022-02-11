create table "users"
(
  id uuid primary key default uuid_generate_v1mc(),

  screen_name text collate "case_insensitive" unique not null,

  profile_image_url text,

  created_at timestamptz not null default now(),

  updated_at timestamptz
);

SELECT trigger_updated_at('"users"');
