use crate::database::{Manager, Pool};
use crate::routes::{crabs::crabs_routes, health::health_check};
use crate::settings::Settings;
use axum::{routing::get, Extension, Router};
use std::sync::Arc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

pub fn create_app(config: Settings) -> Router {
    let mgr = Manager::new(config.database);
    let pool = Pool::builder(mgr).max_size(50).build().unwrap();
    let shared_pool = Arc::new(pool);

    Router::new()
        .route("/health", get(health_check))
        .nest("/api/v1", crabs_routes())
        .layer(TraceLayer::new_for_http())
        .layer(Extension(shared_pool))
        .layer(CorsLayer::permissive())
}
