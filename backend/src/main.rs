use crab_watch::app::create_app;
use crab_watch::settings::Settings;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let config = Settings::new().expect("Failed to load settings");
    let app = create_app(config.clone());

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "crab_watch=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let port = config.application.port;
    let host = config.application.host;
    let listener = tokio::net::TcpListener::bind(&format!("{}:{}", host, port))
        .await
        .expect("Could not listen to port");
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
