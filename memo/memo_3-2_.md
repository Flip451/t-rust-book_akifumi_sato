# 環境構築

## パッケージのインストール

```sh
cargo add hyper --features full
cargo add tokio --features full
cargo add serde --features derive
cargo add tracing-subscriber --features env-filter
cargo add axum tower mime serde_json tracing anyhow thiserror
```

## hello world

- axum の一番簡単な例：

  **`src/main.rs`**

  ```rust
  use anyhow::Result;
  use axum::{routing::get, Router};
  use std::net::SocketAddr;

  #[tokio::main]
  async fn main() -> Result<()> {
      // ルーティング設定の作成
      //  route メソッドでは
      //    - 第一引数で URL
      //    - 第二引数で、URL にマッチしたときに呼び出す関数を定義
      //      - 第二引数に渡す関数は、get(...) などでラップして HTTP メソッドを指定する
      //      - get(get_handler).post(post_handler) のように
      //        メソッドチェーンで指定すれば一つの URL に対する複数のメソッドでの動作を設定できる
      let app = Router::new().route("/", get(root));

      // アドレスの作成
      let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

      // bind でアドレスとポートのバインディング（TCP コネクションの受け付け）
      // serve でサーバーを立ち上げ
      // 非同期関数なので .await で実行
      axum::Server::bind(&addr)
          .serve(app.into_make_service())
          .await?;

      Ok(())
  }

  async fn root() -> &'static str {
      "Hello, world!"
  }
  ```

- サーバの起動

  ```sh
  cargo run
  ```

  &rarr; <http://127.0.0.1:3000/> にアクセスすると `Hello, world!` と表示される

## logging の導入

- `tracing` を導入してロギングを導入

  ```diff
  use anyhow::Result;
  use axum::{routing::get, Router};
  - use std::net::SocketAddr;
  + use std::{net::SocketAddr, env};

  #[tokio::main]
  async fn main() -> Result<()> {
  +   // logging の初期化
  +   let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
  +   env::set_var("RUST_LOG", log_level);
  +   tracing_subscriber::fmt::init();

      let app = Router::new().route("/", get(root));
      let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

  +   // ログにアドレスを表示
  +   tracing::debug!("listening on {}", addr);

      axum::Server::bind(&addr)
          .serve(app.into_make_service())
          .await?;

      Ok(())
  }

  async fn root() -> &'static str {
      "Hello, world!"
  }
  ```

- ログを表示するように環境変数を設定してサーバを起動

  ```sh
  RUST_LOG=debug cargo run
  ```

## ファイル分割

- `main` 関数が肥大化するのを避けるためにファイルを分割
  - ディレクトリ構成

    ```sh
    .
    ├── Cargo.lock
    ├── Cargo.toml
    └── src
        ├── lib.rs
        ├── logs.rs
        ├── main.rs
        ├── routes
        │   └── root.rs
        └── routes.rs
    ```

  - **`main.rs`**

    ```rust
    use anyhow::Result;
    use hello_world_axum_2::{logs::init_log, routes::create_app};
    use std::net::SocketAddr;

    #[tokio::main]
    async fn main() -> Result<()> {
        // logging の初期化
        init_log();

        // ルーティングの設定
        let app = create_app();

        // アドレスを作成
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

        // ログにアドレスを表示
        tracing::debug!("listening on {}", addr);

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
    ```

  - **`lib.rs`**

    ```rust
    pub mod routes;
    pub mod logs;
    ```

  - **`routes.rs`**

    ```rust
    mod root;

    use axum::{routing::get, Router};

    pub fn create_app() -> Router {
        Router::new().route("/", get(root::index))
    }
    ```

  - **`routes/root.rs`**

    ```rust
    pub async fn index() -> &'static str {
        "Hello, world!"
    }
    ```

  - **`logs.rs`**

    ```rust
    use std::env;

    pub fn init_log() {
        let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
        env::set_var("RUST_LOG", log_level);
        tracing_subscriber::fmt::init();
    }
    ```

## POST リクエストと JSON レスポンス

- POST メソッドで JSON を受け取って、レスポンスでも JSON を返却する

- ルーティング設定に `/users` への POST メソッドを追加
  **`src/routes.rs`**

  ```diff
  mod root;
  + mod users;

  use axum::{
  -   routing::get,
  +   routing::{get, post},
      Router,
  };

  pub fn create_app() -> Router {
      Router::new()
          .route("/", get(root::index))
  +       .route("/users", post(users::create))
  }
  ```

- `/users` に POST メソッドで JSON が送られてきた際の処理を定義する関数本体を実装

  **`src/routes/users.rs`**

  ```rust
  use axum::{http::StatusCode, response::IntoResponse, Json};
  use serde::{Deserialize, Serialize};

  // serde を利用して User 構造体を JSON にシリアライズできるようにする
  #[derive(Serialize)]
  struct User {
      id: i32,
      username: String,
  }

  // serde を利用して CreateUser 構造体を JSON からデシリアライズできるようにする
  #[derive(Deserialize)]
  pub struct CreateUser {
      username: String,
  }

  // CreateUser と同じ構造の JSON が送られてきたら
  // User と同じ構造の JSON を送り返すメソッドを定義
  // `src/routes.rs` で利用できるように返り値の型には、`IntoResponse` を実装する型を指定
  // (StatusCode, Json<T>) はこれを実装する型の一つ
  pub async fn create(Json(payload): Json<CreateUser>) -> impl IntoResponse {
      let user = User {
          id: 1337,
          username: payload.username,
      };
      (StatusCode::CREATED, Json(user))
  }
  ```

- 動作を確認
  - <http://127.0.0.1:3000/users> に以下の JSON を POST する

    ```json
    {
        "username": "佐藤太郎"
    }
    ```

    - 注意：この際、ヘッダーに `Content-Type: application/json` を設定すること

  - すると、以下のレスポンスが返ってくる

    ```json
    {
        "id": 1337,
        "username": "佐藤太郎"
    }
    ```
