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
