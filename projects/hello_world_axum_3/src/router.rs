mod root_handlers;
mod user_handlers;

use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{
    application::users::{
        user_create_application_service::UserCreateApplicationService,
        user_delete_application_service::UserDeleteApplicationService,
        user_get_all_aplication_service::UserGetAllApplicationService,
        user_get_application_service::UserGetApplicationService,
        user_update_application_service::UserUpdateApplicationService,
    },
    infra::repository::users::IUserRepository,
};

pub fn create_app<UserRep>(user_repository: UserRep) -> Router
where
    UserRep: IUserRepository,
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
        .with_state(Arc::new(user_repository))
}
