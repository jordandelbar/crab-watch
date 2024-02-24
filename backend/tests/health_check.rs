#[cfg(test)]
mod tests {
    use crab_watch::{app::create_app, settings::Settings};
    use tokio::net::TcpListener;

    async fn spawn_app() -> std::net::SocketAddr {
        let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        let config = temp_env::with_var("APP_ENVIRONMENT", Some("test"), || {
            Settings::new().expect("")
        });
        tokio::spawn(async move {
            axum::serve(listener, create_app(config)).await.unwrap();
        });
        addr
    }

    #[tokio::test]
    async fn test_health_check() {
        let addr = spawn_app().await;
        let client = reqwest::Client::new();

        let response = client
            .get(format!("http://{addr}/health"))
            .send()
            .await
            .expect("Failed to execute request.");

        assert!(response.status().is_success())
    }
}
