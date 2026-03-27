use axum::{Json, Router, extract::State, routing::get};

use crate::{
    core::database::schema::create_tables,
    error::{Error, Result},
    initialization::item_init::initialize_items,
    state::SharedState,
};

pub fn router() -> Router<SharedState> {
    Router::new().route("/", get(get_init_status).post(initialize))
}

async fn get_init_status(State(state): State<SharedState>) -> Json<bool> {
    Json(state.is_initialized)
}

async fn initialize(State(state): State<SharedState>) -> Result<()> {
    let mut connection = state
        .db_connection
        .lock()
        .map_err(|e| Error::MutexLockFailed {
            details: e.to_string(),
        })?;

    let items_path = state.config.data.items_path.clone();
    create_tables(&connection)?;

    if std::path::Path::new(&items_path).exists() {
        initialize_items(&mut connection, items_path)?;
    } else {
        return Err(Error::DataMissing { path: items_path });
    }

    Ok(())
}
