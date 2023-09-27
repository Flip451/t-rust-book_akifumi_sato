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

## "/users" への JSON の POST に関するテスト

**`src/routes.rs`**

```rust
// --snip--

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use axum::{
        body::Body,
        http::{header, method::Method, Request},
        response::Response,
    };
    use mime;
    use serde::de::DeserializeOwned;

    pub fn build_req_with_empty(uri: &str, method: Method) -> Result<Request<Body>> {
        // --snip--
    }

    // JSON 文字列を伴うリクエストを作成するための関数
    pub fn build_req_with_json(
        uri: &str,
        method: Method,
        json_body_string: String,
    ) -> Result<Request<Body>> {
        let req = Request::builder()
            .uri(uri)
            .method(method)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(json_body_string))?;
        Ok(req)
    }

    // レスポンスのボディに含まれる JSON をパースするための関数
    pub async fn res_to_struct<T>(res: Response) -> Result<T>
    where
        T: DeserializeOwned,
    {
        // レスポンスからボディを取得
        let bytes = hyper::body::to_bytes(res.into_body()).await?;

        // ボディをバイト列から文字列に変換
        let body = String::from_utf8(bytes.to_vec())?;

        // ボディを json としてパース
        let data: T = serde_json::from_str(&body)?;
        Ok(data)
    }
}
```

**`src/routes/users.rs`**

```rust
// --snip--

// テストのために必要な注釈を追加
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct User {
    id: i32,
    username: String,
}

// --snip--

#[cfg(test)]
mod tests {
    use super::*;

    use crate::routes::{self, tests};

    use anyhow::Result;
    use axum::http::method::Method;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_create_user() -> Result<()> {
        let req_body = r#"{"username": "佐藤 太郎"}"#.to_string();
        let req = tests::build_req_with_json("/users", Method::POST, req_body)?;
        let res = routes::create_app().oneshot(req).await?;
        let res_body: User = tests::res_to_struct(res).await?;
        
        let expected = User {
            id: 1337,
            username: "佐藤 太郎".to_string()
        };
        assert_eq!(expected, body_res);
        Ok(())
    }
}
```
