# スキーマ設計

```plantuml
@startuml

package database <<Database>> {
  entity users {
    + id [PK]
    ---
    name
    twitter_id
  }

  entity events {
    + id [PK]
    ---
    name
    started_at
    ended_at
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
    started_at
    ended_at
  }

  entity expenses {
    + id [PK]
    ---
    # event_id [FK]
    # user_id [FK]
    category(transportation, food, lodging)
    description
    cost
    created_at
    updated_at
  }
}

users ||-d-o{ participants
users ||-r-o{ group_users
users ||-o{ expenses

events ||-l-o{ participants
events ||-u-o{ groups
events ||-d-o{ schedules
events ||-o{ expenses

@enduml
```

## `users`テーブル

ユーザー情報を管理する
Twitterのアカウントの紐付けもここで管理

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
