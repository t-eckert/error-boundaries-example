use super::{Result, RouterState};
use axum::{extract::State, Json, Router};
use serde::{Deserialize, Serialize};

pub fn router() -> Router<RouterState> {
    Router::new().route("/users", axum::routing::post(create_user_handler))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateUserRequest {
    pub name: String,
    pub password: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateUserResponse {
    pub id: usize,
    pub name: String,
}

async fn create_user_handler(
    State(bank): State<RouterState>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<CreateUserResponse>> {
    let user = bank
        .sign_up_new_user(&request.name, &request.password)
        .await?;

    Ok(Json(CreateUserResponse {
        id: user.id,
        name: user.name.clone(),
    }))
}
