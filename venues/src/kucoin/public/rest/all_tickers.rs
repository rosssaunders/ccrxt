use serde::{Deserialize, Serialize};

use crate::kucoin::{ResponseHeaders, RestResponse, Result};

use super::RestClient;

const ALL_TICKERS_ENDPOINT: &str = "/api/v1/market/allTickers";

/// Request for getting all tickers
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAllTickersRequest {}

/// 24hr ticker statistics
#[derive(Debug, Clone, Deserialize)]
pub struct TickerStatistics {
    /// Symbol name
    pub symbol: String,

    /// Symbol name for display (optional as it might not always be present)
    pub name: Option<String>,

    /// Last traded price
    #[serde(rename = "last")]
    pub last_price: Option<String>,

    /// 24hr change percentage (optional as it might not always be present)
    #[serde(rename = "changePercentage")]
    pub change_percentage: Option<String>,

    /// 24hr change amount (optional as it might not always be present)
    #[serde(rename = "changePrice")]
    pub change_price: Option<String>,

    /// 24hr high price
    pub high: Option<String>,

    /// 24hr low price
    pub low: Option<String>,

    /// 24hr volume in base currency
    pub vol: Option<String>,

    /// 24hr volume in quote currency
    #[serde(rename = "volValue")]
    pub vol_value: Option<String>,

    /// Last trade size (optional)
    #[serde(rename = "size")]
    pub last_size: Option<String>,

    /// Timestamp of the statistics
    pub time: Option<i64>,
}

/// All tickers response wrapper
#[derive(Debug, Clone, Deserialize)]
pub struct AllTickersResponse {
    /// Server timestamp
    pub time: i64,

    /// List of all ticker statistics
    pub ticker: Vec<TickerStatistics>,
}

impl RestClient {
    /// Get 24hr ticker statistics for all symbols
    ///
    /// Reference: https://docs.kucoin.com/#get-all-tickers
    pub async fn get_all_tickers(
        &self,
        _request: GetAllTickersRequest,
    ) -> Result<(AllTickersResponse, ResponseHeaders)> {
        let (response, headers): (RestResponse<AllTickersResponse>, ResponseHeaders) =
            self.get(ALL_TICKERS_ENDPOINT, None).await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_tickers_request_default() {
        let request = GetAllTickersRequest::default();
        // Just verify it can be created
        assert_eq!(format!("{:?}", request), "GetAllTickersRequest");
    }
}
