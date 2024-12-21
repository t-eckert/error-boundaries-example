use super::endpoints;
use crate::bank::Bank;
use crate::store::Store;
use std::net::SocketAddr;
use std::sync::Arc;

pub type RouterState = Arc<Bank>;

pub struct Router {
    router: axum::Router,
    socket_addr: SocketAddr,
}

impl Router {
    pub fn new(bank: Bank, socket_addr: SocketAddr) -> Self {
        let router = axum::Router::new()
            .route("/ready", axum::routing::get(|| async { "OK" }))
            .merge(endpoints::accounts::router())
            .merge(endpoints::users::router())
            .with_state(Arc::new(bank));

        Self {
            router,
            socket_addr,
        }
    }

    pub async fn serve(self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = tokio::net::TcpListener::bind(&self.socket_addr).await?;

        Ok(axum::serve(listener, self.router).await?)
    }
}
