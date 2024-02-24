use crate::database::Pool;
use axum::{
    extract::Path,
    response::{IntoResponse, Json},
    routing::{delete, get, post},
    Extension, Router,
};
use std::sync::Arc;
use surrealdb::opt::RecordId;
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Payload {
    pub name: String,
    pub description: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Crab {
    pub id: RecordId,
    pub name: String,
    pub description: String,
}

pub fn crabs_routes() -> Router {
    Router::new()
        .route("/crabs", post(create_crab))
        .route("/crabs", get(list_crab))
        .route("/crabs/:id", get(read_crab))
        .route("/crabs/:id", post(update_crab))
        .route("/crabs/:id", delete(delete_crab))
}

pub async fn create_crab(pool: Extension<Arc<Pool>>, payload: Json<Payload>) -> impl IntoResponse {
    let conn = pool
        .get()
        .await
        .expect("Failed to get connection from pool.");

    let id = Uuid::new_v4().to_string();
    let _record: Option<Payload> = conn
        .create(("crab", &id))
        .content(Payload {
            name: payload.name.clone(),
            description: payload.description.clone(),
        })
        .await
        .expect("Failed to create record");
    // Json(serde_json::json!({"data": payload.name, "description": payload.description}))
    Json(serde_json::json!({"response": format!("{} record created", id)}))
}

pub async fn list_crab(pool: Extension<Arc<Pool>>) -> impl IntoResponse {
    let conn = pool
        .get()
        .await
        .expect("Failed to get connection from pool");

    let crabs: Vec<Crab> = conn
        .select("crab")
        .await
        .expect("Failed to retrieve records");

    Json(crabs)
}

pub async fn update_crab(
    pool: Extension<Arc<Pool>>,
    Path(id): Path<String>,
    payload: Json<Payload>,
) -> impl IntoResponse {
    let conn = pool
        .get()
        .await
        .expect("Failed to get connection from pool");

    let _record: Option<Payload> = conn
        .update(("crab", &id))
        .content(Payload {
            name: payload.name.clone(),
            description: payload.description.clone(),
        })
        .await
        .expect("Failed to update record");

    Json(serde_json::json!({"response": format!("{} record updated", id)}))
}

pub async fn delete_crab(pool: Extension<Arc<Pool>>, Path(id): Path<String>) -> impl IntoResponse {
    let conn = pool
        .get()
        .await
        .expect("Failed to get connection from pool");

    let _: Option<Crab> = conn
        .delete(("crab", &id))
        .await
        .expect("Failed to delete record");

    Json(serde_json::json!({"response": format!("{} record deleted", id)}))
}

pub async fn read_crab(pool: Extension<Arc<Pool>>, Path(id): Path<String>) -> impl IntoResponse {
    let conn = pool
        .get()
        .await
        .expect("Failed to get connection from pool");

    let crab: Option<Crab> = conn
        .select(("crab", id))
        .await
        .expect("Failed to retrieve record");

    Json(crab)
}
