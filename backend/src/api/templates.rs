use crate::{Error, error::Result, state::SharedState};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get},
};

use super::requests::CreateTemplateRequest;
use super::responses::TemplatesResponse;
use crate::core::database::template_sql;

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/", get(list_templates).post(insert_template))
        .route("/{id}", get(get_template).delete(delete_template))
}

async fn list_templates(State(_state): State<SharedState>) -> Result<Json<Vec<TemplatesResponse>>> {
    let connection = _state
        .db_connection
        .lock()
        .expect("Failed to acquire database connection lock");

    let templates = template_sql::get_templates(&connection)?;
    let response_templates = templates
        .into_iter()
        .map(|template| template.into())
        .collect();

    Ok(Json(response_templates))
}

async fn insert_template(
    State(_state): State<SharedState>,
    Json(payload): Json<CreateTemplateRequest>,
) -> Result<Json<i32>> {
    let connection = _state
        .db_connection
        .lock()
        .expect("Failed to acquire database connection lock");

    let template_id = template_sql::insert_template(&connection, payload.name, payload.class_id)?;

    Ok(Json(template_id))
}

async fn delete_template(State(state): State<SharedState>, Path(id): Path<i32>) -> Result<()> {
    let connection = state
        .db_connection
        .lock()
        .expect("Failed to acquire database connection lock");

    template_sql::delete_template(&connection, id)?;

    Ok(())
}

async fn get_template(
    State(state): State<SharedState>,
    Path(id): Path<i32>,
) -> Result<Json<TemplatesResponse>> {
    let connection = state
        .db_connection
        .lock()
        .expect("Failed to acquire database connection lock");

    let template = template_sql::get_template(&connection, id)?;

    if let Some(template) = template {
        Ok(Json(template.into()))
    } else {
        Err(Error::DataMissing {
            path: "template".to_string(),
        }
        .into())
    }
}
