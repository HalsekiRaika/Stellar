use error_stack::{Report, ResultExt};
use tracing_subscriber::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::errors::UnRecoverableError;

pub async fn setup_logging() -> Result<(), Report<UnRecoverableError>> {
    tokio::task::spawn_blocking(move || {
        let appender = tracing_appender::rolling::daily(std::path::Path::new("./logs/"), "debug.log");
        let (non_blocking_appender, _guard) = tracing_appender::non_blocking(appender);
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::fmt::layer()
                    .with_filter(tracing_subscriber::EnvFilter::new(
                        std::env::var("RUST_LOG").unwrap_or_else(|_| {
                            "server=debug,axum=trace,tower_http=trace".into()
                        }),
                    ))
                    .with_filter(tracing_subscriber::filter::LevelFilter::DEBUG),
            )
            .with(
                tracing_subscriber::fmt::Layer::default()
                    .with_writer(non_blocking_appender)
                    .with_ansi(false)
                    .with_filter(tracing_subscriber::filter::LevelFilter::DEBUG),
            )
            .init();
    }).await.change_context_lazy(|| UnRecoverableError)?;
    tracing::info!("tracing setup successfully.");
    Ok(())
}
