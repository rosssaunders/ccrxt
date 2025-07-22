use serde::Serialize;

use super::RestClient;
use super::position::OptionsPosition;

/// Request parameters for options positions
#[derive(Debug, Clone, Serialize, Default)]
pub struct OptionsPositionsRequest {
    /// Underlying asset filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,

    /// Maximum number of records to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Page offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

impl RestClient {
    /// Get options positions
    ///
    /// This endpoint returns all options positions for the authenticated user.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.io/docs/developers/apiv4/#list-options-positions>
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `params` - The options positions request parameters
    ///
    /// # Returns
    /// List of options positions
    pub async fn get_options_positions(
        &self,
        params: OptionsPositionsRequest,
    ) -> crate::gateio::options::Result<Vec<OptionsPosition>> {
        self.get_with_query("/options/positions", &params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options_positions_request_minimal() {
        let request = OptionsPositionsRequest::default();

        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty());
    }

    #[test]
    fn test_options_positions_request_with_underlying() {
        let request = OptionsPositionsRequest {
            underlying: Some("BTC_USDT".to_string()),
            limit: None,
            offset: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["underlying"], "BTC_USDT");
        assert!(!json.as_object().unwrap().contains_key("limit"));
        assert!(!json.as_object().unwrap().contains_key("offset"));
    }

    #[test]
    fn test_options_positions_request_with_pagination() {
        let request = OptionsPositionsRequest {
            underlying: None,
            limit: Some(50),
            offset: Some(100),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert!(!json.as_object().unwrap().contains_key("underlying"));
        assert_eq!(json["limit"], 50);
        assert_eq!(json["offset"], 100);
    }

    #[test]
    fn test_options_positions_request_full() {
        let request = OptionsPositionsRequest {
            underlying: Some("ETH_USDT".to_string()),
            limit: Some(25),
            offset: Some(50),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["underlying"], "ETH_USDT");
        assert_eq!(json["limit"], 25);
        assert_eq!(json["offset"], 50);
    }

    #[test]
    fn test_options_positions_request_different_underlyings() {
        let underlyings = vec!["BTC_USDT", "ETH_USDT", "BNB_USDT", "SOL_USDT"];
        
        for underlying in underlyings {
            let request = OptionsPositionsRequest {
                underlying: Some(underlying.to_string()),
                limit: None,
                offset: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["underlying"], underlying);
        }
    }

    #[test]
    fn test_options_positions_request_pagination_limits() {
        let limits = vec![1, 10, 50, 100, 500, 1000];
        
        for limit in limits {
            let request = OptionsPositionsRequest {
                underlying: None,
                limit: Some(limit),
                offset: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["limit"], limit);
        }
    }

    #[test]
    fn test_options_positions_request_pagination_offsets() {
        let offsets = vec![0, 10, 50, 100, 500, 1000, 5000];
        
        for offset in offsets {
            let request = OptionsPositionsRequest {
                underlying: None,
                limit: None,
                offset: Some(offset),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["offset"], offset);
        }
    }

    #[test]
    fn test_options_positions_request_negative_values() {
        let request = OptionsPositionsRequest {
            underlying: None,
            limit: Some(-10),
            offset: Some(-20),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["limit"], -10);
        assert_eq!(json["offset"], -20);
    }

    #[test]
    fn test_options_positions_request_extreme_values() {
        let request = OptionsPositionsRequest {
            underlying: Some("BTC_USDT".to_string()),
            limit: Some(i32::MAX),
            offset: Some(i32::MAX),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["underlying"], "BTC_USDT");
        assert_eq!(json["limit"], i32::MAX);
        assert_eq!(json["offset"], i32::MAX);
    }

    #[test]
    fn test_options_positions_request_serialization_omits_none() {
        let request = OptionsPositionsRequest {
            underlying: Some("ETH_USDT".to_string()),
            limit: None,
            offset: Some(50),
        };

        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 2); // Only underlying and offset
        assert!(obj.contains_key("underlying"));
        assert!(obj.contains_key("offset"));
        assert!(!obj.contains_key("limit"));
    }
}