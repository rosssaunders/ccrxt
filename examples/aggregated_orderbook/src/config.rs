use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct VenueConfig {
    pub exchange: String,
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub venues: Vec<VenueConfig>,
    pub price_precision: u32,
    pub update_interval_ms: u64,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let contents = std::fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&contents)?;
        Ok(config)
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum VenueType {
    BinanceSpot,
    OKX,
    BybitSpot,
}

impl std::str::FromStr for VenueType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "BINANCE" => Ok(VenueType::BinanceSpot),
            "OKX" => Ok(VenueType::OKX),
            "BYBIT" => Ok(VenueType::BybitSpot),
            _ => Err(format!("Unsupported exchange: {}", s)),
        }
    }
}

impl std::fmt::Display for VenueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VenueType::BinanceSpot => write!(f, "Binance"),
            VenueType::OKX => write!(f, "OKX"),
            VenueType::BybitSpot => write!(f, "Bybit"),
        }
    }
} 