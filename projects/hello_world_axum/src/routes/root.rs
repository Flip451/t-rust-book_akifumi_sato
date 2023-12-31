// GET "/" で返却する値を定義する関数
pub async fn index() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use axum::http::Method;
    use tower::ServiceExt;
    use crate::{routes::{create_app, tests::build_req_with_empty}, repository::RepositoryForMemory};

    #[tokio::test]
    async fn should_return_hello_world() -> Result<()> {
        let repository = RepositoryForMemory::new();

        // GET: / へのリクエストを作成
        let req = build_req_with_empty("/", Method::GET)?;

        // GET: / に対するレスポンスを取得
        // `use tower::ServiceExt;` により Router::oneshot メソッドが使えるようになっている
        // oneshot は、リクエストを渡すと一度だけハンドリングを行ってレスポンスを生成してくれる
        let res = create_app(repository).oneshot(req).await?;

        // レスポンス型から Bytes 型を経て String 型のレスポンスボディを取得
        let bytes = hyper::body::to_bytes(res.into_body()).await?;
        let body = String::from_utf8(bytes.to_vec())?;

        assert_eq!(body, "Hello, world!");

        Ok(())
    }
}
