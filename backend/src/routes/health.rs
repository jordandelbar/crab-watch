use axum::response::{IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub async fn health_check() -> impl IntoResponse {
    Json(Status {
        status: String::from("ok"),
    })
}

#[derive(Serialize, Deserialize)]
pub struct Status {
    status: String,
}
