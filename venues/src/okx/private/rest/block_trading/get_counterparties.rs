use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

/// Endpoint URL for getting RFQ counterparties
const GET_COUNTERPARTIES_ENDPOINT: &str = "api/v5/rfq/counterparties";

/// Response containing counterparty information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Counterparty {
    /// The long formative username of trader or entity on the platform
    pub trader_name: String,

    /// A unique identifier of maker which will be publicly visible on the platform.
    /// All RFQ and Quote endpoints will use this as the unique counterparty identifier
    pub trader_code: String,

    /// The counterparty type. "LP" refers to API connected auto market makers
    #[serde(rename = "type")]
    pub counterparty_type: String,
}

impl RestClient {
    /// Get counterparties
    ///
    /// Retrieves the list of counterparties that the user is permitted to trade with.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#block-trading-rest-api-get-counterparties)
    ///
    /// Rate limit: 5 requests per 2 seconds
    ///
    /// # Returns
    /// A list of available counterparties for RFQ trading
    pub async fn get_counterparties(&self) -> RestResult<Counterparty> {
        self.send_get_request(
            GET_COUNTERPARTIES_ENDPOINT,
            None::<&()>,
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::okx::response::ApiResponse;

    #[test]
    fn test_counterparty_deserialization() {
        let counterparty_json = json!({
            "traderName": "Test Trader Name",
            "traderCode": "TEST123",
            "type": "LP"
        });

        let counterparty: Counterparty = serde_json::from_value(counterparty_json).unwrap();
        assert_eq!(counterparty.trader_name, "Test Trader Name");
        assert_eq!(counterparty.trader_code, "TEST123");
        assert_eq!(counterparty.counterparty_type, "LP");
    }

    #[test]
    fn test_get_counterparties_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "traderName": "Market Maker 1",
                    "traderCode": "MM001",
                    "type": "LP"
                },
                {
                    "traderName": "Market Maker 2",
                    "traderCode": "MM002",
                    "type": "LP"
                }
            ]
        });

        let response: ApiResponse<Counterparty> = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data.first().unwrap().trader_code, "MM001");
        assert_eq!(response.data.get(1).unwrap().trader_code, "MM002");
    }

    #[test]
    fn test_counterparty_serialization_roundtrip() {
        let original = Counterparty {
            trader_name: "Test Trader".to_string(),
            trader_code: "TEST456".to_string(),
            counterparty_type: "LP".to_string(),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: Counterparty = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.trader_name, deserialized.trader_name);
        assert_eq!(original.trader_code, deserialized.trader_code);
        assert_eq!(original.counterparty_type, deserialized.counterparty_type);
    }
}
