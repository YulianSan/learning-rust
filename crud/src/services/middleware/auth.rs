use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

use crate::services::users::models::User;

pub async fn auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let token = match req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
    {
        Some(token) => token,
        None => return Err(StatusCode::UNAUTHORIZED),
    };
    if token == "Bearer 123" {
        let user = User {
            id: 1,
            name: String::from("test"),
            email: String::from("test"),
            password: String::from("test"),
        };

        req.extensions_mut().insert(user);
        return Ok(next.run(req).await);
    }

    Err(StatusCode::UNAUTHORIZED)
}
