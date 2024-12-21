use super::{Result, RouterState};
use axum::{
    extract::{Path, State},
    Json, Router,
};
use serde::{Deserialize, Serialize};

pub fn router() -> Router<RouterState> {
    Router::new().route("/accounts", axum::routing::post(create_account_handler))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateAccountRequest {
    pub user_id: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateAccountResponse {
    pub id: usize,
    pub owner_id: usize,
    pub balance: u32,
}

async fn create_account_handler(
    State(bank): State<RouterState>,
    Json(request): Json<CreateAccountRequest>,
) -> Result<Json<CreateAccountResponse>> {
    let account = bank.create_new_account_for_user(request.user_id).await?;

    Ok(Json(CreateAccountResponse {
        id: account.id,
        owner_id: account.owner_id,
        balance: account.balance,
    }))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GetAccountResponse {
    pub id: usize,
    pub owner_id: usize,
    pub balance: u32,
}

async fn get_account_handler(
    State(bank): State<RouterState>,
    Path(account_id): Path<usize>,
) -> Result<Json<GetAccountResponse>> {
    let account = bank.get_account(account_id).await?;

    Ok(Json(GetAccountResponse {
        id: account.id,
        owner_id: account.owner_id,
        balance: account.balance,
    }))
}
