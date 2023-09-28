use axum::{async_trait, body::HttpBody, extract::FromRequest, http, Json, BoxError};
use hyper::{Request, StatusCode};
use serde::de::DeserializeOwned;
use validator::Validate;

#[derive(Debug)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<S, B, T> FromRequest<S, B> for ValidatedJson<T>
where
    B: Send + HttpBody + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
    T: DeserializeOwned + Validate,
{
    type Rejection = (http::StatusCode, String);

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        // JSON としてパースを実行
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|rejection| {
                let message = format!("Json parse error: [{}]", rejection);
                (StatusCode::BAD_REQUEST, message)
            })?;

        // バリデーションの実行
        value.validate().map_err(|rejection| {
            let message = format!("Validation error: [{}]", rejection);
            (StatusCode::BAD_REQUEST, message)
        })?;

        // バリデーション済みの JSON として返却
        Ok(ValidatedJson(value))
    }
}
