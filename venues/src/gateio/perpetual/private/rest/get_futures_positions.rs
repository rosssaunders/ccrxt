use serde::{Deserialize, Serialize};

use super::{RestClient, position::FuturesPosition};

/// Request parameters for futures positions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FuturesPositionsRequest {
    /// Settlement currency
    pub settle: String,

    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,

    /// Hold mode (0: both, 1: long only, 2: short only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holding: Option<i32>,

    /// Page number for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Offset for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

impl RestClient {
    /// Get futures positions
    ///
    /// This endpoint returns all futures positions for the authenticated user.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.io/docs/developers/apiv4/#list-all-positions-of-a-user>
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `params` - The position query parameters
    ///
    /// # Returns
    /// List of futures positions
    pub async fn get_futures_positions(
        &self,
        params: FuturesPositionsRequest,
    ) -> crate::gateio::perpetual::Result<Vec<FuturesPosition>> {
        let endpoint = format!("/futures/{}/positions", params.settle);
        self.get_with_query(&endpoint, &params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_futures_positions_request_minimal() {
        let request = FuturesPositionsRequest {
            settle: "USDT".to_string(),
            contract: None,
            holding: None,
            limit: None,
            offset: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 1); // Only settle
        assert!(!obj.contains_key("contract"));
        assert!(!obj.contains_key("holding"));
        assert!(!obj.contains_key("limit"));
        assert!(!obj.contains_key("offset"));
    }

    #[test]
    fn test_futures_positions_request_full() {
        let request = FuturesPositionsRequest {
            settle: "USDT".to_string(),
            contract: Some("BTC_USDT".to_string()),
            holding: Some(1), // Long only
            limit: Some(50),
            offset: Some(10),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT");
        assert_eq!(json["holding"], 1);
        assert_eq!(json["limit"], 50);
        assert_eq!(json["offset"], 10);

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 5);
    }

    #[test]
    fn test_holding_modes() {
        let holding_modes = vec![
            (0, "Both long and short"),
            (1, "Long only"),
            (2, "Short only"),
        ];

        for (mode, _description) in holding_modes {
            let request = FuturesPositionsRequest {
                settle: "USDT".to_string(),
                contract: None,
                holding: Some(mode),
                limit: None,
                offset: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["holding"], mode);
        }
    }

    #[test]
    fn test_contract_filters() {
        let contracts = vec!["BTC_USDT", "ETH_USDT", "SOL_USDT", "MATIC_USDT"];

        for contract in contracts {
            let request = FuturesPositionsRequest {
                settle: "USDT".to_string(),
                contract: Some(contract.to_string()),
                holding: None,
                limit: None,
                offset: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["contract"], contract);
        }
    }

    #[test]
    fn test_pagination() {
        let pagination_configs = vec![
            (10, 0, "First page with 10 items"),
            (50, 0, "First page with 50 items"),
            (100, 100, "Second page with 100 items"),
            (50, 200, "Fifth page with 50 items"),
        ];

        for (limit, offset, _description) in pagination_configs {
            let request = FuturesPositionsRequest {
                settle: "USDT".to_string(),
                contract: None,
                holding: None,
                limit: Some(limit),
                offset: Some(offset),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["limit"], limit);
            assert_eq!(json["offset"], offset);
        }
    }

    #[test]
    fn test_default_implementation() {
        let request = FuturesPositionsRequest::default();
        assert_eq!(request.settle, "");
        assert!(request.contract.is_none());
        assert!(request.holding.is_none());
        assert!(request.limit.is_none());
        assert!(request.offset.is_none());
    }

    #[test]
    fn test_serialization_omits_null() {
        let request = FuturesPositionsRequest {
            settle: "USDT".to_string(),
            contract: None,
            holding: None,
            limit: None,
            offset: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();

        // Only settle should be present
        assert_eq!(obj.len(), 1);
        assert!(obj.contains_key("settle"));
        assert!(!obj.contains_key("contract"));
        assert!(!obj.contains_key("holding"));
        assert!(!obj.contains_key("limit"));
        assert!(!obj.contains_key("offset"));
    }
}
