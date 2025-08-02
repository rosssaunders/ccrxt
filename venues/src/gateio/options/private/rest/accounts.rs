use serde::{Deserialize, Serialize};

use super::RestClient;

const OPTIONS_ACCOUNTS_ENDPOINT: &str = "/options/accounts";

/// Options account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsAccount {
    /// Total balance
    pub total: String,

    /// Unrealized PnL
    pub unrealised_pnl: String,

    /// Initial margin
    pub init_margin: String,

    /// Maintenance margin
    pub maint_margin: String,

    /// Option value
    pub option_value: String,

    /// Available balance
    pub available: String,

    /// Point balance
    pub point: String,

    /// Currency
    pub currency: String,

    /// Portfolio margin requirement
    pub portfolio_margin: String,
}

impl RestClient {
    /// Get options account information
    ///
    /// This endpoint returns options account balances and margin information.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Returns
    /// Options account information including balances and margins
    pub async fn get_options_accounts(&self) -> crate::gateio::options::Result<OptionsAccount> {
        self.get(OPTIONS_ACCOUNTS_ENDPOINT).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options_account_deserialization() {
        let json = r#"{
            "total": "10000.50000000",
            "unrealised_pnl": "250.75000000",
            "init_margin": "500.25000000",
            "maint_margin": "300.15000000",
            "option_value": "1200.80000000",
            "available": "8750.25000000",
            "point": "100.00000000",
            "currency": "USDT",
            "portfolio_margin": "450.30000000"
        }"#;

        let account: OptionsAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.total, "10000.50000000");
        assert_eq!(account.unrealised_pnl, "250.75000000");
        assert_eq!(account.init_margin, "500.25000000");
        assert_eq!(account.maint_margin, "300.15000000");
        assert_eq!(account.option_value, "1200.80000000");
        assert_eq!(account.available, "8750.25000000");
        assert_eq!(account.point, "100.00000000");
        assert_eq!(account.currency, "USDT");
        assert_eq!(account.portfolio_margin, "450.30000000");
    }

    #[test]
    fn test_options_account_negative_values_deserialization() {
        let json = r#"{
            "total": "5000.00000000",
            "unrealised_pnl": "-125.50000000",
            "init_margin": "750.00000000",
            "maint_margin": "450.00000000",
            "option_value": "800.25000000",
            "available": "3874.50000000",
            "point": "0.00000000",
            "currency": "USDT",
            "portfolio_margin": "600.00000000"
        }"#;

        let account: OptionsAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.total, "5000.00000000");
        assert_eq!(account.unrealised_pnl, "-125.50000000");
        assert_eq!(account.init_margin, "750.00000000");
        assert_eq!(account.maint_margin, "450.00000000");
        assert_eq!(account.option_value, "800.25000000");
        assert_eq!(account.available, "3874.50000000");
        assert_eq!(account.point, "0.00000000");
        assert_eq!(account.currency, "USDT");
        assert_eq!(account.portfolio_margin, "600.00000000");
    }

    #[test]
    fn test_options_account_zero_values_deserialization() {
        let json = r#"{
            "total": "0.00000000",
            "unrealised_pnl": "0.00000000",
            "init_margin": "0.00000000",
            "maint_margin": "0.00000000",
            "option_value": "0.00000000",
            "available": "0.00000000",
            "point": "0.00000000",
            "currency": "USDT",
            "portfolio_margin": "0.00000000"
        }"#;

        let account: OptionsAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.total, "0.00000000");
        assert_eq!(account.unrealised_pnl, "0.00000000");
        assert_eq!(account.init_margin, "0.00000000");
        assert_eq!(account.maint_margin, "0.00000000");
        assert_eq!(account.option_value, "0.00000000");
        assert_eq!(account.available, "0.00000000");
        assert_eq!(account.point, "0.00000000");
        assert_eq!(account.currency, "USDT");
        assert_eq!(account.portfolio_margin, "0.00000000");
    }

    #[test]
    fn test_options_account_high_precision_deserialization() {
        let json = r#"{
            "total": "99999.99999999",
            "unrealised_pnl": "12345.67890123",
            "init_margin": "5000.12345678",
            "maint_margin": "3000.98765432",
            "option_value": "15000.11111111",
            "available": "76654.10102937",
            "point": "500.55555555",
            "currency": "USDT",
            "portfolio_margin": "4500.87654321"
        }"#;

        let account: OptionsAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.total, "99999.99999999");
        assert_eq!(account.unrealised_pnl, "12345.67890123");
        assert_eq!(account.init_margin, "5000.12345678");
        assert_eq!(account.maint_margin, "3000.98765432");
        assert_eq!(account.option_value, "15000.11111111");
        assert_eq!(account.available, "76654.10102937");
        assert_eq!(account.point, "500.55555555");
        assert_eq!(account.currency, "USDT");
        assert_eq!(account.portfolio_margin, "4500.87654321");
    }

    #[test]
    fn test_options_account_btc_currency_deserialization() {
        let json = r#"{
            "total": "1.50000000",
            "unrealised_pnl": "0.00250000",
            "init_margin": "0.05000000",
            "maint_margin": "0.03000000",
            "option_value": "0.12000000",
            "available": "1.30000000",
            "point": "0.00100000",
            "currency": "BTC",
            "portfolio_margin": "0.04500000"
        }"#;

        let account: OptionsAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.total, "1.50000000");
        assert_eq!(account.unrealised_pnl, "0.00250000");
        assert_eq!(account.init_margin, "0.05000000");
        assert_eq!(account.maint_margin, "0.03000000");
        assert_eq!(account.option_value, "0.12000000");
        assert_eq!(account.available, "1.30000000");
        assert_eq!(account.point, "0.00100000");
        assert_eq!(account.currency, "BTC");
        assert_eq!(account.portfolio_margin, "0.04500000");
    }

    #[test]
    fn test_options_account_small_values_deserialization() {
        let json = r#"{
            "total": "0.00000001",
            "unrealised_pnl": "-0.00000001",
            "init_margin": "0.00000001",
            "maint_margin": "0.00000001",
            "option_value": "0.00000001",
            "available": "0.00000001",
            "point": "0.00000001",
            "currency": "ETH",
            "portfolio_margin": "0.00000001"
        }"#;

        let account: OptionsAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.total, "0.00000001");
        assert_eq!(account.unrealised_pnl, "-0.00000001");
        assert_eq!(account.init_margin, "0.00000001");
        assert_eq!(account.maint_margin, "0.00000001");
        assert_eq!(account.option_value, "0.00000001");
        assert_eq!(account.available, "0.00000001");
        assert_eq!(account.point, "0.00000001");
        assert_eq!(account.currency, "ETH");
        assert_eq!(account.portfolio_margin, "0.00000001");
    }

    #[test]
    fn test_options_account_large_loss_scenario() {
        let json = r#"{
            "total": "1000.00000000",
            "unrealised_pnl": "-500.75000000",
            "init_margin": "800.00000000",
            "maint_margin": "600.00000000",
            "option_value": "200.25000000",
            "available": "100.00000000",
            "point": "50.00000000",
            "currency": "USDT",
            "portfolio_margin": "750.00000000"
        }"#;

        let account: OptionsAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.total, "1000.00000000");
        assert_eq!(account.unrealised_pnl, "-500.75000000");
        assert_eq!(account.init_margin, "800.00000000");
        assert_eq!(account.maint_margin, "600.00000000");
        assert_eq!(account.option_value, "200.25000000");
        assert_eq!(account.available, "100.00000000");
        assert_eq!(account.point, "50.00000000");
        assert_eq!(account.currency, "USDT");
        assert_eq!(account.portfolio_margin, "750.00000000");
    }

    #[test]
    fn test_options_account_serialization() {
        let account = OptionsAccount {
            total: "10000.00000000".to_string(),
            unrealised_pnl: "250.75000000".to_string(),
            init_margin: "500.25000000".to_string(),
            maint_margin: "300.15000000".to_string(),
            option_value: "1200.80000000".to_string(),
            available: "8750.25000000".to_string(),
            point: "100.00000000".to_string(),
            currency: "USDT".to_string(),
            portfolio_margin: "450.30000000".to_string(),
        };

        let json = serde_json::to_value(&account).unwrap();
        assert_eq!(json["total"], "10000.00000000");
        assert_eq!(json["unrealised_pnl"], "250.75000000");
        assert_eq!(json["init_margin"], "500.25000000");
        assert_eq!(json["maint_margin"], "300.15000000");
        assert_eq!(json["option_value"], "1200.80000000");
        assert_eq!(json["available"], "8750.25000000");
        assert_eq!(json["point"], "100.00000000");
        assert_eq!(json["currency"], "USDT");
        assert_eq!(json["portfolio_margin"], "450.30000000");
    }

    #[test]
    fn test_options_account_different_currencies() {
        let currencies = vec!["USDT", "BTC", "ETH", "USDC", "DAI"];

        for currency in currencies {
            let json = format!(
                r#"{{
                "total": "1000.00000000",
                "unrealised_pnl": "50.00000000",
                "init_margin": "100.00000000",
                "maint_margin": "60.00000000",
                "option_value": "200.00000000",
                "available": "800.00000000",
                "point": "10.00000000",
                "currency": "{}",
                "portfolio_margin": "80.00000000"
            }}"#,
                currency
            );

            let account: OptionsAccount = serde_json::from_str(&json).unwrap();
            assert_eq!(account.currency, currency);
            assert_eq!(account.total, "1000.00000000");
        }
    }

    #[test]
    fn test_options_account_extreme_negative_pnl() {
        let json = r#"{
            "total": "10000.00000000",
            "unrealised_pnl": "-9999.99999999",
            "init_margin": "5000.00000000",
            "maint_margin": "3000.00000000",
            "option_value": "0.00000001",
            "available": "0.00000001",
            "point": "0.00000000",
            "currency": "USDT",
            "portfolio_margin": "4800.00000000"
        }"#;

        let account: OptionsAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.total, "10000.00000000");
        assert_eq!(account.unrealised_pnl, "-9999.99999999");
        assert_eq!(account.init_margin, "5000.00000000");
        assert_eq!(account.maint_margin, "3000.00000000");
        assert_eq!(account.option_value, "0.00000001");
        assert_eq!(account.available, "0.00000001");
        assert_eq!(account.point, "0.00000000");
        assert_eq!(account.currency, "USDT");
        assert_eq!(account.portfolio_margin, "4800.00000000");
    }

    #[test]
    fn test_options_account_no_positions_scenario() {
        let json = r#"{
            "total": "5000.00000000",
            "unrealised_pnl": "0.00000000",
            "init_margin": "0.00000000",
            "maint_margin": "0.00000000",
            "option_value": "0.00000000",
            "available": "5000.00000000",
            "point": "25.00000000",
            "currency": "USDT",
            "portfolio_margin": "0.00000000"
        }"#;

        let account: OptionsAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.total, "5000.00000000");
        assert_eq!(account.unrealised_pnl, "0.00000000");
        assert_eq!(account.init_margin, "0.00000000");
        assert_eq!(account.maint_margin, "0.00000000");
        assert_eq!(account.option_value, "0.00000000");
        assert_eq!(account.available, "5000.00000000");
        assert_eq!(account.point, "25.00000000");
        assert_eq!(account.currency, "USDT");
        assert_eq!(account.portfolio_margin, "0.00000000");
    }

    #[test]
    fn test_options_account_high_margin_utilization() {
        let json = r#"{
            "total": "10000.00000000",
            "unrealised_pnl": "100.00000000",
            "init_margin": "8500.00000000",
            "maint_margin": "7000.00000000",
            "option_value": "9000.00000000",
            "available": "1500.00000000",
            "point": "75.00000000",
            "currency": "USDT",
            "portfolio_margin": "8200.00000000"
        }"#;

        let account: OptionsAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.total, "10000.00000000");
        assert_eq!(account.unrealised_pnl, "100.00000000");
        assert_eq!(account.init_margin, "8500.00000000");
        assert_eq!(account.maint_margin, "7000.00000000");
        assert_eq!(account.option_value, "9000.00000000");
        assert_eq!(account.available, "1500.00000000");
        assert_eq!(account.point, "75.00000000");
        assert_eq!(account.currency, "USDT");
        assert_eq!(account.portfolio_margin, "8200.00000000");
    }

    #[test]
    fn test_options_account_complex_portfolio_scenario() {
        let json = r#"{
            "total": "50000.12345678",
            "unrealised_pnl": "1250.87654321",
            "init_margin": "12500.25000000",
            "maint_margin": "8750.75000000",
            "option_value": "15750.50000000",
            "available": "36249.37345678",
            "point": "250.33333333",
            "currency": "USDT",
            "portfolio_margin": "11000.50000000"
        }"#;

        let account: OptionsAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.total, "50000.12345678");
        assert_eq!(account.unrealised_pnl, "1250.87654321");
        assert_eq!(account.init_margin, "12500.25000000");
        assert_eq!(account.maint_margin, "8750.75000000");
        assert_eq!(account.option_value, "15750.50000000");
        assert_eq!(account.available, "36249.37345678");
        assert_eq!(account.point, "250.33333333");
        assert_eq!(account.currency, "USDT");
        assert_eq!(account.portfolio_margin, "11000.50000000");
    }
}
