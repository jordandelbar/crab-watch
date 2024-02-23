#[cfg(test)]
mod tests {
    use circuit_chronicle::{app::create_app, settings::Settings};
    use serde_json::json;
    use tokio::net::TcpListener;

    async fn spawn_app() -> std::net::SocketAddr {
        let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        let config = temp_env::with_var("APP_ENVIRONMENT", Some("test"), || {
            Settings::new().expect("Failed to load settings")
        });
        tokio::spawn(async move {
            axum::serve(listener, create_app(config)).await.unwrap();
        });
        addr
    }

    #[tokio::test]
    async fn test_create_route_returns_200() {
        // Arrange
        let addr = spawn_app().await;
        let client = reqwest::Client::new();

        let body = json!({
            "name": "Cancer pagurus",
            "description": "Cancer pagurus, commonly known as the edible crab or brown crab"
        });

        // Act
        let response = client
            .post(format!("http://{addr}/api/v1/crabs"))
            .json(&body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert!(response.status().is_success())
    }

    #[tokio::test]
    async fn test_read_route_returns_200() {
        // Arrange
        let addr = spawn_app().await;
        let client = reqwest::Client::new();

        // Act
        let response = client
            .get(format!("http://{addr}/api/v1/crabs"))
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert!(response.status().is_success())
    }

    #[tokio::test]
    async fn test_update_route_returns_200() {
        // Arrange
        let addr = spawn_app().await;
        let client = reqwest::Client::new();

        // Act
        let body = json!({
            "name": "Cancer pagurus",
            "description": "Updated description of Cancer pagurus"
        });

        let response = client
            .post(format!("http://{addr}/api/v1/crabs/1"))
            .json(&body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert!(response.status().is_success())
    }

    #[tokio::test]
    async fn test_delete_route_returns_200() {
        // Arrange
        let addr = spawn_app().await;
        let client = reqwest::Client::new();

        // Act
        let response = client
            .delete(format!("http://{addr}/api/v1/crabs/3"))
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert!(response.status().is_success())
    }
}
