use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use base64::{Engine, engine::general_purpose};
use log::error;
use serde_json::{self, Value};

/// This is a custom extractor for Axum that extracts the user info, via the `X-Endpoint-API-UserInfo` header.
/// If the `X-Endpoint-API-UserInfo` header is present and it's a valid base 64 encoded JSON value, it returns it.
/// If the `X-Endpoint-API-UserInfo` header is present and it's an invalid base 64 encoded JSON (either not a base 64 or a JSON structure), it returns a 400 Bad Request error with a specific message.
/// If the `X-Endpoint-API-UserInfo` header is not present, it returns a 400 Bad Request error with a specific message.
///
/// # Links
///
/// https://docs.rs/axum/latest/axum/index.html
/// https://docs.rs/axum/latest/axum/extract/index.html#defining-custom-extractors
/// https://docs.rs/base64/latest/base64/index.html
/// https://docs.rs/serde_json/latest/serde_json/index.html
///
/// # Author
///
/// François GRUCHALA <francois@nebeto.xyz>
///
/// # Examples
///
/// ```rust
/// use axum::{routing::get, Router};
/// use user_info_middleware::ExtractUserInfo;
///
/// async fn handler(ExtractUserInfo(user_info): ExtractUserInfo) {
///     println!("User Info: {:?}", user_info);
/// }
///
/// let app = Router::<()>::new().route("/foo", get(handler));
/// ```
#[derive(Debug, Clone)]
pub struct ExtractUserInfo(pub Value);

const HEADER_X_USER_INFO: &str = "X-Endpoint-API-UserInfo";

impl<S> FromRequestParts<S> for ExtractUserInfo
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let user_info = parts.headers.get(HEADER_X_USER_INFO);

        match user_info {
            Some(user_info) => {
                let user_info = user_info.to_str().unwrap().trim();
                let decoded_user_info = general_purpose::STANDARD.decode(user_info);

                if decoded_user_info.is_err() {
                    error!(
                        "[{}] Failed to decode base 64 due to : {:?}",
                        HEADER_X_USER_INFO,
                        decoded_user_info.err().unwrap()
                    );

                    return Err((
                        StatusCode::BAD_REQUEST,
                        format!("Invalid {} : Not a valid base 64", HEADER_X_USER_INFO),
                    ));
                }

                let parsed_user_info = serde_json::from_slice(&decoded_user_info.unwrap());

                if parsed_user_info.is_err() {
                    error!(
                        "[{}] Failed to parse JSON due to : {:?}",
                        HEADER_X_USER_INFO,
                        parsed_user_info.err().unwrap()
                    );

                    return Err((
                        StatusCode::BAD_REQUEST,
                        format!("Invalid {} : Not a valid JSON", HEADER_X_USER_INFO),
                    ));
                }

                Ok(ExtractUserInfo(parsed_user_info.unwrap()))
            }
            None => Err((
                StatusCode::BAD_REQUEST,
                format!("Invalid {} : Not found", HEADER_X_USER_INFO),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ExtractUserInfo, HEADER_X_USER_INFO};
    use axum::{
        body::Body,
        extract::FromRequestParts,
        http::{Request, StatusCode},
    };

    #[tokio::test]
    async fn test_lib_extract_user_info_with_header_ok_one() {
        let request = Request::builder()
            .header("x-endpoint-api-userinfo", "eyJpc3MiOiJteS1pc3N1ZXIiLCJzdWIiOiJteS1zdWJqZWN0IiwiYXVkIjoibXktYXVkaWVuY2UiLCJuYW1lIjoibXktbmFtZSIsImlhdCI6MTUxNjIzOTAyMiwiZXhwIjoxNTE2MjM5MDIyLCJuYmYiOjE1MTYyMzkwMjIsImp0aSI6Im15LXVuaXF1ZS1pZCJ9")
            .body(Body::empty())
            .unwrap();

        let mut parts = request.into_parts();

        let user_info = ExtractUserInfo::from_request_parts(&mut parts.0, &()).await;

        match user_info {
            Ok(user_info) => assert_eq!(
                user_info.0,
                serde_json::json!({
                  "iss": "my-issuer",
                  "sub": "my-subject",
                  "aud": "my-audience",
                  "name": "my-name",
                  "iat": 1516239022,
                  "exp": 1516239022,
                  "nbf": 1516239022,
                  "jti": "my-unique-id"
                })
            ),
            Err(err) => assert!(false, "Expected a valid user info : {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_lib_extract_user_info_with_header_ok_two() {
        let request = Request::builder()
            .header(
                "X-Endpoint-API-UserInfo",
                " eyJpc3MiOiJteS1pc3N1ZXIiLCJzdWIiOiJteS1zdWJqZWN0IiwiYXVkIjoibXktYXVkaWVuY2UiLCJuYW1lIjoibXktbmFtZSIsImlhdCI6MTUxNjIzOTAyMiwiZXhwIjoxNTE2MjM5MDIyLCJuYmYiOjE1MTYyMzkwMjIsImp0aSI6Im15LXVuaXF1ZS1pZCJ9 ",
            )
            .body(Body::empty())
            .unwrap();

        let mut parts = request.into_parts();

        let user_info = ExtractUserInfo::from_request_parts(&mut parts.0, &()).await;

        match user_info {
            Ok(user_info) => assert_eq!(
                user_info.0,
                serde_json::json!({
                  "iss": "my-issuer",
                  "sub": "my-subject",
                  "aud": "my-audience",
                  "name": "my-name",
                  "iat": 1516239022,
                  "exp": 1516239022,
                  "nbf": 1516239022,
                  "jti": "my-unique-id"
                })
            ),
            Err(err) => assert!(false, "Expected a valid user info : {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_lib_extract_user_info_with_header_ko_not_base64() {
        let request = Request::builder()
            .header("X-Endpoint-api-UserInfo", "this-is-not-a-base64")
            .body(Body::empty())
            .unwrap();

        let mut parts = request.into_parts();

        let user_info = ExtractUserInfo::from_request_parts(&mut parts.0, &()).await;

        match user_info {
            Ok(_) => assert!(false, "Expected an error"),
            Err(err) => assert_eq!(
                err,
                (
                    StatusCode::BAD_REQUEST,
                    format!("Invalid {} : Not a valid base 64", HEADER_X_USER_INFO)
                )
            ),
        }
    }

    #[tokio::test]
    async fn test_lib_extract_user_info_with_header_ko_not_json() {
        let request = Request::builder()
            .header("x-EndPoint-api-UserInfo", "dGhpcy1pcy1ub3QtYS1qc29u")
            .body(Body::empty())
            .unwrap();

        let mut parts = request.into_parts();

        let user_info = ExtractUserInfo::from_request_parts(&mut parts.0, &()).await;

        match user_info {
            Ok(_) => assert!(false, "Expected an error"),
            Err(err) => assert_eq!(
                err,
                (
                    StatusCode::BAD_REQUEST,
                    format!("Invalid {} : Not a valid JSON", HEADER_X_USER_INFO)
                )
            ),
        }
    }

    #[tokio::test]
    async fn test_lib_extract_user_info_without_header() {
        let request = Request::builder().body(Body::empty()).unwrap();

        let mut parts = request.into_parts();

        let user_info = ExtractUserInfo::from_request_parts(&mut parts.0, &()).await;

        match user_info {
            Ok(_) => assert!(false, "Expected an error"),
            Err(err) => assert_eq!(
                err,
                (
                    StatusCode::BAD_REQUEST,
                    format!("Invalid {} : Not found", HEADER_X_USER_INFO)
                )
            ),
        }
    }
}
