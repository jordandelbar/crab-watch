use crate::helpers::spawn_app;
use serde_json::json;

#[tokio::test]
async fn test_create_route_returns_200() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = json!({
        "name": "Cancer pagurus",
        "description": "Cancer pagurus, commonly known as the edible crab or brown crab"
    });

    // Act
    let response = client
        .post(&format!("{}/api/v1/crabs", &app.address))
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
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/api/v1/crabs", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success())
}

#[tokio::test]
async fn test_update_route_returns_200() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let body = json!({
        "name": "Cancer pagurus",
        "description": "Updated description of Cancer pagurus"
    });

    let response = client
        .post(&format!("{}/api/v1/crabs/1", &app.address))
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
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .delete(&format!("{}/api/v1/crabs/3", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success())
}
