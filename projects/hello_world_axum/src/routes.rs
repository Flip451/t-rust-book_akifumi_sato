pub mod root;
pub mod todo;
pub mod user;

use std::sync::Arc;

use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use root::index;
use todo::create_todo;
use user::create_user;

use crate::repository::{
    todo::{CreateTodo, Todo, UpdateTodo},
    user::{CreateUser, UpdateUser, User},
    Repository,
};

pub fn create_app<T>(repository: T) -> Router
where
    T: Repository<Todo, CreateTodo, UpdateTodo>,
    T: Repository<User, CreateUser, UpdateUser>,
{
    // ルーティング設定の作成
    // route メソッドでは
    // 第一引数で URL
    // 第二引数で、URL にマッチしたときに呼び出す関数を定義
    // 第二引数に渡す関数は、get(...) などでラップして HTTP メソッドを指定する
    // get(get_handler).post(post_handler) のように
    // メソッドチェーンで指定すれば複数のメソッドを指定できる
    Router::new()
        .route("/", get(index))
        .route("/users", post(create_user::<T>))
        .route("/todos", post(create_todo::<T>))
        .layer(Extension(Arc::new(repository)))
}
