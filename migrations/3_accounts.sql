create table "accounts"
(
  id uuid primary key default uuid_generate_v1mc(),

  user_id uuid not null references "users" (id) on delete cascade,

  provider provider not null,

  provider_account_id text not null,

  access_token text not null,

  refresh_token text not null,

  expires_at timestamptz not null,

  created_at timestamptz not null default now(),

  updated_at timestamptz,

  UNIQUE(user_id, provider)
);

SELECT trigger_updated_at('"accounts"');
