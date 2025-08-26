use rust_decimal::Decimal;
use serde::Deserialize;

use crate::binance::options::PublicRestClient as RestClient;
use crate::binance::options::{OptionsContractType, RestResult};

const EXCHANGE_INFO_ENDPOINT: &str = "/eapi/v1/exchangeInfo";

/// Exchange information response
#[derive(Debug, Clone, Deserialize)]
pub struct ExchangeInfoResponse {
    /// Time zone used by the server
    #[serde(rename = "timezone")]
    pub timezone: String,

    /// Current system time
    #[serde(rename = "serverTime")]
    pub server_time: u64,

    /// Option contract underlying asset info
    #[serde(rename = "optionContracts")]
    pub option_contracts: Vec<OptionContract>,

    /// Option asset info
    #[serde(rename = "optionAssets")]
    pub option_assets: Vec<OptionAsset>,

    /// Option trading pair info
    #[serde(rename = "optionSymbols")]
    pub option_symbols: Vec<OptionSymbol>,

    /// Rate limits
    #[serde(rename = "rateLimits")]
    pub rate_limits: Vec<RateLimit>,
}

/// Option contract underlying asset information
#[derive(Debug, Clone, Deserialize)]
pub struct OptionContract {
    /// Base currency
    #[serde(rename = "baseAsset")]
    pub base_asset: String,

    /// Quotation asset
    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,

    /// Name of the underlying asset of the option contract
    #[serde(rename = "underlying")]
    pub underlying: String,

    /// Settlement currency
    #[serde(rename = "settleAsset")]
    pub settle_asset: String,
}

/// Option asset information
#[derive(Debug, Clone, Deserialize)]
pub struct OptionAsset {
    /// Asset name
    #[serde(rename = "name")]
    pub name: String,
}

/// Option trading pair information
#[derive(Debug, Clone, Deserialize)]
pub struct OptionSymbol {
    /// Expiry time
    #[serde(rename = "expiryDate")]
    pub expiry_date: u64,

    /// Filters
    #[serde(rename = "filters")]
    pub filters: Vec<Filter>,

    /// Trading pair name
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Direction: CALL long, PUT short
    #[serde(rename = "side")]
    pub side: OptionsContractType,

    /// Strike price
    #[serde(rename = "strikePrice")]
    pub strike_price: Decimal,

    /// Underlying asset of the contract
    #[serde(rename = "underlying")]
    pub underlying: String,

    /// Contract unit, the quantity of the underlying asset represented by a single contract
    #[serde(rename = "unit")]
    pub unit: u32,

    /// Maker commission rate
    #[serde(rename = "makerFeeRate")]
    pub maker_fee_rate: Decimal,

    /// Taker commission rate
    #[serde(rename = "takerFeeRate")]
    pub taker_fee_rate: Decimal,

    /// Minimum order quantity
    #[serde(rename = "minQty")]
    pub min_qty: Decimal,

    /// Maximum order quantity
    #[serde(rename = "maxQty")]
    pub max_qty: Decimal,

    /// Initial Margin Ratio
    #[serde(rename = "initialMargin")]
    pub initial_margin: Decimal,

    /// Maintenance Margin Ratio
    #[serde(rename = "maintenanceMargin")]
    pub maintenance_margin: Decimal,

    /// Min Initial Margin Ratio
    #[serde(rename = "minInitialMargin")]
    pub min_initial_margin: Decimal,

    /// Min Maintenance Margin Ratio
    #[serde(rename = "minMaintenanceMargin")]
    pub min_maintenance_margin: Decimal,

    /// Price precision
    #[serde(rename = "priceScale")]
    pub price_scale: u32,

    /// Quantity precision
    #[serde(rename = "quantityScale")]
    pub quantity_scale: u32,

    /// Quotation asset
    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,
}

/// Trading rule filter
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "filterType")]
pub enum Filter {
    #[serde(rename = "PRICE_FILTER")]
    PriceFilter {
        #[serde(rename = "minPrice")]
        min_price: Decimal,
        #[serde(rename = "maxPrice")]
        max_price: Decimal,
        #[serde(rename = "tickSize")]
        tick_size: Decimal,
    },
    #[serde(rename = "LOT_SIZE")]
    LotSize {
        #[serde(rename = "minQty")]
        min_qty: Decimal,
        #[serde(rename = "maxQty")]
        max_qty: Decimal,
        #[serde(rename = "stepSize")]
        step_size: Decimal,
    },
}

/// Rate limit information
#[derive(Debug, Clone, Deserialize)]
pub struct RateLimit {
    /// Rate limit type
    #[serde(rename = "rateLimitType")]
    pub rate_limit_type: String,

    /// Interval type
    #[serde(rename = "interval")]
    pub interval: String,

    /// Interval number
    #[serde(rename = "intervalNum")]
    pub interval_num: u32,

    /// Limit value
    #[serde(rename = "limit")]
    pub limit: u32,
}

impl RestClient {
    /// Get exchange information
    ///
    /// Returns current exchange trading rules and symbol information.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/option/market-data/Exchange-Information)
    ///
    /// Method: GET /eapi/v1/exchangeInfo
    /// Weight: 1
    /// Security: None
    pub async fn get_exchange_info(&self) -> RestResult<ExchangeInfoResponse> {
        self.send_get_request(EXCHANGE_INFO_ENDPOINT, None::<()>, 1)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;
    use crate::binance::options::OptionsContractType;

    #[test]
    fn test_exchange_info_response_deserialization() {
        let json = r#"{
            "timezone": "UTC",
            "serverTime": 1625184000000,
            "optionContracts": [
                {
                    "baseAsset": "BTC",
                    "quoteAsset": "USDT",
                    "underlying": "BTCUSDT",
                    "settleAsset": "USDT"
                }
            ],
            "optionAssets": [
                {
                    "name": "BTC"
                },
                {
                    "name": "USDT"
                }
            ],
            "optionSymbols": [
                {
                    "expiryDate": 1640995200000,
                    "filters": [
                        {
                            "filterType": "PRICE_FILTER",
                            "minPrice": "0.01",
                            "maxPrice": "100000",
                            "tickSize": "0.01"
                        }
                    ],
                    "symbol": "BTC-231231-50000-C",
                    "side": "CALL",
                    "strikePrice": "50000",
                    "underlying": "BTCUSDT",
                    "unit": 1,
                    "makerFeeRate": "0.0003",
                    "takerFeeRate": "0.0003",
                    "minQty": "0.01",
                    "maxQty": "1000",
                    "initialMargin": "0.15",
                    "maintenanceMargin": "0.075",
                    "minInitialMargin": "0.15",
                    "minMaintenanceMargin": "0.075",
                    "priceScale": 4,
                    "quantityScale": 2,
                    "quoteAsset": "USDT"
                }
            ],
            "rateLimits": [
                {
                    "rateLimitType": "REQUEST_WEIGHT",
                    "interval": "MINUTE",
                    "intervalNum": 1,
                    "limit": 2400
                }
            ]
        }"#;

        let response: ExchangeInfoResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.timezone, "UTC");
        assert_eq!(response.server_time, 1625184000000);
        assert_eq!(response.option_contracts.len(), 1);
        assert_eq!(response.option_assets.len(), 2);
        assert_eq!(response.option_symbols.len(), 1);
        assert_eq!(response.rate_limits.len(), 1);
    }

    #[test]
    fn test_option_contract_deserialization() {
        let json = r#"{
            "baseAsset": "ETH",
            "quoteAsset": "USDT",
            "underlying": "ETHUSDT",
            "settleAsset": "USDT"
        }"#;

        let contract: OptionContract = serde_json::from_str(json).unwrap();
        assert_eq!(contract.base_asset, "ETH");
        assert_eq!(contract.quote_asset, "USDT");
        assert_eq!(contract.underlying, "ETHUSDT");
        assert_eq!(contract.settle_asset, "USDT");
    }

    #[test]
    fn test_option_asset_deserialization() {
        let json = r#"{
            "name": "BTC"
        }"#;

        let asset: OptionAsset = serde_json::from_str(json).unwrap();
        assert_eq!(asset.name, "BTC");
    }

    #[test]
    fn test_option_symbol_call_deserialization() {
        let json = r#"{
            "expiryDate": 1640995200000,
            "filters": [],
            "symbol": "BTC-231231-50000-C",
            "side": "CALL",
            "strikePrice": "50000.00",
            "underlying": "BTCUSDT",
            "unit": 1,
            "makerFeeRate": "0.0003",
            "takerFeeRate": "0.0003",
            "minQty": "0.01",
            "maxQty": "1000.00",
            "initialMargin": "0.15",
            "maintenanceMargin": "0.075",
            "minInitialMargin": "0.15",
            "minMaintenanceMargin": "0.075",
            "priceScale": 4,
            "quantityScale": 2,
            "quoteAsset": "USDT"
        }"#;

        let symbol: OptionSymbol = serde_json::from_str(json).unwrap();
        assert_eq!(symbol.expiry_date, 1640995200000);
        assert_eq!(symbol.symbol, "BTC-231231-50000-C");
        assert_eq!(symbol.side, OptionsContractType::Call);
        assert_eq!(symbol.strike_price, dec!(50000.00));
        assert_eq!(symbol.underlying, "BTCUSDT");
        assert_eq!(symbol.unit, 1);
        assert_eq!(symbol.maker_fee_rate, dec!(0.0003));
        assert_eq!(symbol.taker_fee_rate, dec!(0.0003));
        assert_eq!(symbol.min_qty, dec!(0.01));
        assert_eq!(symbol.max_qty, dec!(1000.00));
        assert_eq!(symbol.initial_margin, dec!(0.15));
        assert_eq!(symbol.maintenance_margin, dec!(0.075));
        assert_eq!(symbol.min_initial_margin, dec!(0.15));
        assert_eq!(symbol.min_maintenance_margin, dec!(0.075));
        assert_eq!(symbol.price_scale, 4);
        assert_eq!(symbol.quantity_scale, 2);
        assert_eq!(symbol.quote_asset, "USDT");
    }

    #[test]
    fn test_option_symbol_put_deserialization() {
        let json = r#"{
            "expiryDate": 1640995200000,
            "filters": [],
            "symbol": "BTC-231231-40000-P",
            "side": "PUT",
            "strikePrice": "40000.00",
            "underlying": "BTCUSDT",
            "unit": 1,
            "makerFeeRate": "0.0003",
            "takerFeeRate": "0.0003",
            "minQty": "0.01",
            "maxQty": "1000.00",
            "initialMargin": "0.15",
            "maintenanceMargin": "0.075",
            "minInitialMargin": "0.15",
            "minMaintenanceMargin": "0.075",
            "priceScale": 4,
            "quantityScale": 2,
            "quoteAsset": "USDT"
        }"#;

        let symbol: OptionSymbol = serde_json::from_str(json).unwrap();
        assert_eq!(symbol.side, OptionsContractType::Put);
        assert_eq!(symbol.strike_price, dec!(40000.00));
        assert_eq!(symbol.symbol, "BTC-231231-40000-P");
    }

    #[test]
    fn test_filter_price_filter_deserialization() {
        let json = r#"{
            "filterType": "PRICE_FILTER",
            "minPrice": "0.01",
            "maxPrice": "100000.00",
            "tickSize": "0.01"
        }"#;

        let filter: Filter = serde_json::from_str(json).unwrap();
        match filter {
            Filter::PriceFilter {
                min_price,
                max_price,
                tick_size,
            } => {
                assert_eq!(min_price, dec!(0.01));
                assert_eq!(max_price, dec!(100000.00));
                assert_eq!(tick_size, dec!(0.01));
            }
            _ => panic!("Expected PriceFilter"),
        }
    }

    #[test]
    fn test_filter_lot_size_deserialization() {
        let json = r#"{
            "filterType": "LOT_SIZE",
            "minQty": "0.01",
            "maxQty": "1000.00",
            "stepSize": "0.01"
        }"#;

        let filter: Filter = serde_json::from_str(json).unwrap();
        match filter {
            Filter::LotSize {
                min_qty,
                max_qty,
                step_size,
            } => {
                assert_eq!(min_qty, dec!(0.01));
                assert_eq!(max_qty, dec!(1000.00));
                assert_eq!(step_size, dec!(0.01));
            }
            _ => panic!("Expected LotSize"),
        }
    }

    #[test]
    fn test_rate_limit_deserialization() {
        let json = r#"{
            "rateLimitType": "REQUEST_WEIGHT",
            "interval": "MINUTE",
            "intervalNum": 1,
            "limit": 2400
        }"#;

        let rate_limit: RateLimit = serde_json::from_str(json).unwrap();
        assert_eq!(rate_limit.rate_limit_type, "REQUEST_WEIGHT");
        assert_eq!(rate_limit.interval, "MINUTE");
        assert_eq!(rate_limit.interval_num, 1);
        assert_eq!(rate_limit.limit, 2400);
    }

    #[test]
    fn test_exchange_info_empty_arrays() {
        let json = r#"{
            "timezone": "UTC",
            "serverTime": 1625184000000,
            "optionContracts": [],
            "optionAssets": [],
            "optionSymbols": [],
            "rateLimits": []
        }"#;

        let response: ExchangeInfoResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.timezone, "UTC");
        assert_eq!(response.server_time, 1625184000000);
        assert_eq!(response.option_contracts.len(), 0);
        assert_eq!(response.option_assets.len(), 0);
        assert_eq!(response.option_symbols.len(), 0);
        assert_eq!(response.rate_limits.len(), 0);
    }

    #[test]
    fn test_option_symbol_high_precision_values() {
        let json = r#"{
            "expiryDate": 1640995200000,
            "filters": [],
            "symbol": "BTC-231231-65000-C",
            "side": "CALL",
            "strikePrice": "65432.12345678",
            "underlying": "BTCUSDT",
            "unit": 1,
            "makerFeeRate": "0.00012345",
            "takerFeeRate": "0.00023456",
            "minQty": "0.00000001",
            "maxQty": "999999.99999999",
            "initialMargin": "0.12345678",
            "maintenanceMargin": "0.06789012",
            "minInitialMargin": "0.11111111",
            "minMaintenanceMargin": "0.05555555",
            "priceScale": 8,
            "quantityScale": 8,
            "quoteAsset": "USDT"
        }"#;

        let symbol: OptionSymbol = serde_json::from_str(json).unwrap();
        assert_eq!(symbol.strike_price.to_string(), "65432.12345678");
        assert_eq!(symbol.maker_fee_rate.to_string(), "0.00012345");
        assert_eq!(symbol.taker_fee_rate.to_string(), "0.00023456");
        assert_eq!(symbol.min_qty.to_string(), "0.00000001");
        assert_eq!(symbol.max_qty.to_string(), "999999.99999999");
        assert_eq!(symbol.initial_margin.to_string(), "0.12345678");
        assert_eq!(symbol.maintenance_margin.to_string(), "0.06789012");
        assert_eq!(symbol.min_initial_margin.to_string(), "0.11111111");
        assert_eq!(symbol.min_maintenance_margin.to_string(), "0.05555555");
        assert_eq!(symbol.price_scale, 8);
        assert_eq!(symbol.quantity_scale, 8);
    }

    #[test]
    fn test_option_symbol_with_multiple_filters() {
        let json = r#"{
            "expiryDate": 1640995200000,
            "filters": [
                {
                    "filterType": "PRICE_FILTER",
                    "minPrice": "0.01",
                    "maxPrice": "100000.00",
                    "tickSize": "0.01"
                },
                {
                    "filterType": "LOT_SIZE",
                    "minQty": "0.01",
                    "maxQty": "1000.00",
                    "stepSize": "0.01"
                }
            ],
            "symbol": "ETH-231231-3000-C",
            "side": "CALL",
            "strikePrice": "3000.00",
            "underlying": "ETHUSDT",
            "unit": 1,
            "makerFeeRate": "0.0003",
            "takerFeeRate": "0.0003",
            "minQty": "0.01",
            "maxQty": "1000.00",
            "initialMargin": "0.15",
            "maintenanceMargin": "0.075",
            "minInitialMargin": "0.15",
            "minMaintenanceMargin": "0.075",
            "priceScale": 4,
            "quantityScale": 2,
            "quoteAsset": "USDT"
        }"#;

        let symbol: OptionSymbol = serde_json::from_str(json).unwrap();
        assert_eq!(symbol.filters.len(), 2);
        assert_eq!(symbol.symbol, "ETH-231231-3000-C");
        assert_eq!(symbol.underlying, "ETHUSDT");
        assert_eq!(symbol.strike_price, dec!(3000.00));
    }
}
