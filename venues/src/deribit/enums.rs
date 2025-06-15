use serde::{Deserialize, Serialize};

/// Combo state as returned by Deribit API
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ComboState {
    /// Request for quote state
    Rfq,
    /// Active combo
    Active,
    /// Inactive combo
    Inactive,
}

/// Combo leg information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComboLeg {
    /// Size multiplier of a leg. A negative value indicates that the trades on given leg 
    /// are in opposite direction to the combo trades they originate from
    pub amount: i64,
    /// Unique instrument identifier
    pub instrument_name: String,
}

/// Combo information response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComboInfo {
    /// The timestamp (milliseconds since the Unix epoch)
    pub creation_timestamp: i64,
    /// Unique combo identifier
    pub id: String,
    /// Instrument ID
    pub instrument_id: i64,
    /// Array of combo legs
    pub legs: Vec<ComboLeg>,
    /// Combo state: "rfq", "active", "inactive"
    pub state: ComboState,
    /// The timestamp (milliseconds since the Unix epoch)
    pub state_timestamp: i64,
}