//! Custody withdrawal limits endpoint for Bullish Exchange API

use serde::Deserialize;

use crate::bullish::{EndpointType, PrivateRestClient as RestClient, RestResult};

/// Endpoint URL path for custody withdrawal limits
const CUSTODY_LIMITS_ENDPOINT: &str = "/v1/wallets/limits";

/// Withdrawal limits for a symbol
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustodyLimits {
    /// Asset symbol, e.g. USDC, BTC, ETH
    pub symbol: String,

    /// Remaining limit that can be withdrawn now
    pub available: String,

    /// 24-hour withdrawal limit
    #[serde(rename = "twentyFourHour")]
    pub twenty_four_hour: String,
}

impl RestClient {
    /// Get custody withdrawal limits for a symbol
    pub async fn get_custody_limits(&mut self, symbol: &str) -> RestResult<CustodyLimits> {
        let endpoint = format!("{}/{}", CUSTODY_LIMITS_ENDPOINT, symbol);
        self.send_get_request::<CustodyLimits, ()>(&endpoint, (), EndpointType::PrivateCustody)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_limits_fields() {
        let json = r#"{"symbol":"USDC","available":"20000.0","twentyFourHour":"1000000.00"}"#;
        let v: CustodyLimits = serde_json::from_str(json).unwrap();
        assert_eq!(v.symbol, "USDC");
        assert_eq!(v.available, "20000.0");
        assert_eq!(v.twenty_four_hour, "1000000.00");
    }
}
