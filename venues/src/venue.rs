use std::fmt::Display;
use orderbook::aggregated::Venue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinanceSpot {
    Spot,
}

impl Display for BinanceSpot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BNCE")
    }
}

impl Venue for BinanceSpot {
    fn is_usd_denominated(&self) -> bool {
        false // Binance Spot is USDT denominated
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Okx {
    Spot,
}

impl Display for Okx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OKX")
    }
}

impl Venue for Okx {
    fn is_usd_denominated(&self) -> bool {
        false // OKX is USDT denominated
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BybitSpot {
    Spot,
}

impl Display for BybitSpot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BYBT")
    }
}

impl Venue for BybitSpot {
    fn is_usd_denominated(&self) -> bool {
        false // Bybit Spot is USDT denominated
    }
} 