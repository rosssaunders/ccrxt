use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::gateio::RestResult;

const DELIVERY_TICKERS_ENDPOINT: &str = "/delivery/{}/tickers";

/// Request parameters for delivery tickers
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeliveryTickersRequest {
    /// Settlement currency
    pub settle: String,

    /// Contract name (optional - if not provided, returns all contracts)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,
}

/// Delivery ticker information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryTicker {
    /// Contract name
    pub contract: String,

    /// Last trading price
    pub last: String,

    /// Recent lowest ask
    pub lowest_ask: String,

    /// Recent highest bid
    pub highest_bid: String,

    /// Change percentage (24h)
    pub change_percentage: String,

    /// Change amount (24h)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_utc0: Option<String>,

    /// Change amount (UTC 8)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_utc8: Option<String>,

    /// Total trading volume (24h)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size: Option<String>,

    /// Trading volume (24h in quote currency)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_24h: Option<String>,

    /// Trading volume (24h in base currency)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_24h_btc: Option<String>,

    /// Trading volume (24h in quote currency)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_24h_usd: Option<String>,

    /// Trading volume (24h in base currency)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_24h_base: Option<String>,

    /// Trading volume (24h in quote currency)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_24h_quote: Option<String>,

    /// Trading volume (24h in settlement currency, BTC denominated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_24h_settle: Option<String>,

    /// Mark price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_price: Option<String>,

    /// Index price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_price: Option<String>,

    /// Trading enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quanto_base_rate: Option<String>,

    /// Basis rate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basis_rate: Option<String>,

    /// Basis value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basis_value: Option<String>,
}

impl RestClient {
    /// List delivery tickers
    ///
    /// Retrieve trading info for delivery contracts
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#list-futures-tickers-2)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `params` - The delivery tickers request parameters
    ///
    /// # Returns
    /// List of delivery tickers
    pub async fn get_delivery_tickers(
        &self,
        params: DeliveryTickersRequest,
    ) -> RestResult<Vec<DeliveryTicker>> {
        let endpoint = DELIVERY_TICKERS_ENDPOINT.replace("{}", &params.settle);
        self.get_with_query(&endpoint, Some(&params)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delivery_tickers_endpoint_constant() {
        assert_eq!(DELIVERY_TICKERS_ENDPOINT, "/delivery/{}/tickers");
    }

    #[test]
    fn test_delivery_tickers_request_minimal() {
        let request = DeliveryTickersRequest {
            settle: "BTC".to_string(),
            contract: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "BTC");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 1); // Only settle
        assert!(!obj.contains_key("contract"));
    }

    #[test]
    fn test_delivery_tickers_request_with_contract() {
        let request = DeliveryTickersRequest {
            settle: "USDT".to_string(),
            contract: Some("BTC_USDT_20240315".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT_20240315");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 2);
    }

    #[test]
    fn test_delivery_tickers_request_default() {
        let request = DeliveryTickersRequest::default();
        assert_eq!(request.settle, "");
        assert_eq!(request.contract, None);
    }

    #[test]
    fn test_delivery_tickers_request_different_settlements() {
        let settlements = vec!["BTC", "USDT", "ETH"];

        for settle in settlements {
            let request = DeliveryTickersRequest {
                settle: settle.to_string(),
                contract: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["settle"], settle);
        }
    }

    #[test]
    fn test_delivery_ticker_complete_deserialization() {
        let json = r#"{
            "contract": "BTC_USDT_20240315",
            "last": "45000.50",
            "lowest_ask": "45001.00",
            "highest_bid": "44999.50",
            "change_percentage": "2.5",
            "change_utc0": "1100.25",
            "change_utc8": "1125.50",
            "total_size": "15000",
            "volume_24h": "125000000",
            "volume_24h_btc": "2780.0",
            "volume_24h_usd": "125000000",
            "volume_24h_base": "2780.0",
            "volume_24h_quote": "125000000",
            "volume_24h_settle": "2780.0",
            "mark_price": "45000.75",
            "index_price": "45001.25",
            "quanto_base_rate": "1.0",
            "basis_rate": "0.000025",
            "basis_value": "1.25"
        }"#;

        let ticker: DeliveryTicker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.contract, "BTC_USDT_20240315");
        assert_eq!(ticker.last, "45000.50");
        assert_eq!(ticker.lowest_ask, "45001.00");
        assert_eq!(ticker.highest_bid, "44999.50");
        assert_eq!(ticker.change_percentage, "2.5");
        assert_eq!(ticker.change_utc0, Some("1100.25".to_string()));
        assert_eq!(ticker.change_utc8, Some("1125.50".to_string()));
        assert_eq!(ticker.total_size, Some("15000".to_string()));
        assert_eq!(ticker.volume_24h, Some("125000000".to_string()));
        assert_eq!(ticker.volume_24h_btc, Some("2780.0".to_string()));
        assert_eq!(ticker.volume_24h_usd, Some("125000000".to_string()));
        assert_eq!(ticker.volume_24h_base, Some("2780.0".to_string()));
        assert_eq!(ticker.volume_24h_quote, Some("125000000".to_string()));
        assert_eq!(ticker.volume_24h_settle, Some("2780.0".to_string()));
        assert_eq!(ticker.mark_price, Some("45000.75".to_string()));
        assert_eq!(ticker.index_price, Some("45001.25".to_string()));
        assert_eq!(ticker.quanto_base_rate, Some("1.0".to_string()));
        assert_eq!(ticker.basis_rate, Some("0.000025".to_string()));
        assert_eq!(ticker.basis_value, Some("1.25".to_string()));
    }

    #[test]
    fn test_delivery_ticker_minimal_deserialization() {
        let json = r#"{
            "contract": "ETH_USDT_20240415",
            "last": "3500.00",
            "lowest_ask": "3501.00",
            "highest_bid": "3499.00",
            "change_percentage": "-1.2"
        }"#;

        let ticker: DeliveryTicker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.contract, "ETH_USDT_20240415");
        assert_eq!(ticker.last, "3500.00");
        assert_eq!(ticker.lowest_ask, "3501.00");
        assert_eq!(ticker.highest_bid, "3499.00");
        assert_eq!(ticker.change_percentage, "-1.2");

        // All optional fields should be None
        assert_eq!(ticker.change_utc0, None);
        assert_eq!(ticker.change_utc8, None);
        assert_eq!(ticker.total_size, None);
        assert_eq!(ticker.volume_24h, None);
        assert_eq!(ticker.volume_24h_btc, None);
        assert_eq!(ticker.volume_24h_usd, None);
        assert_eq!(ticker.volume_24h_base, None);
        assert_eq!(ticker.volume_24h_quote, None);
        assert_eq!(ticker.volume_24h_settle, None);
        assert_eq!(ticker.mark_price, None);
        assert_eq!(ticker.index_price, None);
        assert_eq!(ticker.quanto_base_rate, None);
        assert_eq!(ticker.basis_rate, None);
        assert_eq!(ticker.basis_value, None);
    }

    #[test]
    fn test_delivery_ticker_negative_change() {
        let json = r#"{
            "contract": "BTC_USDT_20240315",
            "last": "42500.00",
            "lowest_ask": "42501.00",
            "highest_bid": "42499.00",
            "change_percentage": "-5.8",
            "change_utc0": "-2610.50",
            "basis_rate": "-0.000050"
        }"#;

        let ticker: DeliveryTicker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.change_percentage, "-5.8");
        assert_eq!(ticker.change_utc0, Some("-2610.50".to_string()));
        assert_eq!(ticker.basis_rate, Some("-0.000050".to_string()));
    }

    #[test]
    fn test_delivery_ticker_high_precision_values() {
        let json = r#"{
            "contract": "BTC_USDT_20240315",
            "last": "45000.123456789",
            "lowest_ask": "45001.987654321",
            "highest_bid": "44999.555555555",
            "change_percentage": "2.123456789",
            "basis_rate": "0.000025123456",
            "basis_value": "1.987654321"
        }"#;

        let ticker: DeliveryTicker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.last, "45000.123456789");
        assert_eq!(ticker.lowest_ask, "45001.987654321");
        assert_eq!(ticker.highest_bid, "44999.555555555");
        assert_eq!(ticker.change_percentage, "2.123456789");
        assert_eq!(ticker.basis_rate, Some("0.000025123456".to_string()));
        assert_eq!(ticker.basis_value, Some("1.987654321".to_string()));
    }

    #[test]
    fn test_delivery_ticker_zero_values() {
        let json = r#"{
            "contract": "SOL_USDT_20240515",
            "last": "0.0",
            "lowest_ask": "0.0",
            "highest_bid": "0.0",
            "change_percentage": "0.0",
            "total_size": "0",
            "volume_24h": "0",
            "basis_rate": "0.0",
            "basis_value": "0.0"
        }"#;

        let ticker: DeliveryTicker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.last, "0.0");
        assert_eq!(ticker.change_percentage, "0.0");
        assert_eq!(ticker.total_size, Some("0".to_string()));
        assert_eq!(ticker.volume_24h, Some("0".to_string()));
        assert_eq!(ticker.basis_rate, Some("0.0".to_string()));
        assert_eq!(ticker.basis_value, Some("0.0".to_string()));
    }

    #[test]
    fn test_delivery_ticker_array_deserialization() {
        let json = r#"[
            {
                "contract": "BTC_USDT_20240315",
                "last": "45000.0",
                "lowest_ask": "45001.0",
                "highest_bid": "44999.0",
                "change_percentage": "2.5"
            },
            {
                "contract": "ETH_USDT_20240415",
                "last": "3500.0",
                "lowest_ask": "3501.0",
                "highest_bid": "3499.0",
                "change_percentage": "-1.2"
            }
        ]"#;

        let tickers: Vec<DeliveryTicker> = serde_json::from_str(json).unwrap();
        assert_eq!(tickers.len(), 2);

        assert_eq!(tickers[0].contract, "BTC_USDT_20240315");
        assert_eq!(tickers[0].last, "45000.0");
        assert_eq!(tickers[0].change_percentage, "2.5");

        assert_eq!(tickers[1].contract, "ETH_USDT_20240415");
        assert_eq!(tickers[1].last, "3500.0");
        assert_eq!(tickers[1].change_percentage, "-1.2");
    }

    #[test]
    fn test_delivery_ticker_serialization() {
        let ticker = DeliveryTicker {
            contract: "BTC_USDT_20240315".to_string(),
            last: "45000.0".to_string(),
            lowest_ask: "45001.0".to_string(),
            highest_bid: "44999.0".to_string(),
            change_percentage: "2.5".to_string(),
            change_utc0: Some("1100.0".to_string()),
            change_utc8: None,
            total_size: Some("15000".to_string()),
            volume_24h: Some("125000000".to_string()),
            volume_24h_btc: None,
            volume_24h_usd: Some("125000000".to_string()),
            volume_24h_base: None,
            volume_24h_quote: Some("125000000".to_string()),
            volume_24h_settle: None,
            mark_price: Some("45000.5".to_string()),
            index_price: Some("45001.0".to_string()),
            quanto_base_rate: Some("1.0".to_string()),
            basis_rate: Some("0.000025".to_string()),
            basis_value: Some("1.25".to_string()),
        };

        let json = serde_json::to_value(&ticker).unwrap();
        assert_eq!(json["contract"], "BTC_USDT_20240315");
        assert_eq!(json["last"], "45000.0");
        assert_eq!(json["change_percentage"], "2.5");
        assert_eq!(json["change_utc0"], "1100.0");
        assert!(!json.as_object().unwrap().contains_key("change_utc8"));
    }

    #[test]
    fn test_delivery_ticker_round_trip() {
        let original = DeliveryTicker {
            contract: "ETH_USDT_20240415".to_string(),
            last: "3500.75".to_string(),
            lowest_ask: "3501.25".to_string(),
            highest_bid: "3499.25".to_string(),
            change_percentage: "-1.5".to_string(),
            change_utc0: Some("-52.5".to_string()),
            change_utc8: Some("-55.0".to_string()),
            total_size: Some("8500".to_string()),
            volume_24h: Some("29750000".to_string()),
            volume_24h_btc: Some("8500.0".to_string()),
            volume_24h_usd: Some("29750000".to_string()),
            volume_24h_base: Some("8500.0".to_string()),
            volume_24h_quote: Some("29750000".to_string()),
            volume_24h_settle: Some("8500.0".to_string()),
            mark_price: Some("3500.50".to_string()),
            index_price: Some("3501.00".to_string()),
            quanto_base_rate: Some("1.0".to_string()),
            basis_rate: Some("-0.000015".to_string()),
            basis_value: Some("-0.525".to_string()),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: DeliveryTicker = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.contract, original.contract);
        assert_eq!(deserialized.last, original.last);
        assert_eq!(deserialized.change_percentage, original.change_percentage);
        assert_eq!(deserialized.change_utc0, original.change_utc0);
        assert_eq!(deserialized.change_utc8, original.change_utc8);
        assert_eq!(deserialized.mark_price, original.mark_price);
        assert_eq!(deserialized.basis_rate, original.basis_rate);
        assert_eq!(deserialized.basis_value, original.basis_value);
    }

    #[test]
    fn test_endpoint_path_construction() {
        let settlements = vec!["BTC", "USDT", "ETH"];

        for settle in settlements {
            let endpoint = DELIVERY_TICKERS_ENDPOINT.replace("{}", settle);
            assert_eq!(endpoint, format!("/delivery/{}/tickers", settle));
        }
    }

    #[test]
    fn test_delivery_ticker_different_contract_types() {
        let contracts = vec![
            "BTC_USDT_20240315",
            "ETH_BTC_20240415",
            "SOL_USDT_20240515",
            "ADA_USDT_20240615",
        ];

        for contract in contracts {
            let json = format!(
                r#"{{
                "contract": "{}",
                "last": "100.0",
                "lowest_ask": "101.0",
                "highest_bid": "99.0",
                "change_percentage": "1.0"
            }}"#,
                contract
            );

            let ticker: DeliveryTicker = serde_json::from_str(&json).unwrap();
            assert_eq!(ticker.contract, contract);
        }
    }

    #[test]
    fn test_delivery_ticker_debug_output() {
        let ticker = DeliveryTicker {
            contract: "BTC_USDT_20240315".to_string(),
            last: "45000.0".to_string(),
            lowest_ask: "45001.0".to_string(),
            highest_bid: "44999.0".to_string(),
            change_percentage: "2.5".to_string(),
            change_utc0: None,
            change_utc8: None,
            total_size: None,
            volume_24h: None,
            volume_24h_btc: None,
            volume_24h_usd: None,
            volume_24h_base: None,
            volume_24h_quote: None,
            volume_24h_settle: None,
            mark_price: None,
            index_price: None,
            quanto_base_rate: None,
            basis_rate: None,
            basis_value: None,
        };

        let debug_str = format!("{:?}", ticker);
        assert!(debug_str.contains("DeliveryTicker"));
        assert!(debug_str.contains("BTC_USDT_20240315"));
        assert!(debug_str.contains("45000.0"));
    }
}
