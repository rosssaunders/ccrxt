use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{ContractStatus, ContractType, ResponseHeaders, RestResponse, Result};

/// Get contract information request
#[derive(Debug, Clone, Serialize)]
pub struct GetContractRequest {
    pub symbol: String,
}

/// Contract information response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractInfo {
    /// Symbol of the contract
    pub symbol: String,
    /// Root symbol of the contract
    pub root_symbol: String,
    /// Type of the contract
    #[serde(rename = "type")]
    pub contract_type: ContractType,
    /// First open date
    pub first_open_date: i64,
    /// Expiry date
    pub expire_date: Option<i64>,
    /// Settlement date
    pub settle_date: Option<i64>,
    /// Base currency
    pub base_currency: String,
    /// Quote currency
    pub quote_currency: String,
    /// Settlement currency
    pub settle_currency: String,
    /// Maximum order quantity
    pub max_order_qty: f64,
    /// Maximum price
    pub max_price: f64,
    /// Lot size
    pub lot_size: f64,
    /// Tick size
    pub tick_size: f64,
    /// Index price tick size
    pub index_price_tick_size: f64,
    /// Multiplier
    pub multiplier: f64,
    /// Initial margin
    pub initial_margin: f64,
    /// Maintenance margin
    pub maintenance_margin: Option<f64>,
    /// Maximum risk limit
    pub max_risk_limit: f64,
    /// Minimum risk limit
    pub min_risk_limit: f64,
    /// Risk limit step
    pub risk_limit_step: Option<f64>,
    /// Maker fee rate
    pub maker_fee_rate: f64,
    /// Taker fee rate
    pub taker_fee_rate: f64,
    /// Taker fixed fee
    pub taker_fixed_fee: Option<f64>,
    /// Maker fixed fee
    pub maker_fixed_fee: Option<f64>,
    /// Settlement fee
    pub settlement_fee: Option<f64>,
    /// Is quanto
    pub is_quanto: bool,
    /// Is inverse
    pub is_inverse: bool,
    /// Mark method
    pub mark_method: Option<String>,
    /// Fair method
    pub fair_method: Option<String>,
    /// Funding base symbol
    pub funding_base_symbol: Option<String>,
    /// Funding quote symbol
    pub funding_quote_symbol: Option<String>,
    /// Funding rate symbol
    pub funding_rate_symbol: Option<String>,
    /// Index symbol
    pub index_symbol: Option<String>,
    /// Settlement symbol
    pub settlement_symbol: Option<String>,
    /// Status
    pub status: ContractStatus,
    /// Fund fee begin time
    pub fund_fee_begin_time: Option<i64>,
    /// Fund fee end time
    pub fund_fee_end_time: Option<i64>,
    /// Fund interval
    pub fund_interval: Option<i64>,
    /// Delivery fee
    pub delivery_fee: Option<f64>,
    /// Position limit
    pub position_limit: Option<f64>,
}

/// Get all contracts request
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAllContractsRequest;

/// Response for getting all contracts
pub type GetAllContractsResponse = Vec<ContractInfo>;

impl super::RestClient {
    /// Get contract information for a specific symbol
    pub async fn get_contract(
        &self,
        request: GetContractRequest,
    ) -> Result<(RestResponse<ContractInfo>, ResponseHeaders)> {
        let endpoint = format!("/api/v1/contracts/{}", request.symbol);
        self.get(&endpoint, None).await
    }

    /// Get all contract information
    pub async fn get_all_contracts(
        &self,
        _request: GetAllContractsRequest,
    ) -> Result<(RestResponse<GetAllContractsResponse>, ResponseHeaders)> {
        let endpoint = "/api/v1/contracts/active";
        self.get(endpoint, None).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_contract_request_serialization() {
        let request = GetContractRequest {
            symbol: "XBTUSDTM".to_string(),
        };

        // Test that the struct can be created and accessed
        assert_eq!(request.symbol, "XBTUSDTM");
    }

    #[test]
    fn test_contract_info_deserialization() {
        let json = r#"{
            "symbol": "XBTUSDTM",
            "rootSymbol": "USDT",
            "type": "FFWCSX",
            "firstOpenDate": 1585054800000,
            "expireDate": null,
            "settleDate": null,
            "baseCurrency": "XBT",
            "quoteCurrency": "USDT",
            "settleCurrency": "USDT",
            "maxOrderQty": 1000000,
            "maxPrice": 1000000.0,
            "lotSize": 1.0,
            "tickSize": 0.1,
            "indexPriceTickSize": 0.01,
            "multiplier": 0.001,
            "initialMargin": 0.01,
            "maintenanceMargin": null,
            "maxRiskLimit": 200000,
            "minRiskLimit": 200000,
            "riskLimitStep": null,
            "makerFeeRate": 0.0002,
            "takerFeeRate": 0.0006,
            "takerFixedFee": null,
            "makerFixedFee": null,
            "settlementFee": null,
            "isQuanto": false,
            "isInverse": true,
            "markMethod": null,
            "fairMethod": null,
            "fundingBaseSymbol": null,
            "fundingQuoteSymbol": null,
            "fundingRateSymbol": null,
            "indexSymbol": null,
            "settlementSymbol": null,
            "status": "Open",
            "fundFeeBeginTime": null,
            "fundFeeEndTime": null,
            "fundInterval": null,
            "deliveryFee": null,
            "positionLimit": null
        }"#;

        let contract: ContractInfo = serde_json::from_str(json).unwrap();
        assert_eq!(contract.symbol, "XBTUSDTM");
        assert_eq!(contract.base_currency, "XBT");
        assert_eq!(contract.quote_currency, "USDT");
    }
}
