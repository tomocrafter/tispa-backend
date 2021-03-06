# スキーマ設計

```plantuml
@startuml

package database <<Database>> {
  entity users {
    + id [PK]
    ---
    name
    screen_name
    profile_image_url
  }

  entity accounts {
    + id [PK]
    ---
    # user_id [FK]
    provider(twitter)
    provider_account_id
    access_token
    refresh_token
    expires_at
  }

  entity events {
    + id [PK]
    ---
    name
    starts_at
    ends_at
  }

  entity participants {
    + id [PK]
    ---
    # event_id [FK]
    # user_id [FK]
  }

  entity groups {
    + id [PK]
    ---
    # event_id [FK]
    name
  }

  entity group_users {
    + id [PK]
    ---
    # group_id [FK]
    # user_id [FK]
  }

  entity schedules {
    + id [PK]
    ---
    # event_id [FK]
    # group_id [FK]
    name
    description
    starts_at
    ends_at
  }

  entity expenses {
    + id [PK]
    ---
    # event_id [FK]
    # user_id [FK]
    category(transportation, food, lodging)
    description
    cost
  }
}

users ||-o{ accounts
users ||-d-o{ participants
users ||-r-o{ group_users
users ||-o{ expenses

events ||-l-o{ participants
events ||-u-o{ groups
events ||-d-o{ schedules
events ||-o{ expenses

groups ||-o{ schedules

group_users ||-o{ groups

@enduml
```

## `users`テーブル

ユーザーのプロフィールを管理する

## `accounts` テーブル

ソーシャルログインを管理する

Twitter のアカウントの紐付けはここで管理

## `events`テーブル

イベントを管理する

## `participants`テーブル

イベントに参加するユーザーを管理する

## `groups`テーブル

イベントごとにあるグループを管理する

## `group_users`テーブル

グループに所属するユーザーを管理する

## `schedules`テーブル

スケジュールを管理する

## `expenses`テーブル

イベントごとにかかったそれぞれの費用を管理する
