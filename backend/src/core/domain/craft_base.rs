use super::{
    currency::Currency,
    item::{Item, ItemSource},
    item_bonus::ItemBonus,
    item_slot::ItemSlot,
    item_type::ItemType,
    realm::Realm,
};

#[derive(Debug, Clone, PartialEq)]
pub struct CraftBase {
    pub id: u32,
    pub name: String,
    pub item_type: ItemType,
    pub item_slot: ItemSlot,
    pub realm: Realm,
    pub base_bonus: ItemBonus,
}

impl From<CraftBase> for Item {
    fn from(item: CraftBase) -> Self {
        Item {
            id: item.id,
            name: item.name,
            model: 0,
            object_type: item.item_type,
            item_slot: item.item_slot,
            level: 0,
            quality: 100,
            weapon_hand: 0,
            weapon_speed: 0,
            damage_type: 0,
            realm: item.realm,
            required_level: 0,
            bonus_level: 0,
            shield_size: 0,
            instrument_type: 0,
            is_tradable: false,
            utility_single: 0.0,
            utility: 0.0,
            source: ItemSource::Crafted,
            allowed_classes: Vec::new(),
            bonuses: vec![item.base_bonus],
            proc1_json: None,
            proc2_json: None,
            use1_json: None,
            use2_json: None,
            passive_json: None,
            react1_json: None,
            react2_json: None,
            price: 0,
            currency: Currency::None,
        }
    }
}
