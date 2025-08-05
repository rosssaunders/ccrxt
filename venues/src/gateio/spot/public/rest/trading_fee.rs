use serde::{Deserialize, Serialize};

use super::RestClient;

const TRADING_FEE_ENDPOINT: &str = "/spot/batch_fee";

/// Request parameters for trading fee inquiry
#[derive(Debug, Clone, Serialize, Default)]
pub struct TradingFeeRequest {
    /// Currency pair to query fee for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,
}

/// Trading fee information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingFee {
    /// Currency pair
    pub currency_pair: String,

    /// Maker fee rate
    pub maker_fee: String,

    /// Taker fee rate
    pub taker_fee: String,

    /// GT deduction enabled
    pub gt_deduction: bool,

    /// GT taker fee rate
    pub gt_taker_fee: String,

    /// GT maker fee rate
    pub gt_maker_fee: String,

    /// Loan fee rate
    pub loan_fee: String,

    /// Point type (0: GT, 1: Point card, 2: Disabled)
    pub point_type: i32,
}

/// Batch trading fee information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchTradingFee {
    /// Currency pair
    pub currency_pair: String,

    /// Maker fee rate
    pub maker_fee: String,

    /// Taker fee rate  
    pub taker_fee: String,
}

impl RestClient {
    /// Get trading fee information for a currency pair
    ///
    /// This endpoint returns the current trading fees for a specific currency pair
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#query-user-trading-fee-rates>
    /// or all pairs if no pair is specified.
    pub async fn get_trading_fee(
        &self,
        params: TradingFeeRequest,
    ) -> crate::gateio::spot::RestResult<TradingFee> {
        self.get_with_query("/spot/fee", Some(&params)).await
    }

    /// Get batch trading fee information
    ///
    /// This endpoint returns trading fees for multiple currency pairs at once.
    pub async fn get_batch_trading_fee(&self) -> crate::gateio::spot::RestResult<Vec<BatchTradingFee>> {
        self.get(TRADING_FEE_ENDPOINT).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trading_fee_request_minimal_serialization() {
        let request = TradingFeeRequest {
            currency_pair: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_trading_fee_request_with_pair() {
        let request = TradingFeeRequest {
            currency_pair: Some("BTC_USDT".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "currency_pair=BTC_USDT");
    }

    #[test]
    fn test_trading_fee_request_different_pairs() {
        let pairs = vec![
            "BTC_USDT",
            "ETH_USDT",
            "BNB_USDT",
            "SOL_USDT",
            "ADA_USDT",
            "DOT_USDT",
            "MATIC_USDT",
        ];

        for pair in pairs {
            let request = TradingFeeRequest {
                currency_pair: Some(pair.to_string()),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("currency_pair={}", pair));
        }
    }

    #[test]
    fn test_trading_fee_request_default() {
        let request = TradingFeeRequest::default();
        assert_eq!(request.currency_pair, None);
    }

    #[test]
    fn test_trading_fee_deserialization() {
        let json = r#"{
            "currency_pair": "BTC_USDT",
            "maker_fee": "0.002",
            "taker_fee": "0.002",
            "gt_deduction": true,
            "gt_taker_fee": "0.0015",
            "gt_maker_fee": "0.0015",
            "loan_fee": "0.01",
            "point_type": 0
        }"#;

        let fee: TradingFee = serde_json::from_str(json).unwrap();
        assert_eq!(fee.currency_pair, "BTC_USDT");
        assert_eq!(fee.maker_fee, "0.002");
        assert_eq!(fee.taker_fee, "0.002");
        assert_eq!(fee.gt_deduction, true);
        assert_eq!(fee.gt_taker_fee, "0.0015");
        assert_eq!(fee.gt_maker_fee, "0.0015");
        assert_eq!(fee.loan_fee, "0.01");
        assert_eq!(fee.point_type, 0);
    }

    #[test]
    fn test_trading_fee_without_gt_deduction() {
        let json = r#"{
            "currency_pair": "ETH_USDT",
            "maker_fee": "0.001",
            "taker_fee": "0.002",
            "gt_deduction": false,
            "gt_taker_fee": "0",
            "gt_maker_fee": "0",
            "loan_fee": "0.02",
            "point_type": 2
        }"#;

        let fee: TradingFee = serde_json::from_str(json).unwrap();
        assert_eq!(fee.currency_pair, "ETH_USDT");
        assert_eq!(fee.maker_fee, "0.001");
        assert_eq!(fee.taker_fee, "0.002");
        assert_eq!(fee.gt_deduction, false);
        assert_eq!(fee.gt_taker_fee, "0");
        assert_eq!(fee.gt_maker_fee, "0");
        assert_eq!(fee.loan_fee, "0.02");
        assert_eq!(fee.point_type, 2); // Disabled
    }

    #[test]
    fn test_trading_fee_with_point_card() {
        let json = r#"{
            "currency_pair": "BNB_USDT",
            "maker_fee": "0.0008",
            "taker_fee": "0.001",
            "gt_deduction": false,
            "gt_taker_fee": "0",
            "gt_maker_fee": "0",
            "loan_fee": "0.015",
            "point_type": 1
        }"#;

        let fee: TradingFee = serde_json::from_str(json).unwrap();
        assert_eq!(fee.point_type, 1); // Point card
    }

    #[test]
    fn test_trading_fee_zero_fees() {
        let json = r#"{
            "currency_pair": "PROMO_USDT",
            "maker_fee": "0",
            "taker_fee": "0",
            "gt_deduction": true,
            "gt_taker_fee": "0",
            "gt_maker_fee": "0",
            "loan_fee": "0",
            "point_type": 0
        }"#;

        let fee: TradingFee = serde_json::from_str(json).unwrap();
        assert_eq!(fee.maker_fee, "0");
        assert_eq!(fee.taker_fee, "0");
        assert_eq!(fee.loan_fee, "0");
    }

    #[test]
    fn test_trading_fee_high_fees() {
        let json = r#"{
            "currency_pair": "NEWCOIN_USDT",
            "maker_fee": "0.005",
            "taker_fee": "0.01",
            "gt_deduction": true,
            "gt_taker_fee": "0.0075",
            "gt_maker_fee": "0.00375",
            "loan_fee": "0.05",
            "point_type": 0
        }"#;

        let fee: TradingFee = serde_json::from_str(json).unwrap();
        assert_eq!(fee.maker_fee, "0.005");
        assert_eq!(fee.taker_fee, "0.01");
        assert_eq!(fee.loan_fee, "0.05");
    }

    #[test]
    fn test_trading_fee_stablecoin_pair() {
        let json = r#"{
            "currency_pair": "USDC_USDT",
            "maker_fee": "0.0002",
            "taker_fee": "0.0005",
            "gt_deduction": true,
            "gt_taker_fee": "0.000375",
            "gt_maker_fee": "0.00015",
            "loan_fee": "0.005",
            "point_type": 0
        }"#;

        let fee: TradingFee = serde_json::from_str(json).unwrap();
        assert_eq!(fee.currency_pair, "USDC_USDT");
        assert_eq!(fee.maker_fee, "0.0002"); // Lower fees for stablecoin pairs
        assert_eq!(fee.taker_fee, "0.0005");
    }

    #[test]
    fn test_trading_fee_extreme_precision() {
        let json = r#"{
            "currency_pair": "TEST_USDT",
            "maker_fee": "0.00123456789",
            "taker_fee": "0.00234567890",
            "gt_deduction": true,
            "gt_taker_fee": "0.00175925925",
            "gt_maker_fee": "0.00092592593",
            "loan_fee": "0.01234567890",
            "point_type": 0
        }"#;

        let fee: TradingFee = serde_json::from_str(json).unwrap();
        assert_eq!(fee.maker_fee, "0.00123456789");
        assert_eq!(fee.taker_fee, "0.00234567890");
        assert_eq!(fee.gt_taker_fee, "0.00175925925");
        assert_eq!(fee.gt_maker_fee, "0.00092592593");
        assert_eq!(fee.loan_fee, "0.01234567890");
    }

    #[test]
    fn test_trading_fee_point_type_values() {
        let point_types = vec![(0, "GT"), (1, "Point card"), (2, "Disabled")];

        for (point_type, _description) in point_types {
            let json = format!(
                r#"{{
                "currency_pair": "BTC_USDT",
                "maker_fee": "0.002",
                "taker_fee": "0.002",
                "gt_deduction": true,
                "gt_taker_fee": "0.0015",
                "gt_maker_fee": "0.0015",
                "loan_fee": "0.01",
                "point_type": {}
            }}"#,
                point_type
            );

            let fee: TradingFee = serde_json::from_str(&json).unwrap();
            assert_eq!(fee.point_type, point_type);
        }
    }

    #[test]
    fn test_trading_fee_serialization() {
        let fee = TradingFee {
            currency_pair: "BTC_USDT".to_string(),
            maker_fee: "0.002".to_string(),
            taker_fee: "0.002".to_string(),
            gt_deduction: true,
            gt_taker_fee: "0.0015".to_string(),
            gt_maker_fee: "0.0015".to_string(),
            loan_fee: "0.01".to_string(),
            point_type: 0,
        };

        let json = serde_json::to_value(&fee).unwrap();
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["maker_fee"], "0.002");
        assert_eq!(json["taker_fee"], "0.002");
        assert_eq!(json["gt_deduction"], true);
        assert_eq!(json["gt_taker_fee"], "0.0015");
        assert_eq!(json["gt_maker_fee"], "0.0015");
        assert_eq!(json["loan_fee"], "0.01");
        assert_eq!(json["point_type"], 0);
    }

    #[test]
    fn test_trading_fee_round_trip() {
        let original = TradingFee {
            currency_pair: "ETH_BTC".to_string(),
            maker_fee: "0.001".to_string(),
            taker_fee: "0.002".to_string(),
            gt_deduction: false,
            gt_taker_fee: "0".to_string(),
            gt_maker_fee: "0".to_string(),
            loan_fee: "0.02".to_string(),
            point_type: 2,
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: TradingFee = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.currency_pair, original.currency_pair);
        assert_eq!(deserialized.maker_fee, original.maker_fee);
        assert_eq!(deserialized.taker_fee, original.taker_fee);
        assert_eq!(deserialized.gt_deduction, original.gt_deduction);
        assert_eq!(deserialized.gt_taker_fee, original.gt_taker_fee);
        assert_eq!(deserialized.gt_maker_fee, original.gt_maker_fee);
        assert_eq!(deserialized.loan_fee, original.loan_fee);
        assert_eq!(deserialized.point_type, original.point_type);
    }

    #[test]
    fn test_batch_trading_fee_deserialization() {
        let json = r#"{
            "currency_pair": "BTC_USDT",
            "maker_fee": "0.002",
            "taker_fee": "0.002"
        }"#;

        let fee: BatchTradingFee = serde_json::from_str(json).unwrap();
        assert_eq!(fee.currency_pair, "BTC_USDT");
        assert_eq!(fee.maker_fee, "0.002");
        assert_eq!(fee.taker_fee, "0.002");
    }

    #[test]
    fn test_batch_trading_fee_array_deserialization() {
        let json = r#"[
            {
                "currency_pair": "BTC_USDT",
                "maker_fee": "0.002",
                "taker_fee": "0.002"
            },
            {
                "currency_pair": "ETH_USDT",
                "maker_fee": "0.001",
                "taker_fee": "0.002"
            },
            {
                "currency_pair": "USDC_USDT",
                "maker_fee": "0.0002",
                "taker_fee": "0.0005"
            }
        ]"#;

        let fees: Vec<BatchTradingFee> = serde_json::from_str(json).unwrap();
        assert_eq!(fees.len(), 3);

        assert_eq!(fees[0].currency_pair, "BTC_USDT");
        assert_eq!(fees[0].maker_fee, "0.002");
        assert_eq!(fees[0].taker_fee, "0.002");

        assert_eq!(fees[1].currency_pair, "ETH_USDT");
        assert_eq!(fees[1].maker_fee, "0.001");
        assert_eq!(fees[1].taker_fee, "0.002");

        assert_eq!(fees[2].currency_pair, "USDC_USDT");
        assert_eq!(fees[2].maker_fee, "0.0002");
        assert_eq!(fees[2].taker_fee, "0.0005");
    }

    #[test]
    fn test_batch_trading_fee_empty_array() {
        let json = r#"[]"#;
        let fees: Vec<BatchTradingFee> = serde_json::from_str(json).unwrap();
        assert_eq!(fees.len(), 0);
    }

    #[test]
    fn test_batch_trading_fee_serialization() {
        let fee = BatchTradingFee {
            currency_pair: "BTC_USDT".to_string(),
            maker_fee: "0.002".to_string(),
            taker_fee: "0.002".to_string(),
        };

        let json = serde_json::to_value(&fee).unwrap();
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["maker_fee"], "0.002");
        assert_eq!(json["taker_fee"], "0.002");
    }

    #[test]
    fn test_batch_trading_fee_round_trip() {
        let original = BatchTradingFee {
            currency_pair: "ETH_BTC".to_string(),
            maker_fee: "0.001".to_string(),
            taker_fee: "0.002".to_string(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: BatchTradingFee = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.currency_pair, original.currency_pair);
        assert_eq!(deserialized.maker_fee, original.maker_fee);
        assert_eq!(deserialized.taker_fee, original.taker_fee);
    }

    #[test]
    fn test_trading_fee_realistic_scenarios() {
        // VIP tier with GT discount
        let vip_json = r#"{
            "currency_pair": "BTC_USDT",
            "maker_fee": "0.0008",
            "taker_fee": "0.001",
            "gt_deduction": true,
            "gt_taker_fee": "0.00075",
            "gt_maker_fee": "0.0006",
            "loan_fee": "0.01",
            "point_type": 0
        }"#;

        let vip_fee: TradingFee = serde_json::from_str(vip_json).unwrap();
        assert_eq!(vip_fee.maker_fee, "0.0008"); // VIP maker fee
        assert_eq!(vip_fee.taker_fee, "0.001"); // VIP taker fee

        // Calculate discount percentage
        let maker_discount: f64 = 1.0 - (0.0006 / 0.0008);
        let taker_discount: f64 = 1.0 - (0.00075 / 0.001);
        let epsilon = 1e-10;
        assert!((maker_discount - 0.25).abs() < epsilon); // 25% discount with GT
        assert!((taker_discount - 0.25).abs() < epsilon); // 25% discount with GT
    }

    #[test]
    fn test_batch_trading_fee_realistic_portfolio() {
        let json = r#"[
            {
                "currency_pair": "BTC_USDT",
                "maker_fee": "0.002",
                "taker_fee": "0.002"
            },
            {
                "currency_pair": "ETH_BTC",
                "maker_fee": "0.001",
                "taker_fee": "0.002"
            },
            {
                "currency_pair": "USDC_USDT",
                "maker_fee": "0.0002",
                "taker_fee": "0.0005"
            },
            {
                "currency_pair": "BNB_USDT",
                "maker_fee": "0.001",
                "taker_fee": "0.001"
            }
        ]"#;

        let fees: Vec<BatchTradingFee> = serde_json::from_str(json).unwrap();

        // Verify stablecoin pairs have lower fees
        let stablecoin_fee = fees
            .iter()
            .find(|f| f.currency_pair == "USDC_USDT")
            .unwrap();
        let btc_fee = fees.iter().find(|f| f.currency_pair == "BTC_USDT").unwrap();

        let stablecoin_maker: f64 = stablecoin_fee.maker_fee.parse().unwrap();
        let btc_maker: f64 = btc_fee.maker_fee.parse().unwrap();

        assert!(stablecoin_maker < btc_maker); // Stablecoin pairs typically have lower fees
    }

    #[test]
    fn test_trading_fee_clone() {
        let original = TradingFee {
            currency_pair: "BTC_USDT".to_string(),
            maker_fee: "0.002".to_string(),
            taker_fee: "0.002".to_string(),
            gt_deduction: true,
            gt_taker_fee: "0.0015".to_string(),
            gt_maker_fee: "0.0015".to_string(),
            loan_fee: "0.01".to_string(),
            point_type: 0,
        };

        let cloned = original.clone();
        assert_eq!(cloned.currency_pair, original.currency_pair);
        assert_eq!(cloned.maker_fee, original.maker_fee);
        assert_eq!(cloned.taker_fee, original.taker_fee);
        assert_eq!(cloned.gt_deduction, original.gt_deduction);
        assert_eq!(cloned.gt_taker_fee, original.gt_taker_fee);
        assert_eq!(cloned.gt_maker_fee, original.gt_maker_fee);
        assert_eq!(cloned.loan_fee, original.loan_fee);
        assert_eq!(cloned.point_type, original.point_type);
    }

    #[test]
    fn test_trading_fee_debug() {
        let fee = TradingFee {
            currency_pair: "BTC_USDT".to_string(),
            maker_fee: "0.002".to_string(),
            taker_fee: "0.002".to_string(),
            gt_deduction: true,
            gt_taker_fee: "0.0015".to_string(),
            gt_maker_fee: "0.0015".to_string(),
            loan_fee: "0.01".to_string(),
            point_type: 0,
        };

        let debug_str = format!("{:?}", fee);
        assert!(debug_str.contains("TradingFee"));
        assert!(debug_str.contains("BTC_USDT"));
        assert!(debug_str.contains("0.002"));
    }
}
