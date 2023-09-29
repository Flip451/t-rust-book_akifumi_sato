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

### Todo 構造体を `query_as` で利用できるように注釈を加える

- **`src/models/todos.rs`**

  ```rust
  #[derive(Clone, Debug, Deserialize, Serialize)]
  pub struct Todo {
      id: TodoId,
      text: TodoText,
      completed: bool,
  }

  impl<'r> FromRow<'r, PgRow> for Todo {
      fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
          let id = row.try_get("id")?;
          let text = TodoText::from_row(row)?;
          let completed = row.try_get("completed")?;
          Ok(Self {
              id,
              text,
              completed,
          })
      }
  }

  // --snip--

  #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
  pub struct TodoText {
      #[validate(length(min = 1, message = "Can not be empty"))]
      #[validate(length(max = 100, message = "Over text length"))]
      value: String,
  }

  impl<'r> FromRow<'r, PgRow> for TodoText {
      fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
          let text = row.try_get("text")?;
          Ok(TodoText { value: text })
      }
  }
  ```

### `RepositoryError` の修正

- `TodoRepositoryError` に、sqlx の実行時エラーを表す要素を追加する

  **`src/repositories/todos.rs`**

  ```rust
  #[derive(Error, Debug, PartialEq)]
  pub enum TodoRepositoryError {
      #[error("NotFound, id is {0}")]
      NotFound(TodoId),
      #[error("Unexpected Error: [{0}]")]
      Unexpected(String),
  }
  ```

### `save` メソッド

- 実行する sql 文
  - <https://resanaplaza.com/2023/01/29/%E3%80%90%E5%AE%9F%E7%94%A8%E3%80%91postgresql%E3%81%A7%E4%BD%BF%E3%81%86upsert%E3%81%AE%E6%9B%B8%E3%81%8D%E6%96%B9%E3%81%A8%E6%B3%A8%E6%84%8F%E7%82%B9/> を参考に実装

  ```sql
  insert into todos (id, text, completed)
  values ($1, $2, $3)
  on conflict (id)
  do update set text=$2, completed=$3;
  ```

- **`src/repositories/todos.rs`**

  ```rust
  async fn save(&self, todo: &Todo) -> Result<()> {
      let sql = r#"
  insert into todos (id, text, completed)
  values ($1, $2, $3)
  on conflict (id)
  do update set text=$2, completed=$3
  "#;
      sqlx::query(sql)
          .bind(todo.get_id())
          .bind(todo.get_text())
          .bind(todo.get_completed())
          .execute(&self.pool)
          .await
          .map_err(|e| TodoRepositoryError::Unexpected(e.to_string()))?;
      Ok(())
  }
  ```

### `find` メソッド

- 実行する sql 文

  ```sql
  select * from todos wehre id=$1;
  ```

- **`src/repositories/todos.rs`**

  ```rust
  async fn find(&self, todo_id: &TodoId) -> Result<Todo> {
      let sql = r#"select * from todos where id=$1"#;
      let todo = sqlx::query_as::<_, Todo>(sql)
          .bind(todo_id)
          .fetch_one(&self.pool)
          .await
          .map_err(|e| match e {
              sqlx::Error::RowNotFound => TodoRepositoryError::NotFound(todo_id.clone()),
              _ => TodoRepositoryError::Unexpected(e.to_string()),
          })?;
      Ok(todo)
  }
  ```

### `find_all` メソッド

- 実行する sql 文

  ```sql
  select * from todos order by id desc;
  ```

- **`src/repositories/todos.rs`**

  ```rust
  async fn find_all(&self) -> Result<Vec<Todo>> {
      let sql = r#"select * from todos order by id desc"#;
      let todo = sqlx::query_as::<_, Todo>(sql)
          .fetch_all(&self.pool)
          .await
          .map_err(|e| TodoRepositoryError::Unexpected(e.to_string()))?;
      Ok(todo)
  }
  ```

### `delete` メソッド

- 実行する sql 文

  ```sql
  delete from todos where id=$1;
  ```

- **`src/repositories/todos.rs`**

  ```rust
  async fn delete(&self, todo: Todo) -> Result<()> {
      let id = todo.get_id();
      let sql = r#"delete from todos where id=$1"#;
      sqlx::query(sql)
          .bind(id)
          .execute(&self.pool)
          .await
          .map_err(|e| match e {
              sqlx::Error::RowNotFound => {
                  TodoRepositoryError::NotFound(id.clone())
              }
              _ => TodoRepositoryError::Unexpected(e.to_string()),
          })?;
      Ok(())
  }
  ```
