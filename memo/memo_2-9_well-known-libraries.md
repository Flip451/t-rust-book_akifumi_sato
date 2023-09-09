# よく使われるライブラリ

## 目次

- [よく使われるライブラリ](#よく使われるライブラリ)
  - [目次](#目次)
  - [`anyhow` / `thiserror`](#anyhow--thiserror)
  - [JSON を扱う Serde](#json-を扱う-serde)

## `anyhow` / `thiserror`

- 例えば、以下のようなコードを考える：

  ```rust
  use core::fmt;
  use std::{error, fs::File};

  // カスタムの API エラーを自作
  // あとで error::Error トレイトを実装して、エラーとして扱えるようにする
  // （Debug トレイトは error::Error の実装に必要）
  #[derive(Debug)]
  enum ApiError {
      InternalServerError(String),
      NotFound,
  }

  // API エラー列挙子に Display トレイトを実装（error::Error の実装に必要）
  impl fmt::Display for ApiError {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
          write!(f, "[ApiError]")
      }
  }

  // API エラー列挙子に error::Error を実装
  impl error::Error for ApiError {}

  // データを取得する関数
  // サーバー内部エラーを起こすかもしれない関数
  fn fetch_api() -> Result<(), ApiError> {
      Err(ApiError::InternalServerError("[always_my_error]".to_string()))
  }

  // エラーを起こすかもしれない関数
  // 発生しうるエラーは、ApiError か std::io::Error のどちらでもありうるので
  // 返り値の Result 列挙子内の第２型引数は error::Error のトレイトオブジェクトとしている
  fn maybe_fail() -> Result<(), Box<dyn error::Error>> {
      let _r = fetch_api()?;  // ApiError
      let _f = File::open("hoge.txt")?;  //  std::io::Error
      Ok(())
  }

  fn main() -> Result<(), Box<dyn error::Error>> {
      // 失敗するかもしれない関数を実行
      let _l = maybe_fail()?;
      Ok(())
  }
  ```

  - なお、`?`, `#[derive()]` などについては the book を参照せよ

  - 上記のコードでは、自作のエラー列挙子 `ApiError` を定義して、その列挙子に `error::Error` トレイトを実装し、`Result` 列挙子の第２型引数に渡せるようにしている
    - これにより `?` 演算子と組み合わせて使えるようになっている

  - しかし、このコード内で行っている `ApiError` への `error::Error` トレイトの実装や、`?` 演算子を使う際の返り値の `-> Result<..., Box<dyn error::Error>>` の記述を毎回するのは面倒
    - &rarr; `anyhow` と `thiserror` を導入するとこれらをシンプルにできる

- 以下のコードは `anyhow` と `thiserror` を導入して上記のコードをより簡略にしたもの

  ```rust
  use std::fs::File;

  use anyhow::{Context, Result};
  use thiserror::Error;

  // 自作のエラー列挙子を定義するには `thiserror::Error` マクロを利用して derive すればよい
  // エラーメッセージを定義するには、各要素を #[error("メッセージのフォーマット文")] で修飾すればよい
  #[derive(Error, Debug)]
  enum ApiError {
      #[error("InternalServerError: {0}")]
      InternalServerError(String),
      #[error("NotFound")]
      NotFound,
  }

  // 返り値の型を Result<T, Box<dyn error::Error>> に設定する代わりに
  // anyhow::Result<T> を返り値の型に設定すればよい
  // ただし、関数の内部でエラーを返す時は .into() して型を矯正する必要がある
  // 型の矯正は std::error::Error を実装しているエラー型を anyhow::Error に変換する形で行われる
  fn fetch_api() -> Result<()> {
      Err(ApiError::InternalServerError("[always_my_error]".to_string()).into())
  }

  // .into() の代わりに .context() を呼び出すと
  // std::error::Errorを実装しているエラー型に新たにエラーを追加しつつ
  // anyhow::Errorに変換することができる
  fn maybe_fail() -> Result<()> {
      let _r = fetch_api()?;
      let filename = "hoge.txt";
      let _l = File::open(filename).context(format!("failed to open file: {}", filename))?;
      Ok(())
  }

  fn main() -> Result<()> {
      let _l = maybe_fail()?;
      Ok(())
  }
  ```

- `anyhow` の詳細は <https://zenn.dev/yukinarit/articles/b39cd42820f29e> が詳しい

## JSON を扱う Serde

- `Serde` は JSON/YAML/TOML を扱うのに便利
- シリアライズ・デシリアライズして Rust の構造体として扱えるようにしてくれる
- 使用するには `cargo add serde --features="derive"` すればよい
- また、`json` を扱う場合は別途 `serde_json` クレートを導入する

  ```rust
  use anyhow::{Ok, Result};
  use serde::{Deserialize, Serialize};

  #[derive(Serialize, Deserialize, Debug)]
  struct User {
      name: String,
      age: u32,
  }

  fn main() -> Result<()> {
      let user = User {
          name: String::from("sato"),
          age: 30,
      };

      // `User` を JSON 文字列に変換
      let serialized = serde_json::to_string(&user)?;
      println!("serialized = {}", serialized);

      // JSON 文字列を `User` に変換
      let deserialized: User = serde_json::from_str(&serialized)?;
      println!("{:?}", deserialized);

      Ok(())
  }
  ```