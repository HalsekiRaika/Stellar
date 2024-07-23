#![allow(unused)]

use axum::Router;
use error_stack::{Report, ResultExt};
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use server::errors::UnRecoverableError;
use server::logging;

#[tokio::main]
async fn main() -> Result<(), Report<UnRecoverableError>> {
    logging::setup_logging().await?;
    
    let api = Router::new().with_state(7);
    
    
    let origin_router = Router::new()
        .nest_service("/", ServeDir::new("../frontend/static"))
        .nest("/api", api)
        .layer(TraceLayer::new_for_http())
        .with_state("");

    let tcp = TcpListener::bind("127.0.0.1:8888").await
        .change_context_lazy(|| UnRecoverableError)?;

    tracing::info!("listening on {}", tcp.local_addr().change_context_lazy(|| UnRecoverableError)?);

    axum::serve(tcp, origin_router.into_make_service()).await
        .change_context_lazy(|| UnRecoverableError)?;

    Ok(())
}
