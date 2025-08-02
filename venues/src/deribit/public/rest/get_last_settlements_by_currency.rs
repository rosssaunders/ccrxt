use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult, enums::Currency};

const LAST_SETTLEMENTS_BY_CURRENCY_ENDPOINT: &str = "public/get_last_settlements_by_currency";

/// Settlement type for filtering settlements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SettlementType {
    /// Regular settlement event.
    Settlement,

    /// Delivery event for futures.
    Delivery,

    /// Bankruptcy event.
    Bankruptcy,
}

/// Request parameters for the get_last_settlements_by_currency endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct GetLastSettlementsByCurrencyRequest {
    /// Currency for which to retrieve settlements.
    pub currency: Currency,

    /// Settlement type (settlement, delivery, bankruptcy). Optional.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub settlement_type: Option<SettlementType>,

    /// Number of requested items (default: 20). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,

    /// Continuation token for pagination. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation: Option<String>,

    /// The latest timestamp to return result from (milliseconds since the UNIX epoch). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_start_timestamp: Option<u64>,
}

/// Represents a single settlement entry.
#[derive(Debug, Clone, Deserialize)]
pub struct SettlementEntry {
    /// Settlement type (settlement, delivery, bankruptcy).
    #[serde(rename = "type")]
    pub settlement_type: SettlementType,

    /// Timestamp in milliseconds since epoch.
    pub timestamp: u64,

    /// Total value of session profit and losses (in base currency).
    pub session_profit_loss: f64,

    /// Profit and loss (in base currency; settlement and delivery only). Optional.
    #[serde(default)]
    pub profit_loss: Option<f64>,

    /// Position size (in quote currency; settlement and delivery only). Optional.
    #[serde(default)]
    pub position: Option<f64>,

    /// Mark price for at the settlement time (in quote currency; settlement and delivery only). Optional.
    #[serde(default)]
    pub mark_price: Option<f64>,

    /// Instrument name (settlement and delivery only). Optional.
    #[serde(default)]
    pub instrument_name: Option<String>,

    /// Underlying index price at time of event (in quote currency; settlement and delivery only). Optional.
    #[serde(default)]
    pub index_price: Option<f64>,

    /// Funding (in base currency; settlement for perpetual product only). Optional.
    #[serde(default)]
    pub funding: Option<f64>,

    /// Funded amount (bankruptcy only). Optional.
    #[serde(default)]
    pub funded: Option<f64>,

    /// Value of session bankruptcy (in base currency; bankruptcy only). Optional.
    #[serde(default)]
    pub session_bankruptcy: Option<f64>,

    /// Total amount of paid taxes/fees (in base currency; bankruptcy only). Optional.
    #[serde(default)]
    pub session_tax: Option<f64>,

    /// Rate of paid taxes/fees (in base currency; bankruptcy only). Optional.
    #[serde(default)]
    pub session_tax_rate: Option<f64>,

    /// The amount of the socialized losses (in base currency; bankruptcy only). Optional.
    #[serde(default)]
    pub socialized: Option<f64>,
}

/// The result object for get_last_settlements_by_currency.
#[derive(Debug, Clone, Deserialize)]
pub struct GetLastSettlementsByCurrencyResult {
    /// Continuation token for pagination. Optional.
    #[serde(default)]
    pub continuation: Option<String>,

    /// List of settlement entries.
    pub settlements: Vec<SettlementEntry>,
}

/// Response for public/get_last_settlements_by_currency endpoint following Deribit JSON-RPC 2.0 format.
pub type GetLastSettlementsByCurrencyResponse = JsonRpcResult<GetLastSettlementsByCurrencyResult>;

impl RestClient {
    /// Get last settlements by currency
    ///
    /// Retrieves historical settlement, delivery and bankruptcy events coming from all instruments within a given currency.
    ///
    /// [docs]: https://docs.deribit.com/#public-get_last_settlements_by_currency
    ///
    /// Rate limit: Non-matching engine endpoint
    ///
    /// # Arguments
    /// * `request` - The request parameters for getting last settlements
    ///
    /// # Returns
    /// Response containing settlements data and optional continuation token
    pub async fn get_last_settlements_by_currency(
        &self,
        request: GetLastSettlementsByCurrencyRequest,
    ) -> RestResult<GetLastSettlementsByCurrencyResponse> {
        self.send_request(
            LAST_SETTLEMENTS_BY_CURRENCY_ENDPOINT,
            Some(&request),
            EndpointType::NonMatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;
    use crate::deribit::enums::Currency;

    #[test]
    fn test_serialize_request() {
        let req = GetLastSettlementsByCurrencyRequest {
            currency: Currency::BTC,
            settlement_type: Some(SettlementType::Delivery),
            count: Some(5),
            continuation: None,
            search_start_timestamp: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("BTC"));
        assert!(json.contains("delivery"));
        assert!(json.contains("count"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 16,
            "jsonrpc": "2.0",
            "result": {
                "continuation": "xY7T6cutS3t2B9YtaDkE6TS379oKnkzTvmEDUnEUP2Msa9xKWNNaT",
                "settlements": [
                    {
                        "type": "delivery",
                        "timestamp": 1550242800013,
                        "session_profit_loss": 4.703907906,
                        "profit_loss": -0.427669766,
                        "position": 64,
                        "mark_price": 0.121679828,
                        "instrument_name": "BTC-15FEB19-4000-P",
                        "index_price": 3566.08
                    }
                ]
            }
        }"#;
        let resp: GetLastSettlementsByCurrencyResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 16);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.settlements.len(), 1);
        let settlement = &resp.result.settlements[0];
        assert_eq!(settlement.settlement_type, SettlementType::Delivery);
        assert_eq!(settlement.timestamp, 1550242800013);
        assert!((settlement.session_profit_loss - 4.703907906).abs() < 1e-8);
        assert_eq!(
            settlement.instrument_name.as_ref().unwrap(),
            "BTC-15FEB19-4000-P"
        );
        assert!((settlement.index_price.unwrap() - 3566.08).abs() < 1e-8);
    }

    #[test]
    fn test_deserialize_settlement_types() {
        let settlement_data = r#""settlement""#;
        let settlement: SettlementType = serde_json::from_str(settlement_data).unwrap();
        assert_eq!(settlement, SettlementType::Settlement);

        let delivery_data = r#""delivery""#;
        let delivery: SettlementType = serde_json::from_str(delivery_data).unwrap();
        assert_eq!(delivery, SettlementType::Delivery);

        let bankruptcy_data = r#""bankruptcy""#;
        let bankruptcy: SettlementType = serde_json::from_str(bankruptcy_data).unwrap();
        assert_eq!(bankruptcy, SettlementType::Bankruptcy);
    }
}
