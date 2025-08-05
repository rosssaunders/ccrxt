use serde::{Deserialize, Serialize};

use super::RestClient;

const TOTAL_BALANCE_ENDPOINT: &str = "/wallet/total_balance";

/// Request parameters for total balance
#[derive(Debug, Clone, Serialize, Default)]
pub struct TotalBalanceRequest {
    /// Currency filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

/// Total balance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotalBalance {
    /// Details of each currency
    pub details: std::collections::HashMap<String, CurrencyBalance>,

    /// Total balance in USDT
    pub total: TotalBalanceValue,
}

/// Currency balance details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyBalance {
    /// Available balance
    pub available: String,

    /// Unrealized PnL
    pub unrealised_pnl: String,

    /// Borrowed amount
    pub borrowed: String,
}

/// Total balance value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotalBalanceValue {
    /// Currency (usually USDT)
    pub currency: String,

    /// Total amount
    pub amount: String,
}

impl RestClient {
    /// Get total balance
    ///
    /// This endpoint returns the total balance across all accounts.
    ///
    /// [docs]: https://www.gate.com/docs/developers/apiv4/#query-account-book
    ///
    /// Rate limit: varies by endpoint type
    ///
    /// # Arguments
    /// * `params` - The total balance request parameters
    ///
    /// # Returns
    /// Total balance information across all accounts
    pub async fn get_total_balance(
        &self,
        params: TotalBalanceRequest,
    ) -> crate::gateio::spot::RestResult<TotalBalance> {
        self.get_with_query(TOTAL_BALANCE_ENDPOINT, &params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_balance_request_default() {
        let request = TotalBalanceRequest::default();

        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty());
    }

    #[test]
    fn test_total_balance_request_with_currency() {
        let request = TotalBalanceRequest {
            currency: Some("BTC".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "BTC");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 1);
    }

    #[test]
    fn test_total_balance_deserialization() {
        let json = r#"{
            "details": {
                "BTC": {
                    "available": "1.5",
                    "unrealised_pnl": "0.05",
                    "borrowed": "0"
                },
                "USDT": {
                    "available": "1000.0",
                    "unrealised_pnl": "0",
                    "borrowed": "500.0"
                }
            },
            "total": {
                "currency": "USDT",
                "amount": "45000.0"
            }
        }"#;

        let balance: TotalBalance = serde_json::from_str(json).unwrap();
        assert_eq!(balance.details.len(), 2);
        assert!(balance.details.contains_key("BTC"));
        assert!(balance.details.contains_key("USDT"));
        assert_eq!(balance.total.currency, "USDT");
        assert_eq!(balance.total.amount, "45000.0");

        let btc_balance = &balance.details["BTC"];
        assert_eq!(btc_balance.available, "1.5");
        assert_eq!(btc_balance.unrealised_pnl, "0.05");
        assert_eq!(btc_balance.borrowed, "0");

        let usdt_balance = &balance.details["USDT"];
        assert_eq!(usdt_balance.available, "1000.0");
        assert_eq!(usdt_balance.unrealised_pnl, "0");
        assert_eq!(usdt_balance.borrowed, "500.0");
    }

    #[test]
    fn test_realistic_total_balance_scenario() {
        let json = r#"{
            "details": {
                "BTC": {
                    "available": "0.12345678",
                    "unrealised_pnl": "125.50",
                    "borrowed": "0"
                },
                "ETH": {
                    "available": "5.25",
                    "unrealised_pnl": "-45.25",
                    "borrowed": "0"
                },
                "USDT": {
                    "available": "15000.0",
                    "unrealised_pnl": "0",
                    "borrowed": "1000.0"
                },
                "DOT": {
                    "available": "150.0",
                    "unrealised_pnl": "12.30",
                    "borrowed": "0"
                }
            },
            "total": {
                "currency": "USDT",
                "amount": "25430.85"
            }
        }"#;

        let balance: TotalBalance = serde_json::from_str(json).unwrap();
        assert_eq!(balance.details.len(), 4);
        assert_eq!(balance.total.amount, "25430.85");

        // Check BTC with positive PnL
        let btc = &balance.details["BTC"];
        assert_eq!(btc.available, "0.12345678");
        assert_eq!(btc.unrealised_pnl, "125.50");
        assert_eq!(btc.borrowed, "0");

        // Check ETH with negative PnL
        let eth = &balance.details["ETH"];
        assert_eq!(eth.unrealised_pnl, "-45.25");

        // Check USDT with borrowed amount
        let usdt = &balance.details["USDT"];
        assert_eq!(usdt.borrowed, "1000.0");
    }

    #[test]
    fn test_clone_behavior() {
        let request = TotalBalanceRequest {
            currency: Some("BTC".to_string()),
        };
        let cloned = request.clone();
        assert_eq!(cloned.currency, request.currency);
    }

    #[test]
    fn test_debug_output() {
        let balance = TotalBalance {
            details: std::collections::HashMap::new(),
            total: TotalBalanceValue {
                currency: "USDT".to_string(),
                amount: "1000.0".to_string(),
            },
        };

        let debug_str = format!("{:?}", balance);
        assert!(debug_str.contains("TotalBalance"));
        assert!(debug_str.contains("USDT"));
        assert!(debug_str.contains("1000.0"));
    }
}
