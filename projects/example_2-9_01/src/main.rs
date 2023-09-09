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