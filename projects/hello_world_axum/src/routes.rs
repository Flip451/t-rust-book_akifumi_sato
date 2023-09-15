pub mod root;
pub mod users;

use axum::{
    routing::{get, post},
    Router,
};
use root::index;
use users::create_user;

pub fn create_app() -> Router {
    // ルーティング設定の作成
    // route メソッドでは
    // 第一引数で URL
    // 第二引数で、URL にマッチしたときに呼び出す関数を定義
    // 第二引数に渡す関数は、get(...) などでラップして HTTP メソッドを指定する
    // get(get_handler).post(post_handler) のように
    // メソッドチェーンで指定すれば複数のメソッドを指定できる
    Router::new()
        .route("/", get(index))
        .route("/users", post(create_user))
}
