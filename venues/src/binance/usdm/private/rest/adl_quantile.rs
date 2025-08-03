use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::RestResult;

/// Endpoint path for ADL quantile estimation.
const ADL_QUANTILE_ENDPOINT: &str = "/fapi/v1/adlQuantile";

/// Request parameters for the ADL quantile estimation endpoint.
///
/// Retrieves ADL quantile estimation for positions on Binance USDM futures.
/// All fields are optional except `timestamp`, which is required by the API.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAdlQuantileRequest {
    /// Trading symbol (e.g., "BTCUSDT"). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Cow<'static, str>>,

    /// The number of milliseconds after epoch when the request is created. Required.
    pub timestamp: u64,

    /// The value cannot be greater than 60000. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// ADL quantile values for different position sides.
///
/// Represents the ADL quantile for LONG, SHORT, BOTH, and HEDGE positions as returned by the API.
#[derive(Debug, Clone, Deserialize)]
pub struct AdlQuantileValues {
    /// ADL quantile for LONG position in hedge mode or position in one-way mode.
    /// Value is 0-4, where higher means greater ADL risk.
    #[serde(rename = "LONG")]
    pub long: Option<u8>,

    /// ADL quantile for SHORT position in hedge mode.
    /// Value is 0-4, where higher means greater ADL risk.
    #[serde(rename = "SHORT")]
    pub short: Option<u8>,

    /// ADL quantile for position in one-way mode.
    /// Value is 0-4, where higher means greater ADL risk.
    #[serde(rename = "BOTH")]
    pub both: Option<u8>,

    /// Sign for hedge mode (only a sign, ignore the value).
    /// If present, ignore the value.
    #[serde(rename = "HEDGE")]
    pub hedge: Option<u8>,
}

/// Response for ADL quantile estimation.
///
/// Contains the symbol and its ADL quantile values.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdlQuantileResponse {
    /// Trading symbol (e.g., "BTCUSDT").
    pub symbol: String,

    /// ADL quantile values for the symbol.
    #[serde(rename = "adlQuantile")]
    pub adl_quantile: AdlQuantileValues,
}

impl UsdmClient {
    /// Position ADL Quantile Estimation
    ///
    /// Retrieves ADL quantile estimation for positions.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Position-ADL-Quantile-Estimation
    ///
    /// Rate limit: 5
    ///
    /// # Arguments
    /// * `params` - The request parameters
    ///
    /// # Returns
    /// List of ADL quantile responses for each symbol.
    pub async fn get_adl_quantile(
        &self,
        params: GetAdlQuantileRequest,
    ) -> RestResult<Vec<AdlQuantileResponse>> {
        self.send_get_signed_request(ADL_QUANTILE_ENDPOINT, params, 5, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_deserialize_adl_quantile_response_oneway() {
        let data = json!({
            "symbol": "BTCUSDT",
            "adlQuantile": {
                "LONG": 1,
                "SHORT": 2,
                "BOTH": 0
            }
        });
        let resp: AdlQuantileResponse = serde_json::from_value(data).unwrap();
        assert_eq!(resp.symbol, "BTCUSDT");
        assert_eq!(resp.adl_quantile.long, Some(1));
        assert_eq!(resp.adl_quantile.short, Some(2));
        assert_eq!(resp.adl_quantile.both, Some(0));
        assert_eq!(resp.adl_quantile.hedge, None);
    }

    #[test]
    fn test_deserialize_adl_quantile_response_hedge() {
        let data = json!({
            "symbol": "ETHUSDT",
            "adlQuantile": {
                "LONG": 3,
                "SHORT": 3,
                "HEDGE": 0
            }
        });
        let resp: AdlQuantileResponse = serde_json::from_value(data).unwrap();
        assert_eq!(resp.symbol, "ETHUSDT");
        assert_eq!(resp.adl_quantile.long, Some(3));
        assert_eq!(resp.adl_quantile.short, Some(3));
        assert_eq!(resp.adl_quantile.both, None);
        assert_eq!(resp.adl_quantile.hedge, Some(0));
    }

    #[test]
    fn test_serialize_get_adl_quantile_request() {
        let req = GetAdlQuantileRequest {
            symbol: Some(Cow::Borrowed("BTCUSDT")),
            timestamp: 1234567890,
            recv_window: Some(5000),
        };
        let ser = serde_json::to_string(&req).unwrap();
        assert!(ser.contains("BTCUSDT"));
        assert!(ser.contains("timestamp"));
        assert!(ser.contains("recvWindow"));
    }
}
