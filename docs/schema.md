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

  entity event_users {
    + id [PK]
    ---
    # user_id [FK]
    # event_id [FK]
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

@enduml
```

## `users`テーブル

ユーザー情報を管理する
Twitterのアカウントの紐付けもここで管理

## `events`テーブル

イベントを管理する

## `event_users`テーブル

イベントに参加するユーザーを管理する

## `groups`テーブル

イベントごとにあるグループを管理する

## `group_users`テーブル

グループに所属するユーザーを管理する

## `schedules`テーブル

スケジュールを管理する

## `expenses`テーブル

イベントごとにかかったそれぞれの費用を管理する
