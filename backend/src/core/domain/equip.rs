#[derive(Debug, Clone, PartialEq)]
pub enum EquipSource {
    User,
    Optimizer,
}

#[derive(Debug, Clone)]
pub struct EquippedItemState {
    pub item_id: u32,
    pub source: EquipSource,
    pub gem_ids: Vec<u32>,
}
