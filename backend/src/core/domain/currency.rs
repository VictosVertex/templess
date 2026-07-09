#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum Currency {
    None,
    Gold,
    Epic,
    SummonersHall,
    TrialsOfAtlantis,
    Dragon,
    DarknessFalls,
    BountyPoints,
}

impl Currency {
    pub fn from_string(s: &str) -> Self {
        match s {
            "m" => Currency::Gold,
            "sid" | "tus" | "gal" => Currency::Epic,
            "sh" => Currency::SummonersHall,
            "toa" => Currency::TrialsOfAtlantis,
            "dr" => Currency::Dragon,
            "df" => Currency::DarknessFalls,
            "bp" => Currency::BountyPoints,
            _ => Currency::None,
        }
    }
}

impl From<Currency> for u8 {
    fn from(currency: Currency) -> Self {
        currency as u8
    }
}

impl From<u8> for Currency {
    fn from(value: u8) -> Self {
        match value {
            1 => Currency::Gold,
            2 => Currency::Epic,
            3 => Currency::SummonersHall,
            4 => Currency::TrialsOfAtlantis,
            5 => Currency::Dragon,
            6 => Currency::DarknessFalls,
            7 => Currency::BountyPoints,
            _ => Currency::None,
        }
    }
}
