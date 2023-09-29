# sqlx で作成したリポジトリのテスト

## テストの作成

- 今回は、dokcer コンテナ内の DB をそのまま利用してテストを行う

- **`src/repositories/todos.rs`**

  ```rust
  use anyhow::Result;

  use super::*;
  use crate::pg_pool;

  #[tokio::test]
  async fn todo_crud_senario() -> Result<()> {
      let pool = pg_pool::connect_to_pg_pool().await;
      let repository = TodoRepositoryWithSqlx::new(pool.clone());

      let text = TodoText::new("todo text");
      let new_todo = Todo::new(text);
      let new_todo_id = new_todo.get_id();

      // save
      repository.save(&new_todo).await?;

      // find
      let expected = new_todo.clone();
      let todo_found = repository
          .find(new_todo_id)
          .await
          .expect("failed to find todo.");
      assert_eq!(expected, todo_found);
      assert_eq!("todo text", todo_found.get_text());
      assert_eq!(expected.get_completed(), todo_found.get_completed());

      // find_all
      let expected = new_todo.clone();
      let todos_found = repository.find_all().await?;
      assert!(todos_found
          .into_iter()
          .find(|todo| todo.clone() == expected)
          .is_some());

      // save (update)
      let mut updated_todo = new_todo.clone();
      let updated_text = TodoText::new("updated text");
      updated_todo.set_text(updated_text);
      updated_todo.set_completed(true);
      repository.save(&updated_todo).await?;

      // find
      let expected = updated_todo.clone();
      let todo_found = repository
          .find(new_todo_id)
          .await
          .expect("failed to find todo.");
      assert_eq!(expected, todo_found);
      assert_eq!("updated text", todo_found.get_text());
      assert_eq!(true, todo_found.get_completed());

      // delete
      let todo_id = new_todo_id.clone();
      repository
          .delete(new_todo)
          .await
          .expect("failed to delete todo.");

      // find
      let res = repository.find(&todo_id).await;
      assert!(res.is_err());

      Ok(())
  }
  ```

## feature フラグで DB なしのテストのみを実行する

- このままだと、上記で作成したテスト以外を実行したいときにも、DB を動かしておく必要がある
  - &rarr; DB がないと実行できないテストに feature フラグを付与する

1. **`Cargo.toml`** に以下を追記

   ```toml
   [features]
   default = ["database-test"]
   database-test = []
   ```

2. db が必要なテストモジュールに `#[cfg(feature = "database-test")]` を追記

3. &rarr; `--no-default-features` というオプションをつけて cargo を実行するとスタンドアローンで（DBなしで）実行できるテストのみを実行できるようになる. ここでは `Makefile` に以下を追記する：

   ```Makefile
   # スタンドアローンテスト
   test-s:
      cargo test --no-default-features
   ```
