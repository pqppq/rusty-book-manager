use axum::{extract::State, http::StatusCode, Json};
use kernel::model::auth::event::CreateToken;
use registry::AppRegistry;
use shared::error::AppResult;

use crate::{
    extractor::AuthorizedUser,
    model::auth::{AccessTokenResponse, LoginRequest},
};

pub async fn login(
    State(registry): State<AppRegistry>,
    Json(req): Json<LoginRequest>,
) -> AppResult<Json<AccessTokenResponse>> {
    let user_id = registry
        .auth_repository()
        .verify_user(&req.email, &req.password)
        .await?;

    let access_token = registry
        .auth_repository()
        .crate_token(CreateToken::new(user_id))
        .await?;

    let res = Json(AccessTokenResponse {
        user_id,
        access_token: access_token.0,
    });

    Ok(res)
}

pub async fn logout(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    registry
        .auth_repository()
        .delete_token(user.access_token)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
