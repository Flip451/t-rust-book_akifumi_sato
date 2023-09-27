# テスト

## axum におけるテスト

- `tower::ServiceExt` に定義されている `oneshot` というメソッドを利用して、サーバーのテストを行う

## `/` への GET メソッドに関するテストの追加

**`src/routes.rs`**

```rust
// --snip--

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use axum::{
        body::Body,
        http::{method::Method, Request, header},
    };
    use mime;

    // 中身が空のリクエストを作成するための関数
    pub fn build_req_with_empty(uri: &str, method: Method) -> Result<Request<Body>> {
        let req = Request::builder()
            .uri(uri)
            .method(method)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::empty())?;
        Ok(req)
    }
}
```

**`src/routes/root.rs`**

```rust
// --snip--

#[cfg(test)]
mod tests {
    use crate::routes::{self, tests};

    use anyhow::Result;
    use axum::http::method::Method;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_root() -> Result<()> {
        // リクエストを作成
        let req = tests::build_req_with_empty("/", Method::GET)?;

        // リクエストを送信してレスポンスを受信
        let res = routes::create_app().oneshot(req).await?;

        // レスポンスからボディを取得
        let bytes = hyper::body::to_bytes(res.into_body()).await?;
        
        // ボディを文字列に変換
        let body = String::from_utf8(bytes.to_vec())?;

        assert_eq!(body, "Hello, world!");
        Ok(())
    }
}
```
