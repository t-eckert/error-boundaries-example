use super::RouterState;
use axum::Router;

pub fn router() -> Router<RouterState> {
    Router::new()
}

async fn create_transaction_handler() {
    unimplemented!()
}
