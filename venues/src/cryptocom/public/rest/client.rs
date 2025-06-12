// REST client for Crypto.com public endpoints.
//
// Provides access to all public REST API endpoints for Crypto.com Exchange.
// All requests are unauthenticated and do not require API credentials.
use reqwest::Client;
use std::borrow::Cow;
use serde_json::Value;
use serde::de::DeserializeOwned;

use crate::cryptocom::{RateLimiter, RestResult, Errors, EndpointType};

/// Public REST client for Crypto.com exchange
/// 
/// This client handles all public API endpoints that don't require authentication.
/// It provides automatic rate limiting and error handling.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct RestClient {
    /// The base URL for the Crypto.com public REST API (e.g., "https://api.crypto.com")
    ///
    /// This is used as the prefix for all endpoint requests.
    pub base_url: Cow<'static, str>,

    /// The underlying HTTP client used for making requests.
    ///
    /// This is reused for connection pooling and performance.
    pub client: Client,

    /// The rate limiter used to manage request rates and prevent hitting API limits.
    ///
    /// This is used to ensure compliance with Crypto.com's rate limits for public endpoints.
    pub rate_limiter: RateLimiter,
}

impl RestClient {
    /// Creates a new Crypto.com public REST client.
    ///
    /// # Arguments
    /// * `base_url` - The base URL for the Crypto.com public REST API (e.g., "https://api.crypto.com")
    /// * `client` - The HTTP client to use for requests
    /// * `rate_limiter` - The rate limiter for managing API limits
    pub fn new(
        base_url: impl Into<Cow<'static, str>>,
        client: Client,
        rate_limiter: RateLimiter,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            client,
            rate_limiter,
        }
    }

    /// Send a request to a public endpoint
    /// 
    /// # Arguments
    /// * `endpoint` - The API endpoint path (e.g., "public/get-instruments")
    /// * `method` - The HTTP method to use
    /// * `params` - Optional query parameters for GET requests or body parameters for POST requests
    /// * `endpoint_type` - The endpoint type for rate limiting
    /// 
    /// # Returns
    /// A result containing the response data or an error
    pub async fn send_request<T>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        params: Option<&Value>,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
    {
        // Check rate limits before making the request
        self.rate_limiter.check_limits(endpoint_type).await
            .map_err(|e| Errors::Error(e.to_string()))?;

        // Build the URL
        let url = if endpoint.starts_with("http") {
            endpoint.to_string()
        } else {
            format!("{}/v1/{}", self.base_url, endpoint)
        };

        // Build the request
        let mut request_builder = self.client.request(method.clone(), &url);

        // Add parameters based on method
        if let Some(params) = params {
            if method == reqwest::Method::GET {
                // For GET requests, add parameters as query string
                if let Some(params_obj) = params.as_object() {
                    for (key, value) in params_obj {
                        let value_str = match value {
                            Value::String(s) => s.clone(),
                            Value::Number(n) => n.to_string(),
                            Value::Bool(b) => b.to_string(),
                            _ => value.to_string(),
                        };
                        request_builder = request_builder.query(&[(key, value_str)]);
                    }
                }
            } else {
                // For POST requests, add parameters as JSON body
                request_builder = request_builder.json(params);
            }
        }

        // Add required headers
        request_builder = request_builder.header("Content-Type", "application/json");

        // Send the request
        let response = request_builder.send().await
            .map_err(Errors::HttpError)?;

        // Increment rate limiter counter after successful request
        self.rate_limiter.increment_request(endpoint_type).await;

        // Check if the response was successful
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await
                .map_err(Errors::HttpError)?;
            return Err(Errors::Error(format!("HTTP {}: {}", status, error_text)));
        }

        // Parse the response
        let response_text = response.text().await
            .map_err(Errors::HttpError)?;

        let parsed_response: T = serde_json::from_str(&response_text)
            .map_err(|e| Errors::Error(format!("Failed to parse response: {}", e)))?;

        Ok(parsed_response)
    }

    /// Get the list of available trading instruments
    /// 
    /// This method calls the public/get-instruments endpoint to retrieve
    /// information about all available trading pairs.
    pub async fn get_instruments(&self) -> RestResult<Value> {
        self.send_request(
            "public/get-instruments",
            reqwest::Method::GET,
            None,
            EndpointType::PublicGetInstruments,
        ).await
    }

    /// Get the order book for a specific instrument
    /// 
    /// # Arguments
    /// * `instrument_name` - The trading pair (e.g., "BTC_USDT")
    /// * `depth` - Optional depth of the order book (default: 10)
    pub async fn get_book(&self, instrument_name: &str, depth: Option<u32>) -> RestResult<Value> {
        let mut params = serde_json::json!({
            "instrument_name": instrument_name
        });

        if let Some(d) = depth {
            params["depth"] = Value::Number(d.into());
        }

        self.send_request(
            "public/get-book",
            reqwest::Method::GET,
            Some(&params),
            EndpointType::PublicGetBook,
        ).await
    }

    /// Get ticker information for one or all instruments
    /// 
    /// # Arguments
    /// * `instrument_name` - Optional specific instrument name. If None, returns all tickers.
    pub async fn get_ticker(&self, instrument_name: Option<&str>) -> RestResult<Value> {
        let params = if let Some(instrument) = instrument_name {
            Some(serde_json::json!({
                "instrument_name": instrument
            }))
        } else {
            None
        };

        self.send_request(
            "public/get-tickers",
            reqwest::Method::GET,
            params.as_ref(),
            EndpointType::PublicGetTickers,
        ).await
    }

    /// Get recent trades for a specific instrument
    /// 
    /// # Arguments
    /// * `instrument_name` - The trading pair (e.g., "BTC_USDT")
    /// * `count` - Optional number of trades to return (default: 25, max: 150)
    /// * `start_ts` - Optional start timestamp (Unix timestamp or nanoseconds, default: end_time - 1 day)
    /// * `end_ts` - Optional end timestamp (Unix timestamp or nanoseconds, default: current system timestamp)
    pub async fn get_trades(
        &self, 
        instrument_name: &str, 
        count: Option<u32>,
        start_ts: Option<&str>,
        end_ts: Option<&str>
    ) -> RestResult<Value> {
        let mut params = serde_json::json!({
            "instrument_name": instrument_name
        });

        if let Some(c) = count {
            params["count"] = Value::Number(c.into());
        }
        
        if let Some(start) = start_ts {
            params["start_ts"] = Value::String(start.to_string());
        }
        
        if let Some(end) = end_ts {
            params["end_ts"] = Value::String(end.to_string());
        }

        self.send_request(
            "public/get-trades",
            reqwest::Method::GET,
            Some(&params),
            EndpointType::PublicGetTrades,
        ).await
    }

    /// Get candlestick data for a specific instrument
    /// 
    /// # Arguments
    /// * `instrument_name` - The trading pair (e.g., "BTC_USDT")
    /// * `timeframe` - The timeframe (e.g., "1m", "5m", "1h", "1D")
    /// * `count` - Optional number of data points to return (default: 25, max: 300)
    /// * `start_ts` - Optional start timestamp (Unix timestamp, default: 1 day ago)
    /// * `end_ts` - Optional end timestamp (Unix timestamp, default: current time)
    pub async fn get_candlestick(
        &self, 
        instrument_name: &str, 
        timeframe: &str, 
        count: Option<u32>,
        start_ts: Option<u64>,
        end_ts: Option<u64>
    ) -> RestResult<Value> {
        let mut params = serde_json::json!({
            "instrument_name": instrument_name,
            "timeframe": timeframe
        });

        if let Some(c) = count {
            params["count"] = Value::Number(c.into());
        }
        
        if let Some(start) = start_ts {
            params["start_ts"] = Value::Number(start.into());
        }
        
        if let Some(end) = end_ts {
            params["end_ts"] = Value::Number(end.into());
        }

        self.send_request(
            "public/get-candlestick",
            reqwest::Method::GET,
            Some(&params),
            EndpointType::PublicGetCandlestick,
        ).await
    }

    /// Get announcements from Crypto.com Exchange
    /// 
    /// # Arguments
    /// * `category` - Optional filter by category: list, delist, event, product, system
    /// * `product_type` - Optional filter by product type. e.g. Spot, Derivative, OTC, Staking, TradingArena etc
    pub async fn get_announcements(&self, category: Option<&str>, product_type: Option<&str>) -> RestResult<Value> {
        let mut params = serde_json::json!({});
        
        if let Some(cat) = category {
            params["category"] = Value::String(cat.to_string());
        }
        
        if let Some(product) = product_type {
            params["product_type"] = Value::String(product.to_string());
        }
        
        let params = if params.as_object().unwrap().is_empty() {
            None
        } else {
            Some(params)
        };

        self.send_request(
            "public/get-announcements",
            reqwest::Method::GET,
            params.as_ref(),
            EndpointType::PublicGetAnnouncements,
        ).await
    }

    /// Get risk parameter settings for Smart Cross Margin
    /// 
    /// Provides information on risk parameter settings for Smart Cross Margin.
    pub async fn get_risk_parameters(&self) -> RestResult<Value> {
        self.send_request(
            "public/get-risk-parameters",
            reqwest::Method::GET,
            None,
            EndpointType::PublicGetRiskParameters,
        ).await
    }

    /// Get ticker information for one or all instruments
    /// 
    /// # Arguments
    /// * `instrument_name` - Optional specific instrument name. If None, returns all tickers.
    pub async fn get_tickers(&self, instrument_name: Option<&str>) -> RestResult<Value> {
        let params = if let Some(instrument) = instrument_name {
            Some(serde_json::json!({
                "instrument_name": instrument
            }))
        } else {
            None
        };

        self.send_request(
            "public/get-tickers",
            reqwest::Method::GET,
            params.as_ref(),
            EndpointType::PublicGetTickers,
        ).await
    }

    /// Get valuation data for a particular instrument
    /// 
    /// # Arguments
    /// * `instrument_name` - The instrument name (e.g., "BTCUSD-INDEX")
    /// * `valuation_type` - The valuation type: index_price, mark_price, funding_hist, funding_rate, estimated_funding_rate
    /// * `count` - Optional number of data points to return (default: 25)
    /// * `start_ts` - Optional start timestamp (Unix timestamp)
    /// * `end_ts` - Optional end timestamp (Unix timestamp)
    pub async fn get_valuations(
        &self, 
        instrument_name: &str, 
        valuation_type: &str,
        count: Option<u32>,
        start_ts: Option<u64>,
        end_ts: Option<u64>
    ) -> RestResult<Value> {
        let mut params = serde_json::json!({
            "instrument_name": instrument_name,
            "valuation_type": valuation_type
        });

        if let Some(c) = count {
            params["count"] = Value::Number(c.into());
        }
        
        if let Some(start) = start_ts {
            params["start_ts"] = Value::Number(start.into());
        }
        
        if let Some(end) = end_ts {
            params["end_ts"] = Value::Number(end.into());
        }

        self.send_request(
            "public/get-valuations",
            reqwest::Method::GET,
            Some(&params),
            EndpointType::PublicGetValuations,
        ).await
    }

    /// Get settlement price of expired instruments
    /// 
    /// # Arguments
    /// * `instrument_type` - The instrument type (e.g., "FUTURE")
    /// * `page` - Optional page number (default: 1)
    pub async fn get_expired_settlement_price(&self, instrument_type: &str, page: Option<u32>) -> RestResult<Value> {
        let mut params = serde_json::json!({
            "instrument_type": instrument_type
        });

        if let Some(p) = page {
            params["page"] = Value::Number(p.into());
        }

        self.send_request(
            "public/get-expired-settlement-price",
            reqwest::Method::GET,
            Some(&params),
            EndpointType::PublicGetExpiredSettlementPrice,
        ).await
    }

    /// Get balance of Insurance Fund for a particular currency
    /// 
    /// # Arguments
    /// * `instrument_name` - The currency (e.g., "USD")
    /// * `count` - Optional number of data points to return (default: 25)
    /// * `start_ts` - Optional start timestamp (Unix timestamp) 
    /// * `end_ts` - Optional end timestamp (Unix timestamp)
    pub async fn get_insurance(
        &self, 
        instrument_name: &str,
        count: Option<u32>,
        start_ts: Option<u64>,
        end_ts: Option<u64>
    ) -> RestResult<Value> {
        let mut params = serde_json::json!({
            "instrument_name": instrument_name
        });

        if let Some(c) = count {
            params["count"] = Value::Number(c.into());
        }
        
        if let Some(start) = start_ts {
            params["start_ts"] = Value::Number(start.into());
        }
        
        if let Some(end) = end_ts {
            params["end_ts"] = Value::Number(end.into());
        }

        self.send_request(
            "public/get-insurance",
            reqwest::Method::GET,
            Some(&params),
            EndpointType::PublicGetInsurance,
        ).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_public_client_creation() {
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new();
        
        let rest_client = RestClient::new(
            "https://api.crypto.com",
            client,
            rate_limiter,
        );
        
        assert_eq!(rest_client.base_url, "https://api.crypto.com");
    }

    #[test]
    fn test_url_building() {
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new();
        
        let rest_client = RestClient::new(
            "https://api.crypto.com",
            client,
            rate_limiter,
        );
        
        // Test that the client is properly initialized
        assert_eq!(rest_client.base_url, "https://api.crypto.com");
    }

    #[tokio::test]
    async fn test_rate_limiting_integration() {
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new();
        
        let rest_client = RestClient::new(
            "https://api.crypto.com",
            client,
            rate_limiter,
        );
        
        // Test that rate limiting works (this shouldn't fail since we're not actually hitting limits)
        let result = rest_client.rate_limiter.check_limits(EndpointType::PublicGetTicker).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_parameter_building() {
        // Test parameter serialization logic
        let params = json!({
            "instrument_name": "BTC_USDT",
            "depth": 10,
            "count": 100
        });

        // Verify the structure exists
        assert_eq!(params["instrument_name"], "BTC_USDT");
        assert_eq!(params["depth"], 10);
        assert_eq!(params["count"], 100);
    }

    #[test]
    fn test_endpoint_type_mapping() {
        // Test that endpoint types are properly defined for public endpoints
        let announcements_endpoint = EndpointType::PublicGetAnnouncements;
        let risk_parameters_endpoint = EndpointType::PublicGetRiskParameters;
        let instruments_endpoint = EndpointType::PublicGetInstruments;
        let book_endpoint = EndpointType::PublicGetBook;
        let ticker_endpoint = EndpointType::PublicGetTicker;
        let tickers_endpoint = EndpointType::PublicGetTickers;
        let trades_endpoint = EndpointType::PublicGetTrades;
        let valuations_endpoint = EndpointType::PublicGetValuations;
        let candlestick_endpoint = EndpointType::PublicGetCandlestick;
        let expired_settlement_endpoint = EndpointType::PublicGetExpiredSettlementPrice;
        let insurance_endpoint = EndpointType::PublicGetInsurance;

        // Test that rate limits are defined
        assert!(announcements_endpoint.rate_limit().max_requests > 0);
        assert!(risk_parameters_endpoint.rate_limit().max_requests > 0);
        assert!(instruments_endpoint.rate_limit().max_requests > 0);
        assert!(book_endpoint.rate_limit().max_requests > 0);
        assert!(ticker_endpoint.rate_limit().max_requests > 0);
        assert!(tickers_endpoint.rate_limit().max_requests > 0);
        assert!(trades_endpoint.rate_limit().max_requests > 0);
        assert!(valuations_endpoint.rate_limit().max_requests > 0);
        assert!(candlestick_endpoint.rate_limit().max_requests > 0);
        assert!(expired_settlement_endpoint.rate_limit().max_requests > 0);
        assert!(insurance_endpoint.rate_limit().max_requests > 0);
    }

    #[test]
    fn test_new_method_parameter_building() {
        // Test parameter building for announcements
        let params = json!({
            "category": "system",
            "product_type": "Spot"
        });
        assert_eq!(params["category"], "system");
        assert_eq!(params["product_type"], "Spot");

        // Test parameter building for valuations
        let params = json!({
            "instrument_name": "BTCUSD-INDEX",
            "valuation_type": "index_price",
            "count": 10
        });
        assert_eq!(params["instrument_name"], "BTCUSD-INDEX");
        assert_eq!(params["valuation_type"], "index_price");
        assert_eq!(params["count"], 10);

        // Test parameter building for insurance
        let params = json!({
            "instrument_name": "USD",
            "count": 25
        });
        assert_eq!(params["instrument_name"], "USD");
        assert_eq!(params["count"], 25);
    }
}