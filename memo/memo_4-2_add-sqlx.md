# sqlx の導入

## cli ツールのインストール

```sh
cargo install sqlx-cli
```

## マイグレーションの作成

```sh
sqlx migrate add マイグレーション名
```

## マイグレーションファイルの編集

- `migrations/20230928105731_init.sql`

  ```sql
  -- Todo テーブルの作成
  CREATE TABLE todos
  {
      id          SERIAL PRIMARY kEY,
      text        TEXT    NOT NULL,
      completed   BOOLEAN NOT NULL DEFAULT false,
  };
  ```

## マイグレーションの実行

- Makefile に以下の記述を追加

  ```Makefile
  dev:
      sqlx db create
      sqlx migrate run
      cargo watch -x run
  ```

## .env ファイルの作成

- sqlx は `.env` や環境変数からデータベースの接続先を認識する
- `.env` を作成

  ```sh
  DATABASE_URL="postgres://admin:admin@localhost:5432/todos"
  ```

## 動作確認

```sh
make build
make db
make dev
```

```sh
make connect
```

```sh
3b9544b3c2c8:/# psql -U admin todos
psql (13.12)
Type "help" for help.

todos=# \dt
             List of relations
 Schema |       Name       | Type  | Owner 
--------+------------------+-------+-------
 public | _sqlx_migrations | table | admin
 public | todos            | table | admin
(2 rows)

todos=# \d todos
                              Table "public.todos"
  Column   |  Type   | Collation | Nullable |              Default              
-----------+---------+-----------+----------+-----------------------------------
 id        | integer |           | not null | nextval('todos_id_seq'::regclass)
 text      | text    |           | not null | 
 completed | boolean |           | not null | false
Indexes:
    "todos_pkey" PRIMARY KEY, btree (id)

todos=# exit
```
