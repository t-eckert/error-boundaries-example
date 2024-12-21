mod api;
mod auth;
mod bank;
mod store;

use std::net::SocketAddr;
use std::sync::Arc;

use api::Router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bank = bank::Bank::new(Arc::new(store::StoreMem::new()));
    let router = Router::new(bank, SocketAddr::from(([0, 0, 0, 0], 3000)));
    router.serve().await?;

    Ok(())
}
