use crate::database::Pool;
use actix_web::web;
use actix_web::web::Data;
use actix_web::Responder;
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

pub async fn create_crab(pool: Data<Pool>, payload: web::Json<Payload>) -> impl Responder {
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
    web::Json(serde_json::json!({"response": format!("{} record created", id)}))
}

pub async fn list_crab(pool: Data<Pool>) -> impl Responder {
    let conn = pool
        .get()
        .await
        .expect("Failed to get connection from pool");

    let crabs: Vec<Crab> = conn
        .select("crab")
        .await
        .expect("Failed to retrieve records");

    web::Json(crabs)
}

pub async fn update_crab(
    pool: Data<Pool>,
    path: web::Path<String>,
    payload: web::Json<Payload>,
) -> impl Responder {
    let conn = pool
        .get()
        .await
        .expect("Failed to get connection from pool");

    let id = path.into_inner();
    let _record: Option<Payload> = conn
        .update(("crab", &id))
        .content(Payload {
            name: payload.name.clone(),
            description: payload.description.clone(),
        })
        .await
        .expect("Failed to update record");

    web::Json(serde_json::json!({"response": format!("{} record updated", &id)}))
}

pub async fn delete_crab(pool: Data<Pool>, path: web::Path<String>) -> impl Responder {
    let conn = pool
        .get()
        .await
        .expect("Failed to get connection from pool");

    let id = path.into_inner();
    let _: Option<Crab> = conn
        .delete(("crab", &id))
        .await
        .expect("Failed to delete record");

    web::Json(serde_json::json!({"response": format!("{} record deleted", &id)}))
}

pub async fn read_crab(pool: Data<Pool>, path: web::Path<String>) -> impl Responder {
    let conn = pool
        .get()
        .await
        .expect("Failed to get connection from pool");

    let crab: Option<Crab> = conn
        .select(("crab", path.into_inner()))
        .await
        .expect("Failed to retrieve record");

    web::Json(crab)
}
