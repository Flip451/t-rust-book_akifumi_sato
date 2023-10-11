mod label_handlers;
mod root_handlers;
mod todo_handlers;
mod user_handlers;

use std::sync::Arc;

use axum::{http::HeaderValue, routing::get, Extension, Router};
use hyper::header::CONTENT_TYPE;
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};

#[cfg(test)]
use crate::infra::repository_impl::in_memory::{
    labels::in_memory_label_repository::InMemoryLabelRepository,
    todos::in_memory_todo_repository::InMemoryTodoRepository,
    users::in_memory_user_repository::InMemoryUserRepository,
};

use crate::{
    application::{
        labels::{
            label_create_application_service::LabelCreateApplicationService,
            label_delete_application_service::LabelDeleteApplicationService,
            label_get_all_aplication_service::LabelGetAllApplicationService,
            label_get_application_service::LabelGetApplicationService,
            label_update_application_service::LabelUpdateApplicationService,
        },
        todos::{
            todo_create_application_service::TodoCreateApplicationService,
            todo_delete_application_service::TodoDeleteApplicationService,
            todo_get_all_aplication_service::TodoGetAllApplicationService,
            todo_get_application_service::TodoGetApplicationService,
            todo_update_application_service::TodoUpdateApplicationService,
        },
        users::{
            user_create_application_service::UserCreateApplicationService,
            user_delete_application_service::UserDeleteApplicationService,
            user_get_all_aplication_service::UserGetAllApplicationService,
            user_get_application_service::UserGetApplicationService,
            user_update_application_service::UserUpdateApplicationService,
        },
    },
    domain::models::{
        labels::label_repository::ILabelRepository, todos::todo_repository::ITodoRepository,
        users::user_repository::IUserRepository,
    },
    infra::repository_impl::pg::{
        pg_label_repository::PgLabelRepository, pg_todo_repository::PgTodoRepository,
        pg_user_repository::PgUserRepository,
    },
};

pub struct ArgCreateApp<LabelRep, TodoRep, UserRep>
where
    LabelRep: ILabelRepository,
    UserRep: IUserRepository,
    TodoRep: ITodoRepository,
{
    label_repository: LabelRep,
    todo_repository: TodoRep,
    user_repository: UserRep,
}

#[cfg(test)]
impl ArgCreateApp<InMemoryLabelRepository, InMemoryTodoRepository, InMemoryUserRepository> {
    pub fn new() -> Self {
        let label_repository = InMemoryLabelRepository::new();
        let todo_repository = InMemoryTodoRepository::new();
        let user_repository = InMemoryUserRepository::new();
        Self {
            label_repository,
            todo_repository,
            user_repository,
        }
    }
}

impl ArgCreateApp<PgLabelRepository, PgTodoRepository, PgUserRepository> {
    pub fn new(pg_pool: PgPool) -> Self {
        let label_repository = PgLabelRepository::new(pg_pool.clone());
        let todo_repository = PgTodoRepository::new(pg_pool.clone());
        let user_repository = PgUserRepository::new(pg_pool);
        Self {
            label_repository,
            todo_repository,
            user_repository,
        }
    }
}

pub fn create_app<LabelRep, TodoRep, UserRep>(
    ArgCreateApp {
        label_repository,
        todo_repository,
        user_repository,
    }: ArgCreateApp<LabelRep, TodoRep, UserRep>,
) -> Router
where
    LabelRep: ILabelRepository,
    UserRep: IUserRepository,
    TodoRep: ITodoRepository,
{
    Router::new()
        .route("/", get(root_handlers::index))
        // labels
        .route(
            "/labels",
            get(label_handlers::get_all::<LabelRep, LabelGetAllApplicationService<LabelRep>>)
                .post(label_handlers::create::<LabelRep, LabelCreateApplicationService<LabelRep>>),
        )
        .route(
            "/labels/:id",
            get(label_handlers::get::<LabelRep, LabelGetApplicationService<LabelRep>>)
                .patch(label_handlers::update::<LabelRep, LabelUpdateApplicationService<LabelRep>>)
                .delete(
                    label_handlers::delete::<LabelRep, LabelDeleteApplicationService<LabelRep>>,
                ),
        )
        .layer(Extension(Arc::new(label_repository)))
        // todos
        .route(
            "/todos",
            get(todo_handlers::get_all::<TodoRep, TodoGetAllApplicationService<TodoRep>>).post(
                todo_handlers::create::<
                    TodoRep,
                    LabelRep,
                    TodoCreateApplicationService<TodoRep, LabelRep>,
                >,
            ),
        )
        .route(
            "/todos/:id",
            get(todo_handlers::get::<TodoRep, TodoGetApplicationService<TodoRep>>)
                .patch(
                    todo_handlers::update::<
                        TodoRep,
                        LabelRep,
                        TodoUpdateApplicationService<TodoRep, LabelRep>,
                    >,
                )
                .delete(todo_handlers::delete::<TodoRep, TodoDeleteApplicationService<TodoRep>>),
        )
        .layer(Extension(Arc::new(todo_repository)))
        // users
        .route(
            "/users",
            get(user_handlers::get_all::<UserRep, UserGetAllApplicationService<UserRep>>)
                .post(user_handlers::create::<UserRep, UserCreateApplicationService<UserRep>>),
        )
        .route(
            "/users/:id",
            get(user_handlers::get::<UserRep, UserGetApplicationService<UserRep>>)
                .patch(user_handlers::update::<UserRep, UserUpdateApplicationService<UserRep>>)
                .delete(user_handlers::delete::<UserRep, UserDeleteApplicationService<UserRep>>),
        )
        .layer(Extension(Arc::new(user_repository)))
        // CORS
        .layer(
            CorsLayer::new()
                .allow_origin("http://127.0.0.1:3001".parse::<HeaderValue>().unwrap())
                .allow_methods(Any)
                .allow_headers(vec![CONTENT_TYPE]),
        )
}
