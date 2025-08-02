use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, private::rest::client::RestClient};

const ADL_QUANTILE_ENDPOINT: &str = "/dapi/v1/adlQuantile";

/// Request parameters for getting position ADL quantile estimation (GET /dapi/v1/adlQuantile).
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetPositionAdlQuantileRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP"). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Receive window in milliseconds.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds.
    pub timestamp: u64,
}

/// ADL quantile values for different position sides.
#[derive(Debug, Clone, Deserialize)]
pub struct AdlQuantileValues {
    /// ADL quantile for LONG position in hedge mode or position in one-way mode.
    #[serde(rename = "LONG", skip_serializing_if = "Option::is_none")]
    pub long: Option<u32>,

    /// ADL quantile for SHORT position in hedge mode.
    #[serde(rename = "SHORT", skip_serializing_if = "Option::is_none")]
    pub short: Option<u32>,

    /// ADL quantile for position in one-way mode.
    #[serde(rename = "BOTH", skip_serializing_if = "Option::is_none")]
    pub both: Option<u32>,

    /// Sign for hedge mode (only a sign, ignore the value).
    #[serde(rename = "HEDGE", skip_serializing_if = "Option::is_none")]
    pub hedge: Option<u32>,
}

/// Position ADL quantile entry for a symbol.
#[derive(Debug, Clone, Deserialize)]
pub struct PositionAdlQuantileEntry {
    /// Trading symbol.
    pub symbol: String,

    /// ADL quantile values for different position sides.
    #[serde(rename = "adlQuantile")]
    pub adl_quantile: AdlQuantileValues,
}

/// Response for getting position ADL quantile estimation (GET /dapi/v1/adlQuantile).
pub type GetPositionAdlQuantileResponse = Vec<PositionAdlQuantileEntry>;

impl RestClient {
    /// Gets position ADL quantile estimation (USER_DATA) on Binance Coin-M Futures.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Position-ADL-Quantile-Estimation
    ///
    /// GET /dapi/v1/adlQuantile
    /// Weight: 5
    /// Requires API key and signature.
    ///
    /// Query position ADL quantile estimation.
    ///
    /// - Values update every 30s.
    /// - Values 0, 1, 2, 3, 4 shows the queue position and possibility of ADL from low to high.
    /// - For positions of the symbol are in One-way Mode or isolated margined in Hedge Mode,
    ///   "LONG", "SHORT", and "BOTH" will be returned to show the positions' adl quantiles of different position sides.
    /// - If the positions of the symbol are crossed margined in Hedge Mode:
    ///   - "HEDGE" as a sign will be returned instead of "BOTH";
    ///   - A same value calculated on unrealized pnls on long and short sides' positions will be shown
    ///     for "LONG" and "SHORT" when there are positions in both of long and short sides.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`GetPositionAdlQuantileRequest`])
    ///
    /// # Returns
    /// A [`GetPositionAdlQuantileResponse`] - array of position ADL quantile entries.
    pub async fn get_position_adl_quantile(
        &self,
        params: GetPositionAdlQuantileRequest,
    ) -> RestResult<GetPositionAdlQuantileResponse> {
        let weight = 5;
        self.send_get_signed_request(ADL_QUANTILE_ENDPOINT, params, weight, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_adl_quantile_request_serialization() {
        let request = GetPositionAdlQuantileRequest {
            symbol: Some("BTCUSD_PERP".to_string()),
            recv_window: None,
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSD_PERP"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(!serialized.contains("recvWindow"));
    }

    #[test]
    fn test_position_adl_quantile_request_without_symbol() {
        let request = GetPositionAdlQuantileRequest {
            symbol: None,
            recv_window: Some(5000),
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(!serialized.contains("symbol"));
    }

    #[test]
    fn test_adl_quantile_values_one_way_mode() {
        let json = r#"{
            "BOTH": 2
        }"#;

        let values: AdlQuantileValues = serde_json::from_str(json).unwrap();
        assert_eq!(values.both, Some(2));
        assert!(values.long.is_none());
        assert!(values.short.is_none());
        assert!(values.hedge.is_none());
    }

    #[test]
    fn test_adl_quantile_values_hedge_mode_isolated() {
        let json = r#"{
            "LONG": 3,
            "SHORT": 1
        }"#;

        let values: AdlQuantileValues = serde_json::from_str(json).unwrap();
        assert_eq!(values.long, Some(3));
        assert_eq!(values.short, Some(1));
        assert!(values.both.is_none());
        assert!(values.hedge.is_none());
    }

    #[test]
    fn test_adl_quantile_values_hedge_mode_crossed() {
        let json = r#"{
            "LONG": 4,
            "SHORT": 4,
            "HEDGE": 0
        }"#;

        let values: AdlQuantileValues = serde_json::from_str(json).unwrap();
        assert_eq!(values.long, Some(4));
        assert_eq!(values.short, Some(4));
        assert!(values.both.is_none());
        assert_eq!(values.hedge, Some(0));
    }

    #[test]
    fn test_position_adl_quantile_response_deserialization() {
        let json = r#"[
            {
                "symbol": "BTCUSD_PERP",
                "adlQuantile": {
                    "BOTH": 2
                }
            },
            {
                "symbol": "ETHUSD_PERP",
                "adlQuantile": {
                    "LONG": 3,
                    "SHORT": 1
                }
            }
        ]"#;

        let response: GetPositionAdlQuantileResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 2);

        let entry1 = &response[0];
        assert_eq!(entry1.symbol, "BTCUSD_PERP");
        assert_eq!(entry1.adl_quantile.both, Some(2));
        assert!(entry1.adl_quantile.long.is_none());
        assert!(entry1.adl_quantile.short.is_none());

        let entry2 = &response[1];
        assert_eq!(entry2.symbol, "ETHUSD_PERP");
        assert_eq!(entry2.adl_quantile.long, Some(3));
        assert_eq!(entry2.adl_quantile.short, Some(1));
        assert!(entry2.adl_quantile.both.is_none());
    }

    #[test]
    fn test_empty_position_adl_quantile_response() {
        let json = r#"[]"#;
        let response: GetPositionAdlQuantileResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 0);
    }
}
