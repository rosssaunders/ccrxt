//! Convert endpoints for Binance USDM REST API.

use chrono::Utc;
use reqwest::Method;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::binance::usdm::{
    private::rest::{client::RestClient, order::OrderErrorResponse},
    signing::sign_query,
};

/// Error type for USDM convert endpoints.
#[derive(Debug, Error, Clone, Deserialize)]
#[serde(tag = "code", content = "msg")]
pub enum ConvertError {
    /// Invalid API key or signature.
    #[error("Invalid API key or signature: {0}")]
    #[serde(rename = "-1022")]
    InvalidSignature(String),
    /// Timestamp for this request is outside of the recv window.
    #[error("Timestamp for this request is outside of the recv window: {0}")]
    #[serde(rename = "-1021")]
    TimestampOutOfRecvWindow(String),
    /// Invalid API key format.
    #[error("Invalid API key format: {0}")]
    #[serde(rename = "-2014")]
    BadApiKeyFmt(String),
    /// Invalid API key, IP, or permissions for action.
    #[error("Invalid API key, IP, or permissions for action: {0}")]
    #[serde(rename = "-2015")]
    RejectedMbxKey(String),
    /// Unknown error.
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Request for getting convert exchange info.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetConvertExchangeInfoRequest {
    /// Timestamp in milliseconds.
    pub timestamp: u64,
    /// Signature for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

/// Convert exchange info for a trading pair
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertExchangeInfo {
    /// From asset
    pub from_asset: String,
    /// To asset
    pub to_asset: String,
    /// From asset minimum amount
    pub from_asset_min_amount: String,
    /// From asset maximum amount
    pub from_asset_max_amount: String,
    /// To asset minimum amount
    pub to_asset_min_amount: String,
    /// To asset maximum amount
    pub to_asset_max_amount: String,
}

/// Response from convert exchange info endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertExchangeInfoResponse(pub Vec<ConvertExchangeInfo>);

/// Request for getting convert quote.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetConvertQuoteRequest {
    /// From asset
    pub from_asset: String,
    /// To asset
    pub to_asset: String,
    /// From amount (required if toAmount is not provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_amount: Option<String>,
    /// To amount (required if fromAmount is not provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_amount: Option<String>,
    /// Valid period in seconds for this quote
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_time: Option<u32>,
    /// Timestamp in milliseconds.
    pub timestamp: u64,
    /// Signature for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

/// Response from convert quote endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertQuoteResponse {
    /// Quote ID
    pub quote_id: String,
    /// Quote ratio
    pub ratio: String,
    /// Inverse ratio
    pub inverse_ratio: String,
    /// Valid timestamp in milliseconds
    pub valid_timestamp: u64,
    /// To amount
    pub to_amount: String,
    /// From amount
    pub from_amount: String,
}

/// Request for accepting convert quote.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptConvertQuoteRequest {
    /// Quote ID from previous quote request
    pub quote_id: String,
    /// Timestamp in milliseconds.
    pub timestamp: u64,
    /// Signature for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

/// Convert order status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConvertOrderStatus {
    #[serde(rename = "PROCESS")]
    Process,
    #[serde(rename = "ACCEPT_SUCCESS")]
    AcceptSuccess,
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAIL")]
    Fail,
}

/// Response from accept convert quote endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptConvertQuoteResponse {
    /// Order ID
    pub order_id: String,
    /// Create time in milliseconds
    pub create_time: u64,
    /// Order status
    pub order_status: ConvertOrderStatus,
}

/// Request for getting convert order status.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetConvertOrderStatusRequest {
    /// Order ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Quote ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,
    /// Timestamp in milliseconds.
    pub timestamp: u64,
    /// Signature for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

/// Response from convert order status endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertOrderStatusResponse {
    /// Order ID
    pub order_id: String,
    /// Order status
    pub order_status: ConvertOrderStatus,
    /// From asset
    pub from_asset: String,
    /// From amount
    pub from_amount: String,
    /// To asset
    pub to_asset: String,
    /// To amount
    pub to_amount: String,
    /// Ratio
    pub ratio: String,
    /// Inverse ratio
    pub inverse_ratio: String,
    /// Create time in milliseconds
    pub create_time: u64,
}

impl RestClient {
    /// Query convert exchange info.
    pub async fn get_convert_exchange_info(
        &self,
        api_key: impl Into<SecretString>,
        api_secret: impl Into<SecretString>,
    ) -> Result<ConvertExchangeInfoResponse, ConvertError> {
        // Rate limiting for private endpoints (10 weight)
        self.rate_limiter
            .acquire_request(10)
            .await
            .map_err(|e| ConvertError::Unknown(format!("Rate limiting error: {e}")))?;

        let api_key = api_key.into();
        let api_secret = api_secret.into();
        let timestamp = Utc::now().timestamp_millis() as u64;

        let mut request = GetConvertExchangeInfoRequest {
            timestamp,
            signature: None,
        };

        // Create query string for signing
        let query_string = serde_urlencoded::to_string(&request)
            .map_err(|_| ConvertError::Unknown("Failed to serialize request".to_string()))?;

        // Sign the request
        let signature = sign_query(&query_string, &api_secret);
        request.signature = Some(signature);

        // Make the request
        let response = self
            .client
            .request(
                Method::GET,
                format!("{}/fapi/v1/convert/exchangeInfo", self.base_url),
            )
            .header("X-MBX-APIKEY", api_key.expose_secret())
            .query(&request)
            .send()
            .await
            .map_err(|e| ConvertError::Unknown(e.to_string()))?;

        if response.status().is_success() {
            let exchange_info: Vec<ConvertExchangeInfo> = response
                .json()
                .await
                .map_err(|e| ConvertError::Unknown(e.to_string()))?;
            Ok(ConvertExchangeInfoResponse(exchange_info))
        } else {
            let error_response: OrderErrorResponse = response
                .json()
                .await
                .map_err(|e| ConvertError::Unknown(e.to_string()))?;
            Err(ConvertError::Unknown(error_response.msg))
        }
    }

    /// Send quote request for convert.
    pub async fn get_convert_quote(
        &self,
        api_key: impl Into<SecretString>,
        api_secret: impl Into<SecretString>,
        from_asset: String,
        to_asset: String,
        from_amount: Option<String>,
        to_amount: Option<String>,
        valid_time: Option<u32>,
    ) -> Result<ConvertQuoteResponse, ConvertError> {
        // Rate limiting for private endpoints (200 weight)
        self.rate_limiter
            .acquire_request(200)
            .await
            .map_err(|e| ConvertError::Unknown(format!("Rate limiting error: {e}")))?;

        let api_key = api_key.into();
        let api_secret = api_secret.into();
        let timestamp = Utc::now().timestamp_millis() as u64;

        let mut request = GetConvertQuoteRequest {
            from_asset,
            to_asset,
            from_amount,
            to_amount,
            valid_time,
            timestamp,
            signature: None,
        };

        // Create query string for signing
        let query_string = serde_urlencoded::to_string(&request)
            .map_err(|_| ConvertError::Unknown("Failed to serialize request".to_string()))?;

        // Sign the request
        let signature = sign_query(&query_string, &api_secret);
        request.signature = Some(signature);

        // Make the request
        let response = self
            .client
            .request(
                Method::POST,
                format!("{}/fapi/v1/convert/getQuote", self.base_url),
            )
            .header("X-MBX-APIKEY", api_key.expose_secret())
            .query(&request)
            .send()
            .await
            .map_err(|e| ConvertError::Unknown(e.to_string()))?;

        if response.status().is_success() {
            let quote_response: ConvertQuoteResponse = response
                .json()
                .await
                .map_err(|e| ConvertError::Unknown(e.to_string()))?;
            Ok(quote_response)
        } else {
            let error_response: OrderErrorResponse = response
                .json()
                .await
                .map_err(|e| ConvertError::Unknown(e.to_string()))?;
            Err(ConvertError::Unknown(error_response.msg))
        }
    }

    /// Accept quote for convert.
    pub async fn accept_convert_quote(
        &self,
        api_key: impl Into<SecretString>,
        api_secret: impl Into<SecretString>,
        quote_id: String,
    ) -> Result<AcceptConvertQuoteResponse, ConvertError> {
        // Rate limiting for private endpoints (500 weight)
        self.rate_limiter
            .acquire_request(500)
            .await
            .map_err(|e| ConvertError::Unknown(format!("Rate limiting error: {e}")))?;

        let api_key = api_key.into();
        let api_secret = api_secret.into();
        let timestamp = Utc::now().timestamp_millis() as u64;

        let mut request = AcceptConvertQuoteRequest {
            quote_id,
            timestamp,
            signature: None,
        };

        // Create query string for signing
        let query_string = serde_urlencoded::to_string(&request)
            .map_err(|_| ConvertError::Unknown("Failed to serialize request".to_string()))?;

        // Sign the request
        let signature = sign_query(&query_string, &api_secret);
        request.signature = Some(signature);

        // Make the request
        let response = self
            .client
            .request(
                Method::POST,
                format!("{}/fapi/v1/convert/acceptQuote", self.base_url),
            )
            .header("X-MBX-APIKEY", api_key.expose_secret())
            .query(&request)
            .send()
            .await
            .map_err(|e| ConvertError::Unknown(e.to_string()))?;

        if response.status().is_success() {
            let accept_response: AcceptConvertQuoteResponse = response
                .json()
                .await
                .map_err(|e| ConvertError::Unknown(e.to_string()))?;
            Ok(accept_response)
        } else {
            let error_response: OrderErrorResponse = response
                .json()
                .await
                .map_err(|e| ConvertError::Unknown(e.to_string()))?;
            Err(ConvertError::Unknown(error_response.msg))
        }
    }

    /// Get convert order status.
    pub async fn get_convert_order_status(
        &self,
        api_key: impl Into<SecretString>,
        api_secret: impl Into<SecretString>,
        order_id: Option<String>,
        quote_id: Option<String>,
    ) -> Result<ConvertOrderStatusResponse, ConvertError> {
        // Rate limiting for private endpoints (10 weight)
        self.rate_limiter
            .acquire_request(10)
            .await
            .map_err(|e| ConvertError::Unknown(format!("Rate limiting error: {e}")))?;

        let api_key = api_key.into();
        let api_secret = api_secret.into();
        let timestamp = Utc::now().timestamp_millis() as u64;

        let mut request = GetConvertOrderStatusRequest {
            order_id,
            quote_id,
            timestamp,
            signature: None,
        };

        // Create query string for signing
        let query_string = serde_urlencoded::to_string(&request)
            .map_err(|_| ConvertError::Unknown("Failed to serialize request".to_string()))?;

        // Sign the request
        let signature = sign_query(&query_string, &api_secret);
        request.signature = Some(signature);

        // Make the request
        let response = self
            .client
            .request(
                Method::GET,
                format!("{}/fapi/v1/convert/orderStatus", self.base_url),
            )
            .header("X-MBX-APIKEY", api_key.expose_secret())
            .query(&request)
            .send()
            .await
            .map_err(|e| ConvertError::Unknown(e.to_string()))?;

        if response.status().is_success() {
            let status_response: ConvertOrderStatusResponse = response
                .json()
                .await
                .map_err(|e| ConvertError::Unknown(e.to_string()))?;
            Ok(status_response)
        } else {
            let error_response: OrderErrorResponse = response
                .json()
                .await
                .map_err(|e| ConvertError::Unknown(e.to_string()))?;
            Err(ConvertError::Unknown(error_response.msg))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_exchange_info_response_deserialization() {
        let json = r#"
        [
            {
                "fromAsset": "USDT",
                "toAsset": "BNB",
                "fromAssetMinAmount": "0.1",
                "fromAssetMaxAmount": "100",
                "toAssetMinAmount": "0.001",
                "toAssetMaxAmount": "1"
            }
        ]
        "#;

        let response: Vec<ConvertExchangeInfo> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        assert_eq!(response[0].from_asset, "USDT");
        assert_eq!(response[0].to_asset, "BNB");
    }

    #[test]
    fn test_convert_quote_response_deserialization() {
        let json = r#"
        {
            "quoteId": "12415572564",
            "ratio": "95.65",
            "inverseRatio": "0.01045",
            "validTimestamp": 1625097600000,
            "toAmount": "95.65",
            "fromAmount": "1"
        }
        "#;

        let response: ConvertQuoteResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.quote_id, "12415572564");
        assert_eq!(response.ratio, "95.65");
        assert_eq!(response.valid_timestamp, 1625097600000);
    }

    #[test]
    fn test_convert_order_status_response_deserialization() {
        let json = r#"
        {
            "orderId": "933256278426274426",
            "orderStatus": "SUCCESS",
            "fromAsset": "USDT",
            "fromAmount": "20",
            "toAsset": "BNB",
            "toAmount": "0.06154036",
            "ratio": "0.00307702",
            "inverseRatio": "324.99",
            "createTime": 1624248872184
        }
        "#;

        let response: ConvertOrderStatusResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_id, "933256278426274426");
        assert_eq!(response.from_asset, "USDT");
        assert_eq!(response.to_asset, "BNB");
        matches!(response.order_status, ConvertOrderStatus::Success);
    }
}
