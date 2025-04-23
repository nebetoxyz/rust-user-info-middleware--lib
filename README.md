# Rust Axum Middleware - Extract User Info from Header

Custom extractor for [Rust](https://www.rust-lang.org/) [Axum](https://docs.rs/axum/latest/axum/) to extract the request id from an HTTP header `X-Endpoint-API-UserInfo`.
Works **ONLY** with [Rust](https://www.rust-lang.org/) [Axum](https://docs.rs/axum/latest/axum/).

## Usage

```rust
use axum::{routing::get, Router};
use user_info_middleware::ExtractUserInfo;

async fn handler(ExtractUserInfo(user_info): ExtractUserInfo) {
    println!("User Info: {:?}", user_info);
}

let app = Router::<()>::new().route("/foo", get(handler));
```

If the extracted value is **missing** or is not a valid **base 64 encoded JSON**, it returns a **400 Bad Request** with this message :

- `Invalid X-Endpoint-API-UserInfo : Not found` : it's a requirement error ;
- `Invalid X-Endpoint-API-UserInfo : Not a valid base 64` : it's a decoding error ;
- `Invalid X-Endpoint-API-UserInfo : Not a valid JSON` : it's a parsing error.

## Samples

## Extract user info

```shell
curl -H "X-Endpoint-API-UserInfo: eyJpc3MiOiJteS1pc3N1ZXIiLCJzdWIiOiJteS1zdWJqZWN0IiwiYXVkIjoibXktYXVkaWVuY2UiLCJuYW1lIjoibXktbmFtZSIsImlhdCI6MTUxNjIzOTAyMiwiZXhwIjoxNTE2MjM5MDIyLCJuYmYiOjE1MTYyMzkwMjIsImp0aSI6Im15LXVuaXF1ZS1pZCJ9" http://api.nebeto.xyz/foo
curl -H "x-endpoint-api-userinfo: eyJpc3MiOiJteS1pc3N1ZXIiLCJzdWIiOiJteS1zdWJqZWN0IiwiYXVkIjoibXktYXVkaWVuY2UiLCJuYW1lIjoibXktbmFtZSIsImlhdCI6MTUxNjIzOTAyMiwiZXhwIjoxNTE2MjM5MDIyLCJuYmYiOjE1MTYyMzkwMjIsImp0aSI6Im15LXVuaXF1ZS1pZCJ9" http://api.nebeto.xyz/foo
curl -H "X-endPoint-Api-UserInfo: eyJpc3MiOiJteS1pc3N1ZXIiLCJzdWIiOiJteS1zdWJqZWN0IiwiYXVkIjoibXktYXVkaWVuY2UiLCJuYW1lIjoibXktbmFtZSIsImlhdCI6MTUxNjIzOTAyMiwiZXhwIjoxNTE2MjM5MDIyLCJuYmYiOjE1MTYyMzkwMjIsImp0aSI6Im15LXVuaXF1ZS1pZCJ9" http://api.nebeto.xyz/foo
```

Where `eyJpc3MiOiJteS1pc3N1ZXIiLCJzdWIiOiJteS1zdWJqZWN0IiwiYXVkIjoibXktYXVkaWVuY2UiLCJuYW1lIjoibXktbmFtZSIsImlhdCI6MTUxNjIzOTAyMiwiZXhwIjoxNTE2MjM5MDIyLCJuYmYiOjE1MTYyMzkwMjIsImp0aSI6Im15LXVuaXF1ZS1pZCJ9` is a JSON base64 encoded :

```json
{
  "iss": "my-issuer",
  "sub": "my-subject",
  "aud": "my-audience",
  "name": "my-name",
  "iat": 1516239022,
  "exp": 1516239022,
  "nbf": 1516239022,
  "jti": "my-unique-id"
}
```

Will give in the `user_info` :

```rust
user_info["iss"] // "my-issuer"
user_info["sub"] // "my-subject"
user_info["aud"] // "my-audience"
user_info["name"] // "my-name"
user_info["iat"] // 1516239022
user_info["exp"] // 1516239022
user_info["nbf"] // 1516239022
user_info["jti"] // "my-unique-id"
```

## Contact

For any question or feature suggestion, you can take a look and open, if necessary, a new [discussion](https://github.com/nebetoxyz/rust-user-info-middleware--lib/discussions).

For any bug, you can take a look to our active issues and open, if necessary, a new [issue](https://github.com/nebetoxyz/rust-user-info-middleware--lib/issues).
