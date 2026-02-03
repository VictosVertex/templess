use crate::core::domain::stat::Stat;

#[derive(Clone, PartialEq, Debug)]
pub struct StatData {
    pub stat: Stat,
    pub value: u16,
    pub cap: u16,
}

impl StatData {
    pub fn new(stat: Stat) -> Self {
        Self {
            stat,
            value: 0,
            cap: stat.cap(),
        }
    }
}
