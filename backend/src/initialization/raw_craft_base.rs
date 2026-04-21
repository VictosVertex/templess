use serde::Deserialize;

use crate::core::domain::{
    craft_base::CraftBase, item_bonus::ItemBonus, item_slot::ItemSlot, item_type::ItemType,
    realm::Realm, stat::Stat,
};

#[derive(Debug, Clone, Deserialize)]
pub struct RawCraftBase {
    pub name: String,
    pub item_type: u16,
    pub item_slot: u16,
    pub bonuses: Vec<RawCraftBonus>,
    pub realm: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawCraftBonus {
    pub stat: String,
    pub value: i32,
}

impl RawCraftBase {
    pub fn into_craft_base(self, id: i32) -> Option<CraftBase> {
        let base_bonus = self.bonuses.into_iter().next().and_then(|bonus| {
            let stat = bonus.stat.parse::<Stat>().ok()?;
            let value = u16::try_from(bonus.value).ok()?;

            Some(ItemBonus { stat, value })
        })?;

        Some(CraftBase {
            id,
            name: self.name,
            item_type: ItemType::from_repr(self.item_type)?,
            item_slot: ItemSlot::from_repr(self.item_slot)?,
            realm: Realm::from_repr(self.realm)?,
            base_bonus,
        })
    }
}
