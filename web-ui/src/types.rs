use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCredentials {
    pub api_key: String,
    pub api_secret: String,
    pub passphrase: Option<String>, // For exchanges like Coinbase that require it
}

#[derive(Debug, Clone, PartialEq)]
pub enum VenueType {
    Binance,
    BinanceUsdm,
    BinanceCoinm,
    BinanceOptions,
    BinancePortfolio,
    Coinbase,
    Deribit,
    OKX,
    CryptoCom,
    BingX,
    BitMart,
    Bitget,
    Bybit,
    Bullish,
}

impl VenueType {
    pub fn all() -> Vec<VenueType> {
        vec![
            VenueType::Binance,
            VenueType::BinanceUsdm,
            VenueType::BinanceCoinm,
            VenueType::BinanceOptions,
            VenueType::BinancePortfolio,
            VenueType::Coinbase,
            VenueType::Deribit,
            VenueType::OKX,
            VenueType::CryptoCom,
            VenueType::BingX,
            VenueType::BitMart,
            VenueType::Bitget,
            VenueType::Bybit,
            VenueType::Bullish,
        ]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            VenueType::Binance => "Binance Spot",
            VenueType::BinanceUsdm => "Binance USD-M Futures",
            VenueType::BinanceCoinm => "Binance Coin-M Futures",
            VenueType::BinanceOptions => "Binance Options",
            VenueType::BinancePortfolio => "Binance Portfolio Margin",
            VenueType::Coinbase => "Coinbase",
            VenueType::Deribit => "Deribit",
            VenueType::OKX => "OKX",
            VenueType::CryptoCom => "Crypto.com",
            VenueType::BingX => "BingX",
            VenueType::BitMart => "BitMart",
            VenueType::Bitget => "Bitget",
            VenueType::Bybit => "Bybit",
            VenueType::Bullish => "Bullish",
        }
    }

    pub fn requires_passphrase(&self) -> bool {
        matches!(self, VenueType::Coinbase)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EndpointCategory {
    Public,
    Private,
}

impl EndpointCategory {
    pub fn display_name(&self) -> &'static str {
        match self {
            EndpointCategory::Public => "Public (Market Data)",
            EndpointCategory::Private => "Private (Account/Trading)",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApiEndpoint {
    pub name: String,
    pub description: String,
    pub category: EndpointCategory,
    pub parameters: Vec<EndpointParameter>,
    pub venue: VenueType,
}

#[derive(Debug, Clone)]
pub struct EndpointParameter {
    pub name: String,
    pub param_type: ParameterType,
    pub required: bool,
    pub description: String,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ParameterType {
    String,
    Integer,
    Decimal,
    Boolean,
    DateTime,
    Enum(Vec<String>), // Available enum values
}

impl ParameterType {
    pub fn display_name(&self) -> &'static str {
        match self {
            ParameterType::String => "String",
            ParameterType::Integer => "Integer", 
            ParameterType::Decimal => "Decimal",
            ParameterType::Boolean => "Boolean",
            ParameterType::DateTime => "DateTime",
            ParameterType::Enum(_) => "Enum",
        }
    }
}