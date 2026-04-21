use axum::{Json, Router, extract::State, routing::get};

use crate::{
    core::database::schema::{create_tables, is_initialized},
    error::{Error, Result},
    initialization::craft_base_init::initialize_craft_bases,
    initialization::item_init::initialize_items,
    state::SharedState,
};

pub fn router() -> Router<SharedState> {
    Router::new().route("/", get(get_init_status).post(initialize))
}

async fn get_init_status(State(state): State<SharedState>) -> Result<Json<bool>> {
    let connection = state
        .db_connection
        .lock()
        .map_err(|e| Error::MutexLockFailed {
            details: e.to_string(),
        })?;

    Ok(Json(is_initialized(&connection)?))
}

async fn initialize(State(state): State<SharedState>) -> Result<()> {
    let mut connection = state
        .db_connection
        .lock()
        .map_err(|e| Error::MutexLockFailed {
            details: e.to_string(),
        })?;

    let raw_data_path = std::path::PathBuf::from(&state.config.data.raw_data_path);
    let items_path = raw_data_path.join("items.json");
    let craft_bases_path = raw_data_path.join("craft_bases.json");
    create_tables(&connection)?;

    if items_path.exists() {
        initialize_items(&mut connection, items_path.to_string_lossy().into_owned())?;
    } else {
        return Err(Error::DataMissing {
            path: items_path.to_string_lossy().into_owned(),
        });
    }

    if craft_bases_path.exists() {
        initialize_craft_bases(
            &mut connection,
            craft_bases_path.to_string_lossy().into_owned(),
        )?;
    } else {
        return Err(Error::DataMissing {
            path: craft_bases_path.to_string_lossy().into_owned(),
        });
    }

    Ok(())
}
