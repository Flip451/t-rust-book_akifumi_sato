use std::sync::Arc;

use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
    Json,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    application::labels::{
        label_application_error::LabelApplicationError,
        label_create_application_service::{ILabelCreateApplicationService, LabelCreateCommand},
        label_data::LabelData,
        label_delete_application_service::{ILabelDeleteApplicationService, LabelDeleteCommand},
        label_get_all_aplication_service::{ILabelGetAllApplicationService, LabelGetAllCommand},
        label_get_application_service::{ILabelGetApplicationService, LabelGetCommand},
        label_update_application_service::{ILabelUpdateApplicationService, LabelUpdateCommand},
    },
    infra::repository::labels::ILabelRepository,
};

#[derive(Serialize)]
pub struct LabelResponse {
    id: String,
    name: String,
}

impl LabelResponse {
    fn new(label_data: LabelData) -> Self {
        Self {
            id: label_data.label_id.to_string(),
            name: label_data.label_name,
        }
    }
}

#[derive(Deserialize)]
pub struct LabelCreatePayload {
    name: String,
}

impl LabelCreatePayload {
    fn into_command(self) -> LabelCreateCommand {
        LabelCreateCommand {
            label_name: self.name,
        }
    }
}

#[derive(Deserialize)]
pub struct LabelUpdatePayload {
    name: Option<String>,
}

impl LabelUpdatePayload {
    fn into_command(self, id: String) -> LabelUpdateCommand {
        LabelUpdateCommand {
            label_id: id,
            label_name: self.name,
        }
    }
}

pub async fn create<Rep, AS>(
    Extension(repository): Extension<Arc<Rep>>,
    Json(payload): Json<LabelCreatePayload>,
) -> Result<impl IntoResponse, impl IntoResponse>
where
    Rep: ILabelRepository,
    AS: ILabelCreateApplicationService<Rep>,
{
    let label_create_application_service = AS::new(repository);

    match label_create_application_service
        .handle(payload.into_command())
        .await
    {
        Ok(label_data) => Ok((StatusCode::CREATED, Json(LabelResponse::new(label_data)))),
        Err(e @ LabelApplicationError::DuplicatedLabel(_)) => {
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
        Err(e @ LabelApplicationError::IllegalArgumentError(_)) => {
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
        Err(e @ LabelApplicationError::IllegalLabelId(_)) => {
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
        Err(e @ LabelApplicationError::LabelNotFound(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(e @ LabelApplicationError::Unexpected(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}

pub async fn get<Rep, AS>(
    Extension(repository): Extension<Arc<Rep>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, impl IntoResponse>
where
    Rep: ILabelRepository,
    AS: ILabelGetApplicationService<Rep>,
{
    let label_get_application_service = AS::new(repository);

    match label_get_application_service
        .handle(LabelGetCommand { label_id: id })
        .await
    {
        Ok(label_data) => Ok((StatusCode::OK, Json(LabelResponse::new(label_data)))),
        Err(e @ LabelApplicationError::DuplicatedLabel(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(e @ LabelApplicationError::IllegalArgumentError(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(e @ LabelApplicationError::IllegalLabelId(_)) => {
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
        Err(e @ LabelApplicationError::LabelNotFound(_)) => {
            Err((StatusCode::NOT_FOUND, e.to_string()))
        }
        Err(e @ LabelApplicationError::Unexpected(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}

pub async fn get_all<Rep, AS>(
    Extension(repository): Extension<Arc<Rep>>,
) -> Result<impl IntoResponse, impl IntoResponse>
where
    Rep: ILabelRepository,
    AS: ILabelGetAllApplicationService<Rep>,
{
    let label_get_all_application_service = AS::new(repository);

    match label_get_all_application_service
        .handle(LabelGetAllCommand {})
        .await
    {
        Ok(label_data) => Ok((
            StatusCode::OK,
            Json(
                label_data
                    .into_iter()
                    .map(|label_data| LabelResponse::new(label_data))
                    .collect::<Vec<LabelResponse>>(),
            ),
        )),
        Err(e @ LabelApplicationError::DuplicatedLabel(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(e @ LabelApplicationError::IllegalArgumentError(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(e @ LabelApplicationError::IllegalLabelId(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(e @ LabelApplicationError::LabelNotFound(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(e @ LabelApplicationError::Unexpected(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}

pub async fn update<Rep, AS>(
    Extension(repository): Extension<Arc<Rep>>,
    Path(id): Path<String>,
    Json(payload): Json<LabelUpdatePayload>,
) -> Result<impl IntoResponse, impl IntoResponse>
where
    Rep: ILabelRepository,
    AS: ILabelUpdateApplicationService<Rep>,
{
    let label_update_application_service = AS::new(repository);

    match label_update_application_service
        .handle(payload.into_command(id))
        .await
    {
        Ok(label_data) => Ok((StatusCode::OK, Json(LabelResponse::new(label_data)))),
        Err(e @ LabelApplicationError::DuplicatedLabel(_)) => {
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
        Err(e @ LabelApplicationError::IllegalArgumentError(_)) => {
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
        Err(e @ LabelApplicationError::IllegalLabelId(_)) => {
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
        Err(e @ LabelApplicationError::LabelNotFound(_)) => {
            Err((StatusCode::NOT_FOUND, e.to_string()))
        }
        Err(e @ LabelApplicationError::Unexpected(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}

pub async fn delete<Rep, AS>(
    Extension(repository): Extension<Arc<Rep>>,
    Path(id): Path<String>,
) -> Result<StatusCode, impl IntoResponse>
where
    Rep: ILabelRepository,
    AS: ILabelDeleteApplicationService<Rep>,
{
    let label_delete_application_service = AS::new(repository);

    match label_delete_application_service
        .handle(LabelDeleteCommand { label_id: id })
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e @ LabelApplicationError::DuplicatedLabel(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(e @ LabelApplicationError::IllegalArgumentError(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(e @ LabelApplicationError::IllegalLabelId(_)) => {
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
        Err(e @ LabelApplicationError::LabelNotFound(_)) => {
            Err((StatusCode::NOT_FOUND, e.to_string()))
        }
        Err(e @ LabelApplicationError::Unexpected(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}
