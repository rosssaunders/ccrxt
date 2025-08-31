use serde::{Deserialize, Serialize};

use crate::bullish::{
    DataOrPaginated, EndpointType, PaginatedResult, PaginationParams, PublicRestClient, RestResult,
    enums::CandleInterval,
};

/// Endpoint URL path for candles (singular per API docs)
const CANDLES_ENDPOINT: &str = "/trading-api/v1/markets/{}/candle";

/// Request parameters for getting candles
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetCandlesRequest {
    /// The market symbol to get candles for
    #[serde(skip_serializing)]
    pub symbol: String,

    /// Candlestick interval
    #[serde(rename = "timeBucket", skip_serializing_if = "Option::is_none")]
    pub interval: Option<CandleInterval>,

    /// Filter start datetime (maps to createdAtDatetime[gte])
    #[serde(
        rename = "createdAtDatetime[gte]",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at_datetime_gte: Option<String>,

    /// Filter end datetime (maps to createdAtDatetime[lte])
    #[serde(
        rename = "createdAtDatetime[lte]",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at_datetime_lte: Option<String>,

    /// Pagination parameters (flattened into top-level query)
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

/// Candlestick data as returned by Bullish
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candle {
    /// Opening price
    pub open: String,

    /// Highest price
    pub high: String,

    /// Lowest price
    pub low: String,

    /// Closing price
    pub close: String,

    /// Volume
    pub volume: String,

    /// Candle created time as timestamp (ms since epoch) as string
    #[serde(rename = "createdAtTimestamp")]
    pub created_at_timestamp: String,

    /// Candle created time in ISO 8601 format with millis
    #[serde(rename = "createdAtDatetime")]
    pub created_at_datetime: String,

    /// Time the candle was published (ms since epoch) as string
    #[serde(rename = "publishedAtTimestamp")]
    pub published_at_timestamp: String,
}

impl PublicRestClient {
    /// Get candlestick data for a market symbol
    ///
    /// Retrieves historical candlestick data for a specific market.
    ///
    /// [docs](https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#get-/v1/markets/-symbol-/tick)
    ///
    /// # Arguments
    /// * `request` - Request parameters containing the market symbol and optional filters
    ///
    /// # Returns
    /// A `RestResult<Vec<Candle>>` containing the candlestick data
    ///
    /// # Errors
    /// Returns an error if the request fails or the response cannot be parsed
    pub async fn get_market_candle(
        &self,
        request: &GetCandlesRequest,
    ) -> RestResult<PaginatedResult<Candle>> {
        let endpoint = CANDLES_ENDPOINT.replace("{}", &request.symbol);

        // Serialize the entire request (minus symbol) into query parameters.
        let query_params = serde_urlencoded::to_string(request).unwrap_or_default();

        let full_endpoint = if query_params.is_empty() {
            endpoint
        } else {
            format!("{}?{}", endpoint, query_params)
        };

        // The API may return either a direct array or a paginated wrapper.
        // Parse flexibly and return data + optional links.
        let wire: DataOrPaginated<Candle> = self
            .send_get_request(&full_endpoint, EndpointType::PublicCandles)
            .await?;

        Ok(PaginatedResult::from(wire))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candle_deserialization() {
        let json = r#"
        {
            "open": "50000.00",
            "high": "51000.00",
            "low": "49000.00",
            "close": "50500.00",
            "volume": "100.50000000",
            "createdAtTimestamp": "1704067200000",
            "createdAtDatetime": "2024-01-01T00:00:00.000Z",
            "publishedAtTimestamp": "1704067260000"
        }
        "#;

        let candle: Candle = serde_json::from_str(json).unwrap();
        assert_eq!(candle.open, "50000.00");
        assert_eq!(candle.high, "51000.00");
        assert_eq!(candle.low, "49000.00");
        assert_eq!(candle.close, "50500.00");
    }

    #[test]
    fn test_candle_interval_serialization() {
        assert_eq!(
            serde_json::to_string(&CandleInterval::OneMinute).unwrap(),
            "\"1m\""
        );
        assert_eq!(
            serde_json::to_string(&CandleInterval::OneHour).unwrap(),
            "\"1h\""
        );
        assert_eq!(
            serde_json::to_string(&CandleInterval::OneDay).unwrap(),
            "\"1d\""
        );
    }

    #[test]
    fn test_get_candles_request_query_serialization() {
        let mut req = GetCandlesRequest {
            symbol: "BTCUSD".to_string(),
            interval: Some(CandleInterval::OneMinute),
            created_at_datetime_gte: Some("2024-01-01T00:00:00.000Z".to_string()),
            created_at_datetime_lte: Some("2024-01-01T01:00:00.000Z".to_string()),
            pagination: PaginationParams {
                page_size: Some(50),
                meta_data: Some(true),
                next_page: Some("cursor123".to_string()),
                previous_page: None,
            },
        };

        // Ensure symbol is skipped and flattened pagination fields appear
        let qs = serde_urlencoded::to_string(&req).unwrap_or_else(|_| String::new());
        assert!(!qs.contains("symbol="));
        assert!(qs.contains("timeBucket=1m"));
        assert!(qs.contains("createdAtDatetime%5Bgte%5D=2024-01-01T00%3A00%3A00.000Z"));
        assert!(qs.contains("createdAtDatetime%5Blte%5D=2024-01-01T01%3A00%3A00.000Z"));
        assert!(qs.contains("_pageSize=50"));
        assert!(qs.contains("_metaData=true"));
        assert!(qs.contains("_nextPage=cursor123"));

        // Now ensure previous_page can appear
        req.pagination.previous_page = Some("cursorPrev".to_string());
        let qs2 = serde_urlencoded::to_string(&req).unwrap_or_else(|_| String::new());
        assert!(qs2.contains("_previousPage=cursorPrev"));
    }
}
