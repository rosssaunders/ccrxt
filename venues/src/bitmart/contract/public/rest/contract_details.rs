//! Get Contract Details endpoint for BitMart Futures (Public)
// See: https://api-cloud-v2.bitmart.com/contract/public/details

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::bitmart::contract::public::rest::enums::ContractStatus;

/// Request for contract details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetContractDetailsRequest<'a> {
    /// Symbol of the contract (e.g., BTCUSDT)
    pub symbol: Option<Cow<'a, str>>,
}

/// Response for contract details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetContractDetailsResponse {
    pub code: i32,
    pub message: Cow<'static, str>,
    #[serde(default)]
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContractDetailsData {
    List(Vec<ContractDetails>),
    Object { contracts: Vec<ContractDetails> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractDetails {
    pub symbol: Cow<'static, str>,
    #[serde(default)]
    pub status: Option<ContractStatus>,
    #[serde(default)]
    pub delist_time: Option<u64>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

// RestClient implementation for this endpoint
use crate::bitmart::{contract::public::rest::client::RestClient, error::Result};

impl RestClient {
    /// Get contract details (public endpoint)
    pub async fn get_contract_details(
        &self,
        req: &GetContractDetailsRequest<'_>,
    ) -> Result<GetContractDetailsResponse> {
        let url = match &req.symbol {
            Some(symbol) => format!("/contract/public/details?symbol={}", symbol),
            None => "/contract/public/details".to_string(),
        };
        self.get(&url).await
    }
}
