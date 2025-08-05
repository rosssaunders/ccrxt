use serde::{Deserialize, Serialize};

use super::RestClient;

const FUTURES_ACCOUNTS_ENDPOINT: &str = "/futures/{}/accounts";

/// Request parameters for futures accounts
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FuturesAccountsRequest {
    /// Settlement currency (BTC, USDT, etc.)
    pub settle: String,
}

/// Futures account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesAccount {
    /// Total balance
    pub total: String,

    /// Unrealized PnL
    pub unrealised_pnl: String,

    /// Position margin
    pub position_margin: String,

    /// Order margin
    pub order_margin: String,

    /// Available balance
    pub available: String,

    /// Point balance
    pub point: String,

    /// Currency
    pub currency: String,

    /// Balance in settlement currency
    pub in_dual_mode: bool,

    /// Enable credit
    pub enable_credit: bool,

    /// Position cross margin
    pub position_cross_margin: String,

    /// Order cross margin
    pub order_cross_margin: String,

    /// Available cross margin
    pub available_cross_margin: String,

    /// Total cross margin
    pub total_cross_margin: String,
}

impl RestClient {
    /// Get futures accounts
    ///
    /// This endpoint returns futures account information for the authenticated user.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.com/docs/developers/apiv4/#query-futures-account>
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `params` - The futures account request parameters
    ///
    /// # Returns
    /// Account information
    pub async fn get_futures_accounts(
        &self,
        params: FuturesAccountsRequest,
    ) -> crate::gateio::perpetual::Result<FuturesAccount> {
        let endpoint = FUTURES_ACCOUNTS_ENDPOINT.replace("{}", &params.settle);
        self.get(&endpoint).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_futures_accounts_request() {
        let request = FuturesAccountsRequest {
            settle: "USDT".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 1);
    }

    #[test]
    fn test_different_settlement_currencies() {
        let currencies = vec!["USDT", "BTC", "USD", "ETH"];

        for settle in currencies {
            let request = FuturesAccountsRequest {
                settle: settle.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["settle"], settle);
        }
    }

    #[test]
    fn test_futures_account_deserialization() {
        let json = r#"{
            "total": "12500.5",
            "unrealised_pnl": "250.25",
            "position_margin": "2000.0",
            "order_margin": "500.0",
            "available": "9750.25",
            "point": "100.0",
            "currency": "USDT",
            "in_dual_mode": false,
            "enable_credit": true,
            "position_cross_margin": "1500.0",
            "order_cross_margin": "300.0",
            "available_cross_margin": "8450.25",
            "total_cross_margin": "10250.25"
        }"#;

        let account: FuturesAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.total, "12500.5");
        assert_eq!(account.unrealised_pnl, "250.25");
        assert_eq!(account.position_margin, "2000.0");
        assert_eq!(account.order_margin, "500.0");
        assert_eq!(account.available, "9750.25");
        assert_eq!(account.point, "100.0");
        assert_eq!(account.currency, "USDT");
        assert_eq!(account.in_dual_mode, false);
        assert_eq!(account.enable_credit, true);
        assert_eq!(account.position_cross_margin, "1500.0");
        assert_eq!(account.order_cross_margin, "300.0");
        assert_eq!(account.available_cross_margin, "8450.25");
        assert_eq!(account.total_cross_margin, "10250.25");
    }

    #[test]
    fn test_usdt_account_scenario() {
        let json = r#"{
            "total": "50000.0",
            "unrealised_pnl": "1250.5",
            "position_margin": "8000.0",
            "order_margin": "2000.0",
            "available": "40000.0",
            "point": "500.0",
            "currency": "USDT",
            "in_dual_mode": true,
            "enable_credit": true,
            "position_cross_margin": "6000.0",
            "order_cross_margin": "1500.0",
            "available_cross_margin": "41750.5",
            "total_cross_margin": "49250.5"
        }"#;

        let account: FuturesAccount = serde_json::from_str(json).unwrap();

        // Verify balances add up correctly
        let total: f64 = account.total.parse().unwrap();
        let position_margin: f64 = account.position_margin.parse().unwrap();
        let order_margin: f64 = account.order_margin.parse().unwrap();
        let available: f64 = account.available.parse().unwrap();

        // Total should be close to position_margin + order_margin + available
        let calculated_total = position_margin + order_margin + available;
        assert!((total - calculated_total).abs() < 1000.0); // Allow for PnL differences

        assert_eq!(account.currency, "USDT");
        assert!(account.in_dual_mode);
        assert!(account.enable_credit);
    }

    #[test]
    fn test_btc_account_scenario() {
        let json = r#"{
            "total": "1.25",
            "unrealised_pnl": "0.025",
            "position_margin": "0.2",
            "order_margin": "0.05",
            "available": "0.975",
            "point": "0.01",
            "currency": "BTC",
            "in_dual_mode": false,
            "enable_credit": false,
            "position_cross_margin": "0.15",
            "order_cross_margin": "0.03",
            "available_cross_margin": "1.045",
            "total_cross_margin": "1.225"
        }"#;

        let account: FuturesAccount = serde_json::from_str(json).unwrap();

        // Verify BTC amounts are reasonable
        let total: f64 = account.total.parse().unwrap();
        let available: f64 = account.available.parse().unwrap();

        assert!(total > 0.0 && total < 100.0); // Reasonable BTC amount
        assert!(available > 0.0 && available < total);
        assert_eq!(account.currency, "BTC");
        assert!(!account.in_dual_mode);
        assert!(!account.enable_credit);
    }

    #[test]
    fn test_positive_pnl_scenario() {
        let json = r#"{
            "total": "25000.0",
            "unrealised_pnl": "2500.0",
            "position_margin": "5000.0",
            "order_margin": "1000.0",
            "available": "16500.0",
            "point": "200.0",
            "currency": "USDT",
            "in_dual_mode": true,
            "enable_credit": true,
            "position_cross_margin": "4000.0",
            "order_cross_margin": "800.0",
            "available_cross_margin": "17700.0",
            "total_cross_margin": "22500.0"
        }"#;

        let account: FuturesAccount = serde_json::from_str(json).unwrap();

        // Positive PnL scenario
        let unrealised_pnl: f64 = account.unrealised_pnl.parse().unwrap();
        assert!(unrealised_pnl > 0.0);

        // Available should be healthy with positive PnL
        let available: f64 = account.available.parse().unwrap();
        let total: f64 = account.total.parse().unwrap();
        assert!(available / total > 0.5); // More than 50% available
    }

    #[test]
    fn test_negative_pnl_scenario() {
        let json = r#"{
            "total": "20000.0",
            "unrealised_pnl": "-1500.0",
            "position_margin": "12000.0",
            "order_margin": "3000.0",
            "available": "3500.0",
            "point": "50.0",
            "currency": "USDT",
            "in_dual_mode": false,
            "enable_credit": true,
            "position_cross_margin": "10000.0",
            "order_cross_margin": "2500.0",
            "available_cross_margin": "6000.0",
            "total_cross_margin": "18500.0"
        }"#;

        let account: FuturesAccount = serde_json::from_str(json).unwrap();

        // Negative PnL scenario
        let unrealised_pnl: f64 = account.unrealised_pnl.parse().unwrap();
        assert!(unrealised_pnl < 0.0);

        // Available should be lower with negative PnL
        let available: f64 = account.available.parse().unwrap();
        let total: f64 = account.total.parse().unwrap();
        assert!(available / total < 0.2); // Less than 20% available
    }

    #[test]
    fn test_high_margin_usage_scenario() {
        let json = r#"{
            "total": "10000.0",
            "unrealised_pnl": "-500.0",
            "position_margin": "7600.0",
            "order_margin": "1500.0",
            "available": "400.0",
            "point": "25.0",
            "currency": "USDT",
            "in_dual_mode": true,
            "enable_credit": true,
            "position_cross_margin": "7000.0",
            "order_cross_margin": "1200.0",
            "available_cross_margin": "1300.0",
            "total_cross_margin": "9500.0"
        }"#;

        let account: FuturesAccount = serde_json::from_str(json).unwrap();

        // High margin usage (risky position)
        let position_margin: f64 = account.position_margin.parse().unwrap();
        let order_margin: f64 = account.order_margin.parse().unwrap();
        let total: f64 = account.total.parse().unwrap();
        let available: f64 = account.available.parse().unwrap();

        let margin_usage = (position_margin + order_margin) / total;
        assert!(margin_usage > 0.9); // Over 90% margin usage
        assert!(available < 1000.0); // Low available balance
    }

    #[test]
    fn test_low_margin_usage_scenario() {
        let json = r#"{
            "total": "100000.0",
            "unrealised_pnl": "5000.0",
            "position_margin": "10000.0",
            "order_margin": "5000.0",
            "available": "80000.0",
            "point": "1000.0",
            "currency": "USDT",
            "in_dual_mode": false,
            "enable_credit": false,
            "position_cross_margin": "8000.0",
            "order_cross_margin": "4000.0",
            "available_cross_margin": "88000.0",
            "total_cross_margin": "100000.0"
        }"#;

        let account: FuturesAccount = serde_json::from_str(json).unwrap();

        // Low margin usage (conservative position)
        let position_margin: f64 = account.position_margin.parse().unwrap();
        let order_margin: f64 = account.order_margin.parse().unwrap();
        let total: f64 = account.total.parse().unwrap();
        let available: f64 = account.available.parse().unwrap();

        let margin_usage = (position_margin + order_margin) / total;
        assert!(margin_usage < 0.2); // Under 20% margin usage
        assert!(available > 70000.0); // High available balance
    }

    #[test]
    fn test_dual_mode_scenarios() {
        let dual_mode_scenarios = vec![
            (true, "Dual mode enabled", "50000.0", "45000.0"),
            (false, "Single mode", "50000.0", "50000.0"),
        ];

        for (in_dual_mode, _description, total, total_cross) in dual_mode_scenarios {
            let json = format!(
                r#"{{
                "total": "{}",
                "unrealised_pnl": "0.0",
                "position_margin": "10000.0",
                "order_margin": "5000.0",
                "available": "35000.0",
                "point": "100.0",
                "currency": "USDT",
                "in_dual_mode": {},
                "enable_credit": true,
                "position_cross_margin": "8000.0",
                "order_cross_margin": "4000.0",
                "available_cross_margin": "33000.0",
                "total_cross_margin": "{}"
            }}"#,
                total, in_dual_mode, total_cross
            );

            let account: FuturesAccount = serde_json::from_str(&json).unwrap();
            assert_eq!(account.in_dual_mode, in_dual_mode);

            if in_dual_mode {
                // In dual mode, cross margin total may differ
                let total_val: f64 = account.total.parse().unwrap();
                let total_cross_val: f64 = account.total_cross_margin.parse().unwrap();
                assert!(total_cross_val <= total_val);
            }
        }
    }

    #[test]
    fn test_credit_scenarios() {
        let credit_scenarios = vec![
            (true, "Credit enabled", "500.0"),
            (false, "Credit disabled", "0.0"),
        ];

        for (enable_credit, _, point) in credit_scenarios {
            let json = format!(
                r#"{{
                "total": "10000.0",
                "unrealised_pnl": "0.0",
                "position_margin": "2000.0",
                "order_margin": "1000.0",
                "available": "7000.0",
                "point": "{}",
                "currency": "USDT",
                "in_dual_mode": false,
                "enable_credit": {},
                "position_cross_margin": "1800.0",
                "order_cross_margin": "900.0",
                "available_cross_margin": "7300.0",
                "total_cross_margin": "10000.0"
            }}"#,
                point, enable_credit
            );

            let account: FuturesAccount = serde_json::from_str(&json).unwrap();
            assert_eq!(account.enable_credit, enable_credit);

            let point_val: f64 = account.point.parse().unwrap();
            if enable_credit {
                assert!(point_val >= 0.0); // Can have points
            }
        }
    }

    #[test]
    fn test_cross_margin_vs_isolated_margin() {
        let json = r#"{
            "total": "30000.0",
            "unrealised_pnl": "0.0",
            "position_margin": "6000.0",
            "order_margin": "2000.0",
            "available": "22000.0",
            "point": "150.0",
            "currency": "USDT",
            "in_dual_mode": true,
            "enable_credit": true,
            "position_cross_margin": "4500.0",
            "order_cross_margin": "1500.0",
            "available_cross_margin": "24000.0",
            "total_cross_margin": "30000.0"
        }"#;

        let account: FuturesAccount = serde_json::from_str(json).unwrap();

        // Compare isolated vs cross margin values
        let position_margin: f64 = account.position_margin.parse().unwrap();
        let position_cross_margin: f64 = account.position_cross_margin.parse().unwrap();

        // Cross margin is typically lower than isolated
        assert!(position_cross_margin <= position_margin);

        let order_margin: f64 = account.order_margin.parse().unwrap();
        let order_cross_margin: f64 = account.order_cross_margin.parse().unwrap();
        assert!(order_cross_margin <= order_margin);
    }

    #[test]
    fn test_zero_balance_scenario() {
        let json = r#"{
            "total": "0.0",
            "unrealised_pnl": "0.0",
            "position_margin": "0.0",
            "order_margin": "0.0",
            "available": "0.0",
            "point": "0.0",
            "currency": "USDT",
            "in_dual_mode": false,
            "enable_credit": false,
            "position_cross_margin": "0.0",
            "order_cross_margin": "0.0",
            "available_cross_margin": "0.0",
            "total_cross_margin": "0.0"
        }"#;

        let account: FuturesAccount = serde_json::from_str(json).unwrap();

        // All balances should be zero
        assert_eq!(account.total, "0.0");
        assert_eq!(account.available, "0.0");
        assert_eq!(account.position_margin, "0.0");
        assert_eq!(account.order_margin, "0.0");
    }

    #[test]
    fn test_liquidation_risk_scenario() {
        let json = r#"{
            "total": "5000.0",
            "unrealised_pnl": "-2000.0",
            "position_margin": "4500.0",
            "order_margin": "300.0",
            "available": "-1800.0",
            "point": "10.0",
            "currency": "USDT",
            "in_dual_mode": false,
            "enable_credit": true,
            "position_cross_margin": "4200.0",
            "order_cross_margin": "250.0",
            "available_cross_margin": "-1450.0",
            "total_cross_margin": "3000.0"
        }"#;

        let account: FuturesAccount = serde_json::from_str(json).unwrap();

        // Near liquidation scenario
        let available: f64 = account.available.parse().unwrap();
        let unrealised_pnl: f64 = account.unrealised_pnl.parse().unwrap();

        assert!(available < 0.0); // Negative available balance
        assert!(unrealised_pnl < 0.0); // Losing position

        // Account is at high risk of liquidation
        let total: f64 = account.total.parse().unwrap();
        let position_margin: f64 = account.position_margin.parse().unwrap();
        assert!(position_margin / total > 0.8); // Over 80% in positions
    }

    #[test]
    fn test_large_account_scenario() {
        let json = r#"{
            "total": "1000000.0",
            "unrealised_pnl": "50000.0",
            "position_margin": "200000.0",
            "order_margin": "100000.0",
            "available": "650000.0",
            "point": "5000.0",
            "currency": "USDT",
            "in_dual_mode": true,
            "enable_credit": true,
            "position_cross_margin": "180000.0",
            "order_cross_margin": "90000.0",
            "available_cross_margin": "680000.0",
            "total_cross_margin": "950000.0"
        }"#;

        let account: FuturesAccount = serde_json::from_str(json).unwrap();

        // Large institutional account
        let total: f64 = account.total.parse().unwrap();
        assert!(total >= 1000000.0); // Million+ account

        // Should have conservative margin usage
        let position_margin: f64 = account.position_margin.parse().unwrap();
        let margin_ratio = position_margin / total;
        assert!(margin_ratio < 0.3); // Under 30% margin usage
    }

    #[test]
    fn test_small_account_scenario() {
        let json = r#"{
            "total": "100.0",
            "unrealised_pnl": "5.0",
            "position_margin": "20.0",
            "order_margin": "10.0",
            "available": "65.0",
            "point": "1.0",
            "currency": "USDT",
            "in_dual_mode": false,
            "enable_credit": false,
            "position_cross_margin": "18.0",
            "order_cross_margin": "9.0",
            "available_cross_margin": "68.0",
            "total_cross_margin": "95.0"
        }"#;

        let account: FuturesAccount = serde_json::from_str(json).unwrap();

        // Small retail account
        let total: f64 = account.total.parse().unwrap();
        assert!(total <= 100.0); // Small account

        // Might have higher margin usage
        let position_margin: f64 = account.position_margin.parse().unwrap();
        let margin_ratio = position_margin / total;
        assert!(margin_ratio >= 0.2); // 20%+ margin usage
    }

    #[test]
    fn test_high_precision_values() {
        let json = r#"{
            "total": "12345.678901234",
            "unrealised_pnl": "123.456789012",
            "position_margin": "2345.678901234",
            "order_margin": "567.890123456",
            "available": "9308.453765322",
            "point": "12.345678901",
            "currency": "USDT",
            "in_dual_mode": true,
            "enable_credit": true,
            "position_cross_margin": "2100.123456789",
            "order_cross_margin": "500.987654321",
            "available_cross_margin": "9622.444789790",
            "total_cross_margin": "12223.556789900"
        }"#;

        let account: FuturesAccount = serde_json::from_str(json).unwrap();

        // Verify high precision is maintained
        assert_eq!(account.total, "12345.678901234");
        assert_eq!(account.unrealised_pnl, "123.456789012");
        assert_eq!(account.position_margin, "2345.678901234");
        assert_eq!(account.available, "9308.453765322");
    }

    #[test]
    fn test_different_currency_scenarios() {
        let currency_scenarios = vec![
            ("USDT", "50000.0", "Standard USDT account"),
            ("BTC", "1.5", "BTC-margined account"),
            ("ETH", "20.0", "ETH-margined account"),
            ("USD", "50000.0", "USD account"),
        ];

        for (currency, total, _description) in currency_scenarios {
            let json = format!(
                r#"{{
                "total": "{}",
                "unrealised_pnl": "0.0",
                "position_margin": "0.0",
                "order_margin": "0.0",
                "available": "{}",
                "point": "0.0",
                "currency": "{}",
                "in_dual_mode": false,
                "enable_credit": false,
                "position_cross_margin": "0.0",
                "order_cross_margin": "0.0",
                "available_cross_margin": "{}",
                "total_cross_margin": "{}"
            }}"#,
                total, total, currency, total, total
            );

            let account: FuturesAccount = serde_json::from_str(&json).unwrap();
            assert_eq!(account.currency, currency);

            let total_val: f64 = account.total.parse().unwrap();
            if currency == "BTC" || currency == "ETH" {
                assert!(total_val < 100.0); // Crypto amounts should be smaller
            } else {
                assert!(total_val >= 100.0); // Fiat amounts larger
            }
        }
    }

    #[test]
    fn test_margin_calculation_consistency() {
        let json = r#"{
            "total": "25000.0",
            "unrealised_pnl": "500.0",
            "position_margin": "5000.0",
            "order_margin": "2500.0",
            "available": "18000.0",
            "point": "200.0",
            "currency": "USDT",
            "in_dual_mode": true,
            "enable_credit": true,
            "position_cross_margin": "4500.0",
            "order_cross_margin": "2250.0",
            "available_cross_margin": "17750.0",
            "total_cross_margin": "24500.0"
        }"#;

        let account: FuturesAccount = serde_json::from_str(json).unwrap();

        // Verify margin calculations
        let total: f64 = account.total.parse().unwrap();
        let position_margin: f64 = account.position_margin.parse().unwrap();
        let order_margin: f64 = account.order_margin.parse().unwrap();
        let available: f64 = account.available.parse().unwrap();
        let unrealised_pnl: f64 = account.unrealised_pnl.parse().unwrap();

        // Basic equation: available ≈ total - position_margin - order_margin + unrealised_pnl
        let calculated_available = total - position_margin - order_margin + unrealised_pnl;
        let diff = (available - calculated_available).abs();

        // Should be close (within rounding errors or point balance)
        assert!(diff < 1000.0);
    }

    #[test]
    fn test_clone_behavior() {
        let request = FuturesAccountsRequest {
            settle: "USDT".to_string(),
        };

        let cloned = request.clone();
        assert_eq!(cloned.settle, request.settle);
    }

    #[test]
    fn test_debug_output() {
        let account = FuturesAccount {
            total: "10000.0".to_string(),
            unrealised_pnl: "100.0".to_string(),
            position_margin: "2000.0".to_string(),
            order_margin: "1000.0".to_string(),
            available: "7100.0".to_string(),
            point: "50.0".to_string(),
            currency: "USDT".to_string(),
            in_dual_mode: false,
            enable_credit: true,
            position_cross_margin: "1800.0".to_string(),
            order_cross_margin: "900.0".to_string(),
            available_cross_margin: "7400.0".to_string(),
            total_cross_margin: "10100.0".to_string(),
        };

        let debug_str = format!("{:?}", account);
        assert!(debug_str.contains("FuturesAccount"));
        assert!(debug_str.contains("10000.0"));
        assert!(debug_str.contains("USDT"));
    }

    #[test]
    fn test_serialization_round_trip() {
        let account = FuturesAccount {
            total: "50000.0".to_string(),
            unrealised_pnl: "1000.0".to_string(),
            position_margin: "10000.0".to_string(),
            order_margin: "5000.0".to_string(),
            available: "34000.0".to_string(),
            point: "250.0".to_string(),
            currency: "USDT".to_string(),
            in_dual_mode: true,
            enable_credit: true,
            position_cross_margin: "9000.0".to_string(),
            order_cross_margin: "4500.0".to_string(),
            available_cross_margin: "35500.0".to_string(),
            total_cross_margin: "49000.0".to_string(),
        };

        let json = serde_json::to_string(&account).unwrap();
        let deserialized: FuturesAccount = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.total, account.total);
        assert_eq!(deserialized.unrealised_pnl, account.unrealised_pnl);
        assert_eq!(deserialized.position_margin, account.position_margin);
        assert_eq!(deserialized.order_margin, account.order_margin);
        assert_eq!(deserialized.available, account.available);
        assert_eq!(deserialized.currency, account.currency);
        assert_eq!(deserialized.in_dual_mode, account.in_dual_mode);
        assert_eq!(deserialized.enable_credit, account.enable_credit);
    }
}
