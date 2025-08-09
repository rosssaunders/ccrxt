use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

/// Request for getting 24hr ticker statistics for a symbol
#[derive(Debug, Clone, Serialize)]
pub struct GetTickerRequest {
    /// Trading symbol (e.g., "BTC-USDT")
    pub symbol: String,
}

/// 24hr ticker statistics
#[derive(Debug, Clone, Deserialize)]
pub struct TickerStatistics {
    /// Timestamp
    pub time: i64,

    /// Symbol name
    pub symbol: String,

    /// Best bid price
    pub buy: String,

    /// Best ask price
    pub sell: String,

    /// 24h change rate
    #[serde(rename = "changeRate")]
    pub change_rate: String,

    /// 24h change price
    #[serde(rename = "changePrice")]
    pub change_price: String,

    /// 24hr high price
    pub high: String,

    /// 24hr low price
    pub low: String,

    /// 24hr volume in base currency
    pub vol: String,

    /// 24hr volume in quote currency
    #[serde(rename = "volValue")]
    pub vol_value: String,

    /// Last traded price
    pub last: String,

    /// Average trading price in 24h
    #[serde(rename = "averagePrice")]
    pub average_price: String,

    /// Taker fee rate
    #[serde(rename = "takerFeeRate")]
    pub taker_fee_rate: String,

    /// Maker fee rate
    #[serde(rename = "makerFeeRate")]
    pub maker_fee_rate: String,

    /// Taker coefficient
    #[serde(rename = "takerCoefficient")]
    pub taker_coefficient: String,

    /// Maker coefficient
    #[serde(rename = "makerCoefficient")]
    pub maker_coefficient: String,
}

impl RestClient {
    /// Get 24hr ticker statistics for a specific symbol
    ///
    /// Reference: https://www.kucoin.com/docs-new/rest/spot-trading/market-data/get-ticker
    pub async fn get_ticker(
        &self,
        request: GetTickerRequest,
    ) -> Result<(TickerStatistics, ResponseHeaders)> {
        let (response, headers): (RestResponse<TickerStatistics>, ResponseHeaders) = self
            .get_with_request("/api/v1/market/stats", &request)
            .await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ticker_request_creation() {
        let request = GetTickerRequest {
            symbol: "BTC-USDT".to_string(),
        };
        assert_eq!(request.symbol, "BTC-USDT");
    }
}
