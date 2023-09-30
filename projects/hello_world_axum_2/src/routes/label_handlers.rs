use std::sync::Arc;

use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    models::labels::{Label, LabelDomainService, LabelId, LabelName},
    repositories::{labels::ILabelRepository, RepositoryError},
};

use super::validator::ValidatedJson;

#[derive(Serialize, Clone, Debug, Deserialize, Validate)]
pub struct CreateLabel {
    #[validate]
    pub name: LabelName,
}

#[derive(Serialize, Clone, Debug, Deserialize, Validate)]
pub struct UpdateLabel {
    #[validate]
    name: Option<LabelName>,
}

pub async fn create<T>(
    State(repository): State<Arc<T>>,
    ValidatedJson(payload): ValidatedJson<CreateLabel>,
) -> Result<impl IntoResponse, StatusCode>
where
    T: ILabelRepository,
{
    let name = payload.name;
    let label = Label::new(name);
    let label_service = LabelDomainService::new(repository.clone());
    let is_duplicated = label_service
        .is_duplicated(&label)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;
    if is_duplicated {
        return Err(StatusCode::BAD_REQUEST);
    }
    repository
        .save(&label)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok((StatusCode::CREATED, Json(label)))
}

pub async fn find<T>(
    State(repository): State<Arc<T>>,
    Path(id): Path<LabelId>,
) -> Result<impl IntoResponse, StatusCode>
where
    T: ILabelRepository,
{
    match repository.find(&id).await {
        Ok(label) => Ok((StatusCode::OK, Json(label))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn all<T>(State(repository): State<Arc<T>>) -> Result<impl IntoResponse, StatusCode>
where
    T: ILabelRepository,
{
    let labels = repository
        .find_all()
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok((StatusCode::OK, Json(labels)))
}

pub async fn update<T>(
    State(repository): State<Arc<T>>,
    Path(id): Path<LabelId>,
    ValidatedJson(payload): ValidatedJson<UpdateLabel>,
) -> Result<impl IntoResponse, StatusCode>
where
    T: ILabelRepository,
{
    let mut label = repository.find(&id).await.or(Err(StatusCode::NOT_FOUND))?;
    let UpdateLabel { name: new_name } = payload;
    if let Some(new_name) = new_name {
        label.set_name(new_name);
    }

    let label_service = LabelDomainService::new(repository.clone());
    let is_duplicated = label_service
        .is_duplicated(&label)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;
    if is_duplicated {
        return Err(StatusCode::BAD_REQUEST);
    }

    repository
        .save(&label)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok((StatusCode::OK, Json(label)))
}

pub async fn delete<T>(State(repository): State<Arc<T>>, Path(id): Path<LabelId>) -> StatusCode
where
    T: ILabelRepository,
{
    match repository.find(&id).await {
        Ok(label) => {
            if let Ok(_) = repository.delete(label).await {
                StatusCode::NO_CONTENT
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
        // <https://users.rust-lang.org/t/kind-method-not-found-when-using-anyhow-and-thiserror/81560> を参考に実装
        Err(error) if error.downcast_ref() == Some(&RepositoryError::NotFound(id)) => {
            StatusCode::NOT_FOUND
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::{
        repositories::labels::in_memory_label_repository::InMemoryLabelRepository, routes::tests,
    };

    use axum::{http::method::Method, routing::get, Router};
    use tower::ServiceExt;

    pub fn create_app<T>(repository: T) -> Router
    where
        T: ILabelRepository,
    {
        Router::new()
            .route("/labels", get(all::<T>).post(create::<T>))
            .route(
                "/labels/:id",
                get(find::<T>).patch(update::<T>).delete(delete::<T>),
            )
            .with_state(Arc::new(repository))
    }

    #[tokio::test]
    async fn should_create_label() -> Result<()> {
        let label_repository = InMemoryLabelRepository::new();
        let req_body = r#"{"name": {"value": "label-1"}}"#.to_string();

        let req = tests::build_req_with_json("/labels", Method::POST, req_body)?;
        let res = create_app(label_repository).oneshot(req).await?;
        let res_body: Label = tests::res_to_struct(res).await?;

        let name_in_res = res_body.get_name();

        assert_eq!("label-1", name_in_res.to_string());
        Ok(())
    }

    #[tokio::test]
    async fn should_find_label() -> Result<()> {
        // リポジトリの作成
        let label_repository = InMemoryLabelRepository::new();

        // リポジトリに直接 Label を作成
        let label_saved_to_repository = Label::new(LabelName::new("find-label"));
        label_repository.save(&label_saved_to_repository).await?;
        let label_id_in_repository = label_saved_to_repository.get_id();

        // リクエストの作成とレスポンスの受信
        let req = tests::build_req_with_empty(
            &format!("/labels/{}", label_id_in_repository),
            Method::GET,
        )?;
        let res = create_app(label_repository).oneshot(req).await?;

        // レスポンスボディを読み込んで Label としてパース
        let res_body: Label = tests::res_to_struct(res).await?;

        let name_in_res = res_body.get_name();

        assert_eq!(label_saved_to_repository, res_body);
        assert_eq!("find-label", name_in_res.to_string());

        Ok(())
    }

    #[tokio::test]
    async fn should_get_all_label() -> Result<()> {
        // リポジトリの作成
        let label_repository = InMemoryLabelRepository::new();

        // リポジトリに直接 Label を作成しつつ
        // リポジトリ内の Label の集合を作成
        let mut labels_in_repository = HashMap::new();

        let label_saved_to_repository = Label::new(LabelName::new("get label-1"));
        label_repository.save(&label_saved_to_repository).await?;
        labels_in_repository.insert(
            label_saved_to_repository.get_id().clone(),
            label_saved_to_repository,
        );

        let label_saved_to_repository = Label::new(LabelName::new("get label-2"));
        label_repository.save(&label_saved_to_repository).await?;
        labels_in_repository.insert(
            label_saved_to_repository.get_id().clone(),
            label_saved_to_repository,
        );

        let label_saved_to_repository = Label::new(LabelName::new("get label-3"));
        label_repository.save(&label_saved_to_repository).await?;
        labels_in_repository.insert(
            label_saved_to_repository.get_id().clone(),
            label_saved_to_repository,
        );

        // リクエストの作成とレスポンスの受信
        let req = tests::build_req_with_empty("/labels", Method::GET)?;
        let res = create_app(label_repository).oneshot(req).await?;

        // レスポンスボディを読み込んで Vec<Label> としてパース
        let res_body: Vec<Label> = tests::res_to_struct(res).await?;

        // リポジトリ内の Label の集合とレスポンスで返ってきた Label の集合を比較
        let labels_in_res = res_body
            .iter()
            .map(|label| (label.get_id().clone(), label.clone()))
            .collect();

        // 両者の id の集合が一致していることを確認
        assert_eq!(labels_in_repository, labels_in_res);

        // 両者の内容が一致していることを確認
        for (id, label_in_rep) in labels_in_repository {
            match labels_in_res.get(&id) {
                Some(label_in_res) => {
                    assert_eq!(
                        label_in_rep.get_name().to_string(),
                        label_in_res.get_name().to_string()
                    );
                }
                None => panic!(),
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn should_update_label() -> Result<()> {
        // リポジトリの作成
        let label_repository = InMemoryLabelRepository::new();

        // リポジトリに直接 Label を作成
        let label_saved_to_repository = Label::new(LabelName::new("create-label"));
        label_repository.save(&label_saved_to_repository).await?;
        let label_id_in_repository = label_saved_to_repository.get_id();

        // リクエストの作成とレスポンスの受信
        let req_json_string = r#"{"name": {"value": "update-label"}}"#.to_string();
        let req = tests::build_req_with_json(
            &format!("/labels/{}", label_id_in_repository),
            Method::PATCH,
            req_json_string,
        )?;
        let res = create_app(label_repository).oneshot(req).await?;

        // レスポンスボディを読み込んで Label としてパース
        let res_body: Label = tests::res_to_struct(res).await?;

        let name_in_res = res_body.get_name();

        assert_eq!(label_saved_to_repository, res_body);
        assert_eq!("update-label", name_in_res.to_string());

        Ok(())
    }

    #[tokio::test]
    async fn should_delete_label() -> Result<()> {
        // リポジトリの作成
        let label_repository = InMemoryLabelRepository::new();

        // リポジトリに直接 Label を作成
        let label_saved_to_repository = Label::new(LabelName::new("create-label"));
        label_repository.save(&label_saved_to_repository).await?;
        let label_id_in_repository = label_saved_to_repository.get_id();

        // リクエストの作成とレスポンスの受信
        let req = tests::build_req_with_empty(
            &format!("/labels/{}", label_id_in_repository),
            Method::DELETE,
        )?;
        let res = create_app(label_repository).oneshot(req).await?;

        // 期待通りの結果を確認
        assert_eq!(StatusCode::NO_CONTENT, res.status());

        Ok(())
    }
}
