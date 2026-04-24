use crate::api::responses::{
    ClassResponse, GemResponse, ItemResponse, ItemSlotResponse, ItemTypeResponse, StatResponse,
};
use crate::core::database::craft_base_sql::get_craft_bases_by_class;
use crate::core::database::item_sql::get_items_by_class;
use crate::core::domain::gem::Gem;
use crate::core::domain::item_slot::ItemSlot;
use crate::core::domain::item_type::ItemType;
use crate::core::domain::stat::Stat;
use crate::core::domain::{class::Class, item::Item, realm::Realm};
use crate::{error::Result, state::SharedState};
use axum::extract::Query;
use axum::{Json, Router, extract::State, routing::get};
use serde::Deserialize;
use strum::IntoEnumIterator;

use super::responses::RealmResponse;

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/classes", get(list_classes))
        .route("/items", get(search_items))
        .route("/item_slots", get(list_item_slots))
        .route("/item_types", get(list_item_type))
        .route("/realms", get(list_realms))
        .route("/gems", get(list_gems))
        .route("/stats", get(list_stats))
}

async fn list_classes(State(_state): State<SharedState>) -> Result<Json<Vec<ClassResponse>>> {
    let classes = Class::iter().map(|c| c.into()).collect::<Vec<_>>();

    Ok(Json(classes))
}

#[derive(Deserialize)]
pub struct ItemFilter {
    pub class_id: u16,
}

#[axum::debug_handler]
async fn search_items(
    State(state): State<SharedState>,
    Query(filter): Query<ItemFilter>,
) -> Result<Json<Vec<ItemResponse>>> {
    let connection = state
        .db_connection
        .lock()
        .expect("Failed to lock database connection");

    let class = Class::from_repr(filter.class_id).ok_or(crate::error::Error::DataMissing {
        path: format!("Class with id {} not found", filter.class_id),
    })?;
    let mut items: Vec<Item> = get_items_by_class(&connection, class)?;
    items.extend(
        get_craft_bases_by_class(&connection, class)?
            .into_iter()
            .map(Into::into),
    );
    let response_items: Vec<ItemResponse> = items.into_iter().map(ItemResponse::from).collect();

    Ok(Json(response_items))
}

async fn list_realms(State(_state): State<SharedState>) -> Result<Json<Vec<RealmResponse>>> {
    let realms = Realm::iter()
        .filter(|realm| realm.id() > 0)
        .map(|realm| realm.into())
        .collect::<Vec<_>>();

    Ok(Json(realms))
}

async fn list_stats(State(_state): State<SharedState>) -> Result<Json<Vec<StatResponse>>> {
    let stats = Stat::iter().map(|stat| stat.into()).collect::<Vec<_>>();

    Ok(Json(stats))
}

async fn list_gems(State(_state): State<SharedState>) -> Result<Json<Vec<GemResponse>>> {
    let gems = Gem::all().into_iter().map(Into::into).collect::<Vec<_>>();

    Ok(Json(gems))
}

async fn list_item_slots(State(_state): State<SharedState>) -> Result<Json<Vec<ItemSlotResponse>>> {
    let item_slots = ItemSlot::iter().map(|slot| slot.into()).collect::<Vec<_>>();

    Ok(Json(item_slots))
}

async fn list_item_type(State(_state): State<SharedState>) -> Result<Json<Vec<ItemTypeResponse>>> {
    let item_types = ItemType::iter()
        .map(|item_type| item_type.into())
        .collect::<Vec<_>>();

    Ok(Json(item_types))
}
