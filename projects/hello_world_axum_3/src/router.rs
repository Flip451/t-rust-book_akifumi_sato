mod root_handlers;
mod todo_handlers;
mod user_handlers;

use std::sync::Arc;

use axum::{routing::get, Extension, Router};

use crate::{
    application::{
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
    infra::{
        repository::{todos::ITodoRepository, users::IUserRepository},
        repository_impl::in_memory::{
            todos::in_memory_todo_repository::InMemoryTodoRepository,
            users::in_memory_user_repository::InMemoryUserRepository,
        },
    },
};

pub struct ArgCreateApp<TodoRep, UserRep>
where
    UserRep: IUserRepository,
    TodoRep: ITodoRepository,
{
    todo_repository: TodoRep,
    user_repository: UserRep,
}

impl ArgCreateApp<InMemoryTodoRepository, InMemoryUserRepository>
{
    pub fn new() -> Self {
        let todo_repository = InMemoryTodoRepository::new();
        let user_repository = InMemoryUserRepository::new();
        Self {
            todo_repository,
            user_repository,
        }
    }
}

pub fn create_app<TodoRep, UserRep>(
    ArgCreateApp {
        todo_repository,
        user_repository,
    }: ArgCreateApp<TodoRep, UserRep>,
) -> Router
where
    UserRep: IUserRepository,
    TodoRep: ITodoRepository,
{
    Router::new()
        .route("/", get(root_handlers::index))
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
        .route(
            "/todos",
            get(todo_handlers::get_all::<TodoRep, TodoGetAllApplicationService<TodoRep>>)
                .post(todo_handlers::create::<TodoRep, TodoCreateApplicationService<TodoRep>>),
        )
        .route(
            "/todos/:id",
            get(todo_handlers::get::<TodoRep, TodoGetApplicationService<TodoRep>>)
                .patch(todo_handlers::update::<TodoRep, TodoUpdateApplicationService<TodoRep>>)
                .delete(todo_handlers::delete::<TodoRep, TodoDeleteApplicationService<TodoRep>>),
        )
        .layer(Extension(Arc::new(todo_repository)))
}
