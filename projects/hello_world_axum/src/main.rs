use std::net::SocketAddr;

use axum::{Router, routing::get};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()>{
    // ルーティング設定の作成
    // 第一引数で URL
    // 第二引数で、URL にマッチしたときに呼び出す関数を定義
    // 第二引数に渡す関数は、get(...) などでラップして HTTP メソッドを指定する
    // get(get_handler).post(post_handler) のように
    // メソッドチェーンで指定すれば複数のメソッドを指定できる
    let app =  Router::new().route("/", get(root));
    
    // アドレスとポートの作成
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // bind でアドレスとポートのバインディング（TCP コネクションの受け付け）
    // serve でサーバーを立ち上げ
    // 非同期関数なので .await で実行
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

// GET "/" で返却する値を定義する関数 
async fn root() -> &'static str {
    "Hello, world!"
}
