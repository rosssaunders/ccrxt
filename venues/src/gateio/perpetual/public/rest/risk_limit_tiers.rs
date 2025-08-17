use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for futures risk limit tiers
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FuturesRiskLimitTiersRequest {
    /// Settlement currency
    pub settle: String,

    /// Contract name
    pub contract: String,

    /// List offset (default 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    /// Maximum number of records to return (1-500, default 10)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Risk limit tier information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskLimitTier {
    /// Tier level
    pub tier: i32,

    /// Maximum position size for this tier
    pub risk_limit: String,

    /// Initial margin rate
    pub initial_rate: String,

    /// Maintenance margin rate
    pub maintenance_rate: String,
}

impl RestClient {
    /// List risk limit tiers
    ///
    /// Retrieves risk limit tiers for a specific futures contract.
    ///
    /// [docs](https://www.gate.com/docs/developers/apiv4/#list-risk-limit-tiers)
    ///
    /// Higher tiers require higher margin rates but allow larger positions.
    pub async fn get_futures_risk_limit_tiers(
        &self,
        params: FuturesRiskLimitTiersRequest,
    ) -> crate::gateio::perpetual::RestResult<Vec<RiskLimitTier>> {
        let endpoint = format!("/futures/{}/risk_limit_tiers", params.settle);
        self.get_with_query(&endpoint, Some(&params)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_futures_risk_limit_tiers_request_minimal() {
        let request = FuturesRiskLimitTiersRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            offset: None,
            limit: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 2); // Only settle and contract
        assert!(!obj.contains_key("offset"));
        assert!(!obj.contains_key("limit"));
    }

    #[test]
    fn test_futures_risk_limit_tiers_request_full() {
        let request = FuturesRiskLimitTiersRequest {
            settle: "USDT".to_string(),
            contract: "ETH_USDT".to_string(),
            offset: Some(10),
            limit: Some(50),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "ETH_USDT");
        assert_eq!(json["offset"], 10);
        assert_eq!(json["limit"], 50);

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 4);
    }

    #[test]
    fn test_different_settlement_currencies() {
        let currencies = vec!["USDT", "BTC", "USD"];

        for settle in currencies {
            let request = FuturesRiskLimitTiersRequest {
                settle: settle.to_string(),
                contract: "BTC_USDT".to_string(),
                offset: None,
                limit: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["settle"], settle);
        }
    }

    #[test]
    fn test_different_contract_pairs() {
        let contracts = vec![
            "BTC_USDT",
            "ETH_USDT",
            "ADA_USDT",
            "SOL_USDT",
            "MATIC_USDT",
            "DOT_USDT",
            "AVAX_USDT",
            "LINK_USDT",
        ];

        for contract in contracts {
            let request = FuturesRiskLimitTiersRequest {
                settle: "USDT".to_string(),
                contract: contract.to_string(),
                offset: None,
                limit: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["contract"], contract);
        }
    }

    #[test]
    fn test_offset_variations() {
        let offsets = vec![0, 5, 10, 25, 50, 100];

        for offset in offsets {
            let request = FuturesRiskLimitTiersRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                offset: Some(offset),
                limit: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["offset"], offset);
            assert!(offset >= 0);
        }
    }

    #[test]
    fn test_limit_variations() {
        let limits = vec![1, 5, 10, 25, 50, 100, 250, 500];

        for limit in limits {
            let request = FuturesRiskLimitTiersRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                offset: None,
                limit: Some(limit),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["limit"], limit);
            assert!((1..=500).contains(&limit));
        }
    }

    #[test]
    fn test_pagination_scenarios() {
        let pagination_configs = vec![
            (10, 0, "First page"),
            (10, 10, "Second page"),
            (25, 50, "Third page"),
            (50, 100, "Fourth page"),
        ];

        for (limit, offset, _description) in pagination_configs {
            let request = FuturesRiskLimitTiersRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                offset: Some(offset),
                limit: Some(limit),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["limit"], limit);
            assert_eq!(json["offset"], offset);
        }
    }

    #[test]
    fn test_risk_limit_tier_deserialization() {
        let json = r#"{
            "tier": 1,
            "risk_limit": "1000000",
            "initial_rate": "0.01",
            "maintenance_rate": "0.005"
        }"#;

        let tier: RiskLimitTier = serde_json::from_str(json).unwrap();
        assert_eq!(tier.tier, 1);
        assert_eq!(tier.risk_limit, "1000000");
        assert_eq!(tier.initial_rate, "0.01");
        assert_eq!(tier.maintenance_rate, "0.005");
    }

    #[test]
    fn test_btc_risk_limit_tiers() {
        let btc_tiers = vec![
            (1, "1000000", "0.01", "0.005", "Tier 1 - Safest"),
            (2, "5000000", "0.025", "0.01", "Tier 2 - Conservative"),
            (3, "10000000", "0.05", "0.025", "Tier 3 - Moderate"),
            (4, "25000000", "0.1", "0.05", "Tier 4 - Aggressive"),
            (5, "50000000", "0.15", "0.075", "Tier 5 - High risk"),
        ];

        for (tier, risk_limit, initial_rate, maintenance_rate, _description) in btc_tiers {
            let json = format!(
                r#"{{
                "tier": {},
                "risk_limit": "{}",
                "initial_rate": "{}",
                "maintenance_rate": "{}"
            }}"#,
                tier, risk_limit, initial_rate, maintenance_rate
            );

            let tier_data: RiskLimitTier = serde_json::from_str(&json).unwrap();
            assert_eq!(tier_data.tier, tier);
            assert_eq!(tier_data.risk_limit, risk_limit);
            assert_eq!(tier_data.initial_rate, initial_rate);
            assert_eq!(tier_data.maintenance_rate, maintenance_rate);

            // Verify rates are reasonable
            let initial: f64 = tier_data.initial_rate.parse().unwrap();
            let maintenance: f64 = tier_data.maintenance_rate.parse().unwrap();
            assert!(initial > maintenance); // Initial should be higher than maintenance
            assert!(initial > 0.0 && initial < 1.0); // Rates should be percentages
            assert!(maintenance > 0.0 && maintenance < 1.0);
        }
    }

    #[test]
    fn test_eth_risk_limit_tiers() {
        let eth_tiers = vec![
            (1, "500000", "0.015", "0.0075", "ETH Tier 1"),
            (2, "2500000", "0.03", "0.015", "ETH Tier 2"),
            (3, "5000000", "0.06", "0.03", "ETH Tier 3"),
            (4, "12500000", "0.12", "0.06", "ETH Tier 4"),
            (5, "25000000", "0.18", "0.09", "ETH Tier 5"),
        ];

        for (tier, risk_limit, initial_rate, maintenance_rate, _description) in eth_tiers {
            let json = format!(
                r#"{{
                "tier": {},
                "risk_limit": "{}",
                "initial_rate": "{}",
                "maintenance_rate": "{}"
            }}"#,
                tier, risk_limit, initial_rate, maintenance_rate
            );

            let tier_data: RiskLimitTier = serde_json::from_str(&json).unwrap();
            assert_eq!(tier_data.tier, tier);
            assert_eq!(tier_data.risk_limit, risk_limit);

            // Verify ETH tiers have appropriate risk limits
            let limit: f64 = tier_data.risk_limit.parse().unwrap();
            assert!(limit >= 500000.0); // Minimum ETH risk limit
            assert!(limit <= 25000000.0); // Maximum ETH risk limit
        }
    }

    #[test]
    fn test_altcoin_risk_limit_tiers() {
        let altcoin_scenarios = vec![
            ("ADA_USDT", 1, "100000", "0.02", "0.01"),
            ("SOL_USDT", 1, "200000", "0.025", "0.0125"),
            ("MATIC_USDT", 1, "150000", "0.03", "0.015"),
            ("DOT_USDT", 1, "300000", "0.02", "0.01"),
            ("AVAX_USDT", 1, "250000", "0.025", "0.0125"),
            ("LINK_USDT", 1, "400000", "0.02", "0.01"),
        ];

        for (contract, tier, risk_limit, initial_rate, maintenance_rate) in altcoin_scenarios {
            let request = FuturesRiskLimitTiersRequest {
                settle: "USDT".to_string(),
                contract: contract.to_string(),
                offset: None,
                limit: Some(10),
            };

            let request_json = serde_json::to_value(&request).unwrap();
            assert_eq!(request_json["contract"], contract);

            let tier_json = format!(
                r#"{{
                "tier": {},
                "risk_limit": "{}",
                "initial_rate": "{}",
                "maintenance_rate": "{}"
            }}"#,
                tier, risk_limit, initial_rate, maintenance_rate
            );

            let tier_data: RiskLimitTier = serde_json::from_str(&tier_json).unwrap();
            assert_eq!(tier_data.tier, tier);

            // Altcoins typically have lower risk limits
            let limit: f64 = tier_data.risk_limit.parse().unwrap();
            assert!(limit >= 100000.0);
            assert!(limit <= 500000.0);
        }
    }

    #[test]
    fn test_progressive_tier_margin_increases() {
        let progressive_tiers = vec![
            (1, "1000000", "0.01", "0.005"),
            (2, "2000000", "0.02", "0.01"),
            (3, "5000000", "0.05", "0.025"),
            (4, "10000000", "0.1", "0.05"),
            (5, "20000000", "0.2", "0.1"),
        ];

        let mut prev_risk_limit = 0.0;
        let mut prev_initial_rate = 0.0;
        let mut prev_maintenance_rate = 0.0;

        for (tier, risk_limit, initial_rate, maintenance_rate) in progressive_tiers {
            let json = format!(
                r#"{{
                "tier": {},
                "risk_limit": "{}",
                "initial_rate": "{}",
                "maintenance_rate": "{}"
            }}"#,
                tier, risk_limit, initial_rate, maintenance_rate
            );

            let tier_data: RiskLimitTier = serde_json::from_str(&json).unwrap();

            let current_risk_limit: f64 = tier_data.risk_limit.parse().unwrap();
            let current_initial_rate: f64 = tier_data.initial_rate.parse().unwrap();
            let current_maintenance_rate: f64 = tier_data.maintenance_rate.parse().unwrap();

            // Verify progressive increases
            if tier > 1 {
                assert!(current_risk_limit > prev_risk_limit); // Risk limit should increase
                assert!(current_initial_rate >= prev_initial_rate); // Rates should increase or stay same
                assert!(current_maintenance_rate >= prev_maintenance_rate);
            }

            prev_risk_limit = current_risk_limit;
            prev_initial_rate = current_initial_rate;
            prev_maintenance_rate = current_maintenance_rate;
        }
    }

    #[test]
    fn test_high_tier_risk_scenarios() {
        let high_tier_scenarios = vec![
            (10, "100000000", "0.5", "0.25", "Very high tier"),
            (15, "500000000", "0.75", "0.5", "Extreme tier"),
            (20, "1000000000", "1.0", "0.75", "Maximum tier"),
        ];

        for (tier, risk_limit, initial_rate, maintenance_rate, _description) in high_tier_scenarios
        {
            let json = format!(
                r#"{{
                "tier": {},
                "risk_limit": "{}",
                "initial_rate": "{}",
                "maintenance_rate": "{}"
            }}"#,
                tier, risk_limit, initial_rate, maintenance_rate
            );

            let tier_data: RiskLimitTier = serde_json::from_str(&json).unwrap();
            assert_eq!(tier_data.tier, tier);

            // High tiers should have very high risk limits and margin requirements
            let limit: f64 = tier_data.risk_limit.parse().unwrap();
            let initial: f64 = tier_data.initial_rate.parse().unwrap();
            let maintenance: f64 = tier_data.maintenance_rate.parse().unwrap();

            assert!(limit >= 100000000.0); // >= 100M
            assert!(initial >= 0.5); // >= 50% margin
            assert!(maintenance >= 0.25); // >= 25% maintenance
        }
    }

    #[test]
    fn test_margin_rate_relationships() {
        let rate_scenarios = vec![
            ("0.01", "0.005", "Low margin requirements"),
            ("0.05", "0.025", "Medium margin requirements"),
            ("0.1", "0.05", "High margin requirements"),
            ("0.2", "0.1", "Very high margin requirements"),
            ("0.5", "0.25", "Extreme margin requirements"),
        ];

        for (initial_rate, maintenance_rate, _description) in rate_scenarios {
            let json = format!(
                r#"{{
                "tier": 1,
                "risk_limit": "1000000",
                "initial_rate": "{}",
                "maintenance_rate": "{}"
            }}"#,
                initial_rate, maintenance_rate
            );

            let tier_data: RiskLimitTier = serde_json::from_str(&json).unwrap();

            let initial: f64 = tier_data.initial_rate.parse().unwrap();
            let maintenance: f64 = tier_data.maintenance_rate.parse().unwrap();

            // Verify rate relationships
            assert!(initial > maintenance); // Initial should always be higher
            assert!(initial / maintenance >= 1.5); // Initial should be at least 1.5x maintenance
            assert!(initial / maintenance <= 3.0); // But not more than 3x (reasonable range)
        }
    }

    #[test]
    fn test_risk_limit_tier_validation() {
        let validation_scenarios = vec![
            (1, "1000000", "0.01", "0.005", true, "Valid tier"),
            (0, "1000000", "0.01", "0.005", false, "Invalid tier 0"),
            (1, "0", "0.01", "0.005", false, "Zero risk limit"),
            (1, "1000000", "0", "0.005", false, "Zero initial rate"),
            (1, "1000000", "0.01", "0", false, "Zero maintenance rate"),
            (
                1,
                "1000000",
                "0.005",
                "0.01",
                false,
                "Maintenance > initial",
            ),
        ];

        for (tier, risk_limit, initial_rate, maintenance_rate, should_be_valid, _description) in
            validation_scenarios
        {
            let json = format!(
                r#"{{
                "tier": {},
                "risk_limit": "{}",
                "initial_rate": "{}",
                "maintenance_rate": "{}"
            }}"#,
                tier, risk_limit, initial_rate, maintenance_rate
            );

            let tier_data: RiskLimitTier = serde_json::from_str(&json).unwrap();

            let tier_num = tier_data.tier;
            let limit: f64 = tier_data.risk_limit.parse().unwrap();
            let initial: f64 = tier_data.initial_rate.parse().unwrap();
            let maintenance: f64 = tier_data.maintenance_rate.parse().unwrap();

            if should_be_valid {
                assert!(tier_num > 0);
                assert!(limit > 0.0);
                assert!(initial > 0.0);
                assert!(maintenance > 0.0);
                assert!(initial > maintenance);
            } else {
                // For invalid scenarios, we can still parse but values are inappropriate
                let is_valid = tier_num > 0
                    && limit > 0.0
                    && initial > 0.0
                    && maintenance > 0.0
                    && initial > maintenance;
                assert!(!is_valid);
            }
        }
    }

    #[test]
    fn test_institutional_vs_retail_tiers() {
        let tier_comparisons = vec![
            (1, "500000", "0.01", "0.005", "Retail tier 1"),
            (2, "2000000", "0.02", "0.01", "Retail tier 2"),
            (5, "20000000", "0.1", "0.05", "Institutional tier"),
            (10, "100000000", "0.3", "0.15", "Large institutional tier"),
        ];

        for (tier, risk_limit, initial_rate, maintenance_rate, category) in tier_comparisons {
            let json = format!(
                r#"{{
                "tier": {},
                "risk_limit": "{}",
                "initial_rate": "{}",
                "maintenance_rate": "{}"
            }}"#,
                tier, risk_limit, initial_rate, maintenance_rate
            );

            let tier_data: RiskLimitTier = serde_json::from_str(&json).unwrap();

            let limit: f64 = tier_data.risk_limit.parse().unwrap();
            let initial: f64 = tier_data.initial_rate.parse().unwrap();

            if category.contains("Retail") {
                assert!(limit <= 5000000.0); // Retail caps
                assert!(initial <= 0.05); // Lower margin requirements
            } else if category.contains("institutional") {
                assert!(limit >= 20000000.0); // Institutional minimums
                assert!(initial >= 0.1); // Higher margin requirements
            }
        }
    }

    #[test]
    fn test_cross_asset_tier_comparison() {
        let cross_asset_tiers = vec![
            ("BTC_USDT", 1, "2000000", "0.01", "0.005"),
            ("ETH_USDT", 1, "1000000", "0.015", "0.0075"),
            ("ADA_USDT", 1, "200000", "0.025", "0.0125"),
            ("SOL_USDT", 1, "300000", "0.02", "0.01"),
        ];

        for (contract, tier, risk_limit, initial_rate, maintenance_rate) in cross_asset_tiers {
            let request = FuturesRiskLimitTiersRequest {
                settle: "USDT".to_string(),
                contract: contract.to_string(),
                offset: None,
                limit: Some(5),
            };

            let request_json = serde_json::to_value(&request).unwrap();
            assert_eq!(request_json["contract"], contract);

            let tier_json = format!(
                r#"{{
                "tier": {},
                "risk_limit": "{}",
                "initial_rate": "{}",
                "maintenance_rate": "{}"
            }}"#,
                tier, risk_limit, initial_rate, maintenance_rate
            );

            let tier_data: RiskLimitTier = serde_json::from_str(&tier_json).unwrap();
            let limit: f64 = tier_data.risk_limit.parse().unwrap();
            let initial: f64 = tier_data.initial_rate.parse().unwrap();

            // BTC should have highest risk limits, altcoins lowest
            if contract == "BTC_USDT" {
                assert!(limit >= 1000000.0);
                assert!(initial <= 0.02);
            } else if contract.starts_with("ADA") || contract.starts_with("SOL") {
                assert!(limit <= 500000.0);
                assert!(initial >= 0.02);
            }
        }
    }

    #[test]
    fn test_tier_leverage_calculations() {
        let leverage_scenarios = vec![
            ("0.01", 100.0, "100x leverage"),
            ("0.02", 50.0, "50x leverage"),
            ("0.05", 20.0, "20x leverage"),
            ("0.1", 10.0, "10x leverage"),
            ("0.2", 5.0, "5x leverage"),
            ("0.5", 2.0, "2x leverage"),
        ];

        for (initial_rate, expected_leverage, _description) in leverage_scenarios {
            let json = format!(
                r#"{{
                "tier": 1,
                "risk_limit": "1000000",
                "initial_rate": "{}",
                "maintenance_rate": "0.005"
            }}"#,
                initial_rate
            );

            let tier_data: RiskLimitTier = serde_json::from_str(&json).unwrap();
            let initial: f64 = tier_data.initial_rate.parse().unwrap();

            // Calculate max leverage from initial margin rate
            let calculated_leverage = 1.0 / initial;
            let tolerance = 0.1; // 10% tolerance

            assert!((calculated_leverage - expected_leverage).abs() < tolerance);
        }
    }

    #[test]
    fn test_risk_management_scenarios() {
        let risk_scenarios = vec![
            (1, "1000000", "0.01", "0.005", "Low risk"),
            (3, "5000000", "0.05", "0.025", "Medium risk"),
            (5, "20000000", "0.15", "0.075", "High risk"),
            (8, "50000000", "0.3", "0.15", "Very high risk"),
        ];

        for (tier, risk_limit, initial_rate, maintenance_rate, risk_level) in risk_scenarios {
            let json = format!(
                r#"{{
                "tier": {},
                "risk_limit": "{}",
                "initial_rate": "{}",
                "maintenance_rate": "{}"
            }}"#,
                tier, risk_limit, initial_rate, maintenance_rate
            );

            let tier_data: RiskLimitTier = serde_json::from_str(&json).unwrap();

            let limit: f64 = tier_data.risk_limit.parse().unwrap();
            let initial: f64 = tier_data.initial_rate.parse().unwrap();
            let maintenance: f64 = tier_data.maintenance_rate.parse().unwrap();

            // Verify risk increases with tier
            if risk_level == "Low risk" {
                assert!(limit <= 2000000.0);
                assert!(initial <= 0.02);
            } else if risk_level == "Very high risk" {
                assert!(limit >= 50000000.0);
                assert!(initial >= 0.3);
            }

            // Margin buffer should be reasonable
            let margin_buffer = initial - maintenance;
            assert!(margin_buffer > 0.0);
            assert!(margin_buffer <= initial * 0.6); // Buffer shouldn't be too large
        }
    }

    #[test]
    fn test_high_precision_rates() {
        let json = r#"{
            "tier": 1,
            "risk_limit": "1000000",
            "initial_rate": "0.012345678",
            "maintenance_rate": "0.006789123"
        }"#;

        let tier: RiskLimitTier = serde_json::from_str(json).unwrap();

        // Verify precision is maintained
        assert_eq!(tier.initial_rate, "0.012345678");
        assert_eq!(tier.maintenance_rate, "0.006789123");

        let initial: f64 = tier.initial_rate.parse().unwrap();
        let maintenance: f64 = tier.maintenance_rate.parse().unwrap();
        assert!(initial > maintenance);
    }

    #[test]
    fn test_large_risk_limits() {
        let large_limits = vec![
            ("100000000", "100M limit"),
            ("500000000", "500M limit"),
            ("1000000000", "1B limit"),
            ("5000000000", "5B limit"),
        ];

        for (risk_limit, _description) in large_limits {
            let json = format!(
                r#"{{
                "tier": 10,
                "risk_limit": "{}",
                "initial_rate": "0.2",
                "maintenance_rate": "0.1"
            }}"#,
                risk_limit
            );

            let tier: RiskLimitTier = serde_json::from_str(&json).unwrap();
            assert_eq!(tier.risk_limit, risk_limit);

            let limit: f64 = tier.risk_limit.parse().unwrap();
            assert!(limit >= 100000000.0);
        }
    }

    #[test]
    fn test_edge_case_tier_numbers() {
        let edge_tiers = vec![1, 5, 10, 15, 20, 25, 50, 100];

        for tier_num in edge_tiers {
            let json = format!(
                r#"{{
                "tier": {},
                "risk_limit": "1000000",
                "initial_rate": "0.01",
                "maintenance_rate": "0.005"
            }}"#,
                tier_num
            );

            let tier: RiskLimitTier = serde_json::from_str(&json).unwrap();
            assert_eq!(tier.tier, tier_num);
            assert!(tier.tier > 0);
        }
    }

    #[test]
    fn test_clone_behavior() {
        let request = FuturesRiskLimitTiersRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            offset: Some(10),
            limit: Some(50),
        };

        let cloned = request.clone();
        assert_eq!(cloned.settle, request.settle);
        assert_eq!(cloned.contract, request.contract);
        assert_eq!(cloned.offset, request.offset);
        assert_eq!(cloned.limit, request.limit);
    }

    #[test]
    fn test_debug_output() {
        let tier = RiskLimitTier {
            tier: 1,
            risk_limit: "1000000".to_string(),
            initial_rate: "0.01".to_string(),
            maintenance_rate: "0.005".to_string(),
        };

        let debug_str = format!("{:?}", tier);
        assert!(debug_str.contains("RiskLimitTier"));
        assert!(debug_str.contains("1"));
        assert!(debug_str.contains("1000000"));
        assert!(debug_str.contains("0.01"));
    }

    #[test]
    fn test_serialization_round_trip() {
        let tier = RiskLimitTier {
            tier: 1,
            risk_limit: "1000000".to_string(),
            initial_rate: "0.01".to_string(),
            maintenance_rate: "0.005".to_string(),
        };

        let json = serde_json::to_string(&tier).unwrap();
        let deserialized: RiskLimitTier = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.tier, tier.tier);
        assert_eq!(deserialized.risk_limit, tier.risk_limit);
        assert_eq!(deserialized.initial_rate, tier.initial_rate);
        assert_eq!(deserialized.maintenance_rate, tier.maintenance_rate);
    }
}
