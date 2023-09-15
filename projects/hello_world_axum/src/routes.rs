pub mod root;
pub mod todo;
pub mod user;

use std::sync::Arc;

use axum::{extract::Extension, routing::get, Router};

use crate::repository::{
    todo::{CreateTodo, Todo, UpdateTodo},
    user::{CreateUser, UpdateUser, User},
    Repository,
};

use self::root::index;
use self::todo::{all_todo, create_todo, delete_todo, find_todo, update_todo};
use self::user::{all_user, create_user, delete_user, find_user, update_user};

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
        .route("/users", get(all_user::<T>).post(create_user::<T>))
        .route(
            "/users/:id",
            get(find_user::<T>)
                .patch(update_user::<T>)
                .delete(delete_user::<T>),
        )
        .route("/todos", get(all_todo::<T>).post(create_todo::<T>))
        .route(
            "/todos/:id",
            get(find_todo::<T>)
                .patch(update_todo::<T>)
                .delete(delete_todo::<T>),
        )
        .layer(Extension(Arc::new(repository)))
}
