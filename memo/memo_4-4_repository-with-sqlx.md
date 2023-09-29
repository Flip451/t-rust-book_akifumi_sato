# sqlx を用いてリポジトリを実装する

## sqlx の使用例

### `query`, `query_as`

- `query`: クエリを生成する. 実行時の返り値の型は `Row<'conn>`
- `query_as`: クエリを生成する. 実行時の返り値の型は、ジェネリック引数の二番目の型で指定したものになる

  ```rust
  // この derive マクロによって `query_as` の返り値の型に指定できるようになる
  #[derive(sqlx::FromRow)]
  struct User {
      id: i64,
      name: String,
  }

  // - sqlx::query_as の引数には、SQL 文を生成するフォーマット文を渡す
  // - ジェネリック引数の第二引数に戻り値の型を指定できる
  // - sqlx::query_as の返り値は `QueryAs` 型（クエリをあらわす）
  // - `QueryAs` に対して `bind` メソッドを呼び出すことで、SQL フォーマット文の中のパラメータに値をあてはめることができる
  // - フォーマット文中では、パラメータは `?`, `$1`, `$2` などであらわす
  // - `fetch`, `excute`, `fetch_all` などでクエリを実行する
  let users: Result<Vec<User>, Error> = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
      .bind(user_email)
      .fetch_all(&conn)
      .await;
  ```

### `fetch`, `fetch_xxx`, `excute`

- `excute`: データ削除などを実行. 変更した行数を取得できるが、データ自体は取得不能

- `fetch`: クエリを実行したうえで、データを取得可能

- `fetch`, `excute` 共に取得したレコードは Stream で返ってくる

```rust
// `try_next` を利用できるようにする
use fututes::TryStreamExt;

let mut rows = sqlx::query("SELECT * FROM users WHERE email = ?")
    .bind(email)
    .fetch(&mut conn);

while let Some(row) = rows.try_next().await? {
    let email: &str = row.try_get("email")?;
}
```

- 実際は、`fetch` そのままだと使いづらいので、`fetch_xxx` という形のメソッドが用意されている

  - `fetch_one`: 該当レコードをひとつだけ取得する
    - 返り値の型は `Result<Row, Error>`
    - レコードが見つからなかった場合は、`RowNotFound` というエラーを返す

  - `fetch_all`: 該当レコードをすべて取得する
    - 返り値の型は `Result<Vec<Row>, Error>`

  - `fetch_optional`: 該当レコードをひとつだけ取得する
    - 返り値の型は `Result<Option<Row>, Error>`
    - レコードが見つからなかった場合は、`None` を返す

## リポジトリの各メソッドの実装

