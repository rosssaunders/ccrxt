use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Include the generated endpoints from build.rs
include!("generated_endpoints.rs");

/// Represents the type of an API endpoint parameter
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ParameterType {
    String,
    Number,
    Boolean,
    Decimal,
    Array(Box<ParameterType>),
    Enum(Vec<String>),
    Object(Vec<Parameter>),
}

/// Represents a parameter for an API endpoint
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub param_type: ParameterType,
    pub required: bool,
    pub default: Option<String>,
    pub description: String,
    pub example: Option<String>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub pattern: Option<String>,
}

/// Authentication requirement for an endpoint
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuthType {
    None,
    Public,
    Private,
    Trade,
    View,
}

/// HTTP method for the endpoint
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

/// Supported venues
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub enum Venue {
    Coinbase,
    BinanceSpot,
    BinanceUsdm,
    BinanceCoinm,
    Deribit,
    OKX,
    CryptoCom,
    Bitmart,
    BingX,
    Bullish,
    Bitget,
    Bybit,
}

impl Venue {
    pub fn display_name(&self) -> &'static str {
        match self {
            Venue::Coinbase => "Coinbase",
            Venue::BinanceSpot => "Binance Spot",
            Venue::BinanceUsdm => "Binance USD-M Futures",
            Venue::BinanceCoinm => "Binance Coin-M Futures",
            Venue::Deribit => "Deribit",
            Venue::OKX => "OKX",
            Venue::CryptoCom => "Crypto.com",
            Venue::Bitmart => "BitMart",
            Venue::BingX => "BingX",
            Venue::Bullish => "Bullish",
            Venue::Bitget => "Bitget",
            Venue::Bybit => "Bybit",
        }
    }

    pub fn all() -> Vec<Venue> {
        vec![
            Venue::Coinbase,
            Venue::BinanceSpot,
            Venue::BinanceUsdm,
            Venue::BinanceCoinm,
            Venue::Deribit,
            Venue::OKX,
            Venue::CryptoCom,
            Venue::Bitmart,
            Venue::BingX,
            Venue::Bullish,
            Venue::Bitget,
            Venue::Bybit,
        ]
    }
}

/// API endpoint category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EndpointCategory {
    MarketData,
    Trading,
    Account,
    Wallet,
    System,
}

impl EndpointCategory {
    pub fn display_name(&self) -> &'static str {
        match self {
            EndpointCategory::MarketData => "Market Data",
            EndpointCategory::Trading => "Trading",
            EndpointCategory::Account => "Account",
            EndpointCategory::Wallet => "Wallet",
            EndpointCategory::System => "System",
        }
    }
}

/// Complete API endpoint definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiEndpoint {
    pub id: String,
    pub venue: Venue,
    pub name: String,
    pub method: HttpMethod,
    pub path: String,
    pub category: EndpointCategory,
    pub auth_type: AuthType,
    pub description: String,
    pub documentation: Option<String>,
    pub parameters: Vec<Parameter>,
    pub path_parameters: Vec<Parameter>,
    pub rate_limit_weight: Option<u32>,
    pub example_request: Option<serde_json::Value>,
    pub example_response: Option<serde_json::Value>,
}

impl ApiEndpoint {
    pub fn new(
        id: String,
        venue: Venue,
        name: String,
        method: HttpMethod,
        path: String,
        category: EndpointCategory,
        auth_type: AuthType,
        description: String,
    ) -> Self {
        Self {
            id,
            venue,
            name,
            method,
            path,
            category,
            auth_type,
            description,
            documentation: None,
            parameters: Vec::new(),
            path_parameters: Vec::new(),
            rate_limit_weight: None,
            example_request: None,
            example_response: None,
        }
    }

    pub fn with_parameter(mut self, parameter: Parameter) -> Self {
        self.parameters.push(parameter);
        self
    }

    pub fn with_path_parameter(mut self, parameter: Parameter) -> Self {
        self.path_parameters.push(parameter);
        self
    }

    pub fn with_rate_limit_weight(mut self, weight: u32) -> Self {
        self.rate_limit_weight = Some(weight);
        self
    }

    pub fn with_documentation(mut self, docs: String) -> Self {
        self.documentation = Some(docs);
        self
    }
}

/// Collection of all API endpoints organized by venue
#[derive(Clone)]
pub struct ApiEndpoints {
    endpoints: HashMap<Venue, Vec<ApiEndpoint>>,
}

impl ApiEndpoints {
    pub fn new() -> Self {
        // Use auto-generated endpoints from build script
        Self::create_from_source()
    }

    pub fn get_venue_endpoints(&self, venue: &Venue) -> Option<&Vec<ApiEndpoint>> {
        self.endpoints.get(venue)
    }

    pub fn get_endpoint(&self, venue: &Venue, id: &str) -> Option<&ApiEndpoint> {
        self.endpoints.get(venue)?.iter().find(|ep| ep.id == id)
    }

    pub fn get_endpoints_by_category(&self, venue: &Venue, category: &EndpointCategory) -> Vec<&ApiEndpoint> {
        self.endpoints
            .get(venue)
            .map(|eps| eps.iter().filter(|ep| ep.category == *category).collect())
            .unwrap_or_default()
    }
}

impl Default for ApiEndpoints {
    fn default() -> Self {
        Self::new()
    }
}