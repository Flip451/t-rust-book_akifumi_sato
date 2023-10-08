use std::sync::Arc;

use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
    Json,
};
use hyper::StatusCode;
use serde::Deserialize;

use crate::{
    application::users::{
        user_application_error::UserApplicationError,
        user_create_application_service::{IUserCreateApplicationService, UserCreateCommand},
        user_delete_application_service::{IUserDeleteApplicationService, UserDeleteCommand},
        user_get_all_aplication_service::{IUserGetAllApplicationService, UserGetAllCommand},
        user_get_application_service::{IUserGetApplicationService, UserGetCommand},
        user_update_application_service::{IUserUpdateApplicationService, UserUpdateCommand},
    },
    infra::repository::users::IUserRepository,
};

#[derive(Deserialize)]
pub struct UserCreatePayload {
    user_name: String,
}

impl UserCreatePayload {
    fn into_command(self) -> UserCreateCommand {
        UserCreateCommand {
            user_name: self.user_name,
        }
    }
}

#[derive(Deserialize)]
pub struct UserUpdatePayload {
    user_name: Option<String>,
}

impl UserUpdatePayload {
    fn into_command(self, id: String) -> UserUpdateCommand {
        UserUpdateCommand {
            user_id: id,
            user_name: self.user_name,
        }
    }
}

pub async fn create<Rep, AS>(
    Extension(repository): Extension<Arc<Rep>>,
    Json(payload): Json<UserCreatePayload>,
) -> Result<impl IntoResponse, impl IntoResponse>
where
    Rep: IUserRepository,
    AS: IUserCreateApplicationService<Rep>,
{
    let user_create_application_service = AS::new(repository);

    match user_create_application_service
        .handle(payload.into_command())
        .await
    {
        Ok(user_data) => Ok((StatusCode::CREATED, Json(user_data))),
        Err(e @ UserApplicationError::DuplicatedUser(_)) => {
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
        Err(e @ UserApplicationError::IllegalArgumentError(_)) => {
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
        Err(e @ UserApplicationError::IllegalUserId(_)) => {
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
        Err(e @ UserApplicationError::UserNotFound(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(e @ UserApplicationError::Unexpected(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}

pub async fn get<Rep, AS>(
    Extension(repository): Extension<Arc<Rep>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, impl IntoResponse>
where
    Rep: IUserRepository,
    AS: IUserGetApplicationService<Rep>,
{
    let user_get_application_service = AS::new(repository);

    match user_get_application_service
        .handle(UserGetCommand { user_id: id })
        .await
    {
        Ok(user_data) => Ok((StatusCode::OK, Json(user_data))),
        Err(e @ UserApplicationError::DuplicatedUser(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(e @ UserApplicationError::IllegalArgumentError(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(e @ UserApplicationError::IllegalUserId(_)) => {
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
        Err(e @ UserApplicationError::UserNotFound(_)) => {
            Err((StatusCode::NOT_FOUND, e.to_string()))
        }
        Err(e @ UserApplicationError::Unexpected(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}

pub async fn get_all<Rep, AS>(
    Extension(repository): Extension<Arc<Rep>>,
) -> Result<impl IntoResponse, impl IntoResponse>
where
    Rep: IUserRepository,
    AS: IUserGetAllApplicationService<Rep>,
{
    let user_get_all_application_service = AS::new(repository);

    match user_get_all_application_service
        .handle(UserGetAllCommand {})
        .await
    {
        Ok(user_data) => Ok((StatusCode::OK, Json(user_data))),
        Err(e @ UserApplicationError::DuplicatedUser(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(e @ UserApplicationError::IllegalArgumentError(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(e @ UserApplicationError::IllegalUserId(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(e @ UserApplicationError::UserNotFound(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(e @ UserApplicationError::Unexpected(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}

pub async fn update<Rep, AS>(
    Extension(repository): Extension<Arc<Rep>>,
    Path(id): Path<String>,
    Json(payload): Json<UserUpdatePayload>,
) -> Result<impl IntoResponse, impl IntoResponse>
where
    Rep: IUserRepository,
    AS: IUserUpdateApplicationService<Rep>,
{
    let user_update_application_service = AS::new(repository);

    match user_update_application_service
        .handle(payload.into_command(id))
        .await
    {
        Ok(user_data) => Ok((StatusCode::OK, Json(user_data))),
        Err(e @ UserApplicationError::DuplicatedUser(_)) => {
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
        Err(e @ UserApplicationError::IllegalArgumentError(_)) => {
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
        Err(e @ UserApplicationError::IllegalUserId(_)) => {
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
        Err(e @ UserApplicationError::UserNotFound(_)) => {
            Err((StatusCode::NOT_FOUND, e.to_string()))
        }
        Err(e @ UserApplicationError::Unexpected(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}

pub async fn delete<Rep, AS>(
    Extension(repository): Extension<Arc<Rep>>,
    Path(id): Path<String>,
) -> Result<StatusCode, impl IntoResponse>
where
    Rep: IUserRepository,
    AS: IUserDeleteApplicationService<Rep>,
{
    let user_delete_application_service = AS::new(repository);

    match user_delete_application_service
        .handle(UserDeleteCommand { user_id: id })
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e @ UserApplicationError::DuplicatedUser(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(e @ UserApplicationError::IllegalArgumentError(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(e @ UserApplicationError::IllegalUserId(_)) => {
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
        Err(e @ UserApplicationError::UserNotFound(_)) => {
            Err((StatusCode::NOT_FOUND, e.to_string()))
        }
        Err(e @ UserApplicationError::Unexpected(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}
