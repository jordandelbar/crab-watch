use crate::database::Pool;
use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::web::Data;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::ResponseError;
use std::convert::{TryFrom, TryInto};
use surrealdb::opt::RecordId;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

#[derive(serde::Deserialize)]
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

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CrabName(String);

impl CrabName {
    pub fn parse(s: String) -> Result<CrabName, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!("{} is not a valid crab name.", s))
        } else {
            Ok(Self(s))
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CrabDescription(String);

impl CrabDescription {
    pub fn parse(s: String) -> Result<CrabDescription, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 1024;

        if is_empty_or_whitespace || is_too_long {
            Err(format!("{} is not a valid crab description.", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for CrabDescription {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct NewCrab {
    pub name: CrabName,
    pub description: CrabDescription,
}

impl TryFrom<Payload> for NewCrab {
    type Error = String;

    fn try_from(value: Payload) -> Result<Self, Self::Error> {
        let name = CrabName::parse(value.name)?;
        let description = CrabDescription::parse(value.description)?;
        Ok(Self { name, description })
    }
}

impl AsRef<str> for CrabName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(thiserror::Error)]
pub enum NewCrabError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for NewCrabError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

impl ResponseError for NewCrabError {
    fn status_code(&self) -> StatusCode {
        match self {
            NewCrabError::ValidationError(_) => StatusCode::BAD_REQUEST,
            NewCrabError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub async fn create_crab(
    pool: Data<Pool>,
    payload: web::Json<Payload>,
) -> Result<HttpResponse, NewCrabError> {
    let new_crab: NewCrab = payload
        .0
        .try_into()
        .map_err(NewCrabError::ValidationError)?;
    let conn = pool
        .get()
        .await
        .expect("Failed to get connection from pool.");

    let id = Uuid::new_v4().to_string();
    let _record: Option<NewCrab> = conn
        .create(("crab", &id))
        .content(new_crab)
        .await
        .expect("Failed to create record");
    Ok(HttpResponse::Ok().finish())
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
) -> Result<HttpResponse, NewCrabError> {
    let new_crab: NewCrab = payload
        .0
        .try_into()
        .map_err(NewCrabError::ValidationError)?;
    let conn = pool
        .get()
        .await
        .expect("Failed to get connection from pool");

    let id = path.into_inner();
    let _record: Option<NewCrab> = conn
        .update(("crab", &id))
        .content(new_crab)
        .await
        .expect("Failed to update record");

    Ok(HttpResponse::Ok().finish())
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
