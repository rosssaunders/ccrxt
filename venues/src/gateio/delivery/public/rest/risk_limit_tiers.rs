use serde::{Deserialize, Serialize};

use super::RestClient;

const DELIVERY_RISK_LIMIT_TIERS_ENDPOINT: &str = "/delivery/{}/risk_limit_tiers";

/// Request parameters for delivery risk limit tiers
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeliveryRiskLimitTiersRequest {
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
pub struct DeliveryRiskLimitTier {
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
    /// List delivery risk limit tiers
    ///
    /// Retrieves risk limit tiers for a specific delivery contract.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#list-risk-limit-tiers-2>
    /// Higher tiers require higher margin rates but allow larger positions.
    pub async fn get_delivery_risk_limit_tiers(
        &self,
        params: DeliveryRiskLimitTiersRequest,
    ) -> crate::gateio::delivery::Result<Vec<DeliveryRiskLimitTier>> {
        let endpoint = DELIVERY_RISK_LIMIT_TIERS_ENDPOINT.replace("{}", &params.settle);
        self.get_with_query(&endpoint, Some(&params)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delivery_risk_limit_tiers_request_minimal() {
        let request = DeliveryRiskLimitTiersRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT_20241227".to_string(),
            offset: None,
            limit: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT_20241227");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 2); // Only settle and contract
        assert!(!obj.contains_key("offset"));
        assert!(!obj.contains_key("limit"));
    }

    #[test]
    fn test_delivery_risk_limit_tiers_request_full() {
        let request = DeliveryRiskLimitTiersRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT_20241227".to_string(),
            offset: Some(10),
            limit: Some(50),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT_20241227");
        assert_eq!(json["offset"], 10);
        assert_eq!(json["limit"], 50);
    }

    #[test]
    fn test_pagination_scenarios() {
        let pagination_cases = vec![
            (0, 10, "First page"),
            (10, 10, "Second page"),
            (20, 20, "Third page, larger size"),
            (0, 100, "Large page size"),
            (0, 500, "Maximum page size"),
        ];

        for (offset, limit, _description) in pagination_cases {
            let request = DeliveryRiskLimitTiersRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT_20241227".to_string(),
                offset: Some(offset),
                limit: Some(limit),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["offset"], offset);
            assert_eq!(json["limit"], limit);
            assert!(limit >= 1 && limit <= 500);
        }
    }

    #[test]
    fn test_delivery_risk_limit_tier_deserialization() {
        let json = r#"{
            "tier": 1,
            "risk_limit": "1000000",
            "initial_rate": "0.01",
            "maintenance_rate": "0.005"
        }"#;

        let tier: DeliveryRiskLimitTier = serde_json::from_str(json).unwrap();
        assert_eq!(tier.tier, 1);
        assert_eq!(tier.risk_limit, "1000000");
        assert_eq!(tier.initial_rate, "0.01");
        assert_eq!(tier.maintenance_rate, "0.005");
    }

    #[test]
    fn test_btc_risk_limit_tiers() {
        let btc_tiers = vec![
            r#"{"tier": 1, "risk_limit": "1000000", "initial_rate": "0.01", "maintenance_rate": "0.005"}"#,
            r#"{"tier": 2, "risk_limit": "2000000", "initial_rate": "0.015", "maintenance_rate": "0.0075"}"#,
            r#"{"tier": 3, "risk_limit": "5000000", "initial_rate": "0.02", "maintenance_rate": "0.01"}"#,
            r#"{"tier": 4, "risk_limit": "10000000", "initial_rate": "0.025", "maintenance_rate": "0.0125"}"#,
            r#"{"tier": 5, "risk_limit": "20000000", "initial_rate": "0.03", "maintenance_rate": "0.015"}"#,
        ];

        let mut previous_risk_limit = 0.0;
        let mut previous_initial_rate = 0.0;

        for (i, json_str) in btc_tiers.iter().enumerate() {
            let tier: DeliveryRiskLimitTier = serde_json::from_str(json_str).unwrap();
            
            assert_eq!(tier.tier, (i + 1) as i32);
            
            let risk_limit: f64 = tier.risk_limit.parse().unwrap();
            let initial_rate: f64 = tier.initial_rate.parse().unwrap();
            let maintenance_rate: f64 = tier.maintenance_rate.parse().unwrap();
            
            // Risk limit should increase with tiers
            assert!(risk_limit > previous_risk_limit);
            // Initial rate should increase with tiers
            assert!(initial_rate > previous_initial_rate);
            // Maintenance rate should be lower than initial rate
            assert!(maintenance_rate < initial_rate);
            // Maintenance rate should be roughly half of initial rate
            assert!((maintenance_rate - initial_rate / 2.0).abs() < 0.001);
            
            previous_risk_limit = risk_limit;
            previous_initial_rate = initial_rate;
        }
    }

    #[test]
    fn test_high_leverage_tiers() {
        let high_leverage_tiers = vec![
            r#"{"tier": 6, "risk_limit": "50000000", "initial_rate": "0.05", "maintenance_rate": "0.025"}"#,
            r#"{"tier": 7, "risk_limit": "100000000", "initial_rate": "0.1", "maintenance_rate": "0.05"}"#,
            r#"{"tier": 8, "risk_limit": "200000000", "initial_rate": "0.15", "maintenance_rate": "0.075"}"#,
        ];

        for json_str in high_leverage_tiers {
            let tier: DeliveryRiskLimitTier = serde_json::from_str(json_str).unwrap();
            
            let risk_limit: f64 = tier.risk_limit.parse().unwrap();
            let initial_rate: f64 = tier.initial_rate.parse().unwrap();
            let maintenance_rate: f64 = tier.maintenance_rate.parse().unwrap();
            
            // High tier should have large risk limits
            assert!(risk_limit >= 50000000.0);
            // High tier should have higher margin rates
            assert!(initial_rate >= 0.05);
            // Verify margin relationship
            assert!(maintenance_rate == initial_rate / 2.0);
        }
    }

    #[test]
    fn test_altcoin_risk_limit_tiers() {
        let altcoin_tiers = vec![
            (r#"{"tier": 1, "risk_limit": "50000", "initial_rate": "0.02", "maintenance_rate": "0.01"}"#, "ETH"),
            (r#"{"tier": 1, "risk_limit": "100000", "initial_rate": "0.025", "maintenance_rate": "0.0125"}"#, "ADA"),
            (r#"{"tier": 1, "risk_limit": "75000", "initial_rate": "0.03", "maintenance_rate": "0.015"}"#, "SOL"),
        ];

        for (json_str, _coin) in altcoin_tiers {
            let tier: DeliveryRiskLimitTier = serde_json::from_str(json_str).unwrap();
            
            let risk_limit: f64 = tier.risk_limit.parse().unwrap();
            let initial_rate: f64 = tier.initial_rate.parse().unwrap();
            
            // Altcoins typically have lower risk limits than BTC
            assert!(risk_limit <= 100000.0);
            // Altcoins typically have higher initial margin requirements
            assert!(initial_rate >= 0.02);
        }
    }

    #[test]
    fn test_maximum_leverage_calculation() {
        let tiers = vec![
            ("0.01", 100.0),    // 1% initial rate = 100x max leverage
            ("0.02", 50.0),     // 2% initial rate = 50x max leverage
            ("0.04", 25.0),     // 4% initial rate = 25x max leverage
            ("0.05", 20.0),     // 5% initial rate = 20x max leverage
            ("0.1", 10.0),      // 10% initial rate = 10x max leverage
        ];

        for (initial_rate_str, expected_max_leverage) in tiers {
            let tier = DeliveryRiskLimitTier {
                tier: 1,
                risk_limit: "1000000".to_string(),
                initial_rate: initial_rate_str.to_string(),
                maintenance_rate: "0.005".to_string(),
            };

            let initial_rate: f64 = tier.initial_rate.parse().unwrap();
            let max_leverage = 1.0 / initial_rate;
            
            assert_eq!(max_leverage, expected_max_leverage);
        }
    }

    #[test]
    fn test_progressive_tier_structure() {
        let json = r#"[
            {"tier": 1, "risk_limit": "1000000", "initial_rate": "0.01", "maintenance_rate": "0.005"},
            {"tier": 2, "risk_limit": "2000000", "initial_rate": "0.015", "maintenance_rate": "0.0075"},
            {"tier": 3, "risk_limit": "5000000", "initial_rate": "0.02", "maintenance_rate": "0.01"},
            {"tier": 4, "risk_limit": "10000000", "initial_rate": "0.025", "maintenance_rate": "0.0125"},
            {"tier": 5, "risk_limit": "20000000", "initial_rate": "0.03", "maintenance_rate": "0.015"}
        ]"#;

        let tiers: Vec<DeliveryRiskLimitTier> = serde_json::from_str(json).unwrap();
        assert_eq!(tiers.len(), 5);

        // Verify progressive structure
        for i in 1..tiers.len() {
            let prev_risk: f64 = tiers[i - 1].risk_limit.parse().unwrap();
            let curr_risk: f64 = tiers[i].risk_limit.parse().unwrap();
            let prev_initial: f64 = tiers[i - 1].initial_rate.parse().unwrap();
            let curr_initial: f64 = tiers[i].initial_rate.parse().unwrap();
            
            // Each tier should have higher limits and rates
            assert!(curr_risk > prev_risk);
            assert!(curr_initial > prev_initial);
            
            // Risk limit should increase more aggressively than rates
            let risk_increase = curr_risk / prev_risk;
            let rate_increase = curr_initial / prev_initial;
            assert!(risk_increase > rate_increase);
        }
    }

    #[test]
    fn test_margin_requirement_calculation() {
        let tier = DeliveryRiskLimitTier {
            tier: 3,
            risk_limit: "5000000".to_string(),
            initial_rate: "0.02".to_string(),
            maintenance_rate: "0.01".to_string(),
        };

        let position_sizes = vec![
            (1000000.0, "Within tier 3"),
            (2500000.0, "Mid tier 3"),
            (5000000.0, "Max tier 3"),
        ];

        for (position_size, _description) in position_sizes {
            let initial_rate: f64 = tier.initial_rate.parse().unwrap();
            let maintenance_rate: f64 = tier.maintenance_rate.parse().unwrap();
            let risk_limit: f64 = tier.risk_limit.parse().unwrap();
            
            // Position should be within risk limit
            assert!(position_size <= risk_limit);
            
            // Calculate margin requirements
            let initial_margin = position_size * initial_rate;
            let maintenance_margin = position_size * maintenance_rate;
            
            // Initial margin should be higher than maintenance
            assert!(initial_margin > maintenance_margin);
            
            // Verify reasonable margin amounts
            assert!(initial_margin >= position_size * 0.01); // At least 1%
            assert!(maintenance_margin >= position_size * 0.005); // At least 0.5%
        }
    }

    #[test]
    fn test_cross_margin_vs_isolated_tiers() {
        // Cross margin typically allows higher tiers
        let cross_tier = DeliveryRiskLimitTier {
            tier: 8,
            risk_limit: "200000000".to_string(),
            initial_rate: "0.15".to_string(),
            maintenance_rate: "0.075".to_string(),
        };

        // Isolated margin typically has lower tiers
        let isolated_tier = DeliveryRiskLimitTier {
            tier: 5,
            risk_limit: "20000000".to_string(),
            initial_rate: "0.03".to_string(),
            maintenance_rate: "0.015".to_string(),
        };

        let cross_limit: f64 = cross_tier.risk_limit.parse().unwrap();
        let isolated_limit: f64 = isolated_tier.risk_limit.parse().unwrap();
        
        // Cross margin should allow larger positions
        assert!(cross_limit > isolated_limit);
        
        // But requires higher margin rates at high tiers
        let cross_rate: f64 = cross_tier.initial_rate.parse().unwrap();
        let isolated_rate: f64 = isolated_tier.initial_rate.parse().unwrap();
        assert!(cross_rate > isolated_rate);
    }

    #[test]
    fn test_settlement_currency_differences() {
        let settlement_scenarios = vec![
            ("USDT", "BTC_USDT_20241227", "1000000", "0.01"),
            ("BTC", "ETH_BTC_20241227", "100", "0.02"),
        ];

        for (settle, contract, risk_limit, initial_rate) in settlement_scenarios {
            let _request = DeliveryRiskLimitTiersRequest {
                settle: settle.to_string(),
                contract: contract.to_string(),
                offset: None,
                limit: None,
            };

            let tier = DeliveryRiskLimitTier {
                tier: 1,
                risk_limit: risk_limit.to_string(),
                initial_rate: initial_rate.to_string(),
                maintenance_rate: "0.005".to_string(),
            };

            // Verify appropriate risk limits for settlement currency
            let limit_val: f64 = tier.risk_limit.parse().unwrap();
            if settle == "USDT" {
                assert!(limit_val >= 100000.0); // USDT denominated in larger numbers
            } else if settle == "BTC" {
                assert!(limit_val <= 1000.0); // BTC denominated in smaller numbers
            }
        }
    }

    #[test]
    fn test_liquidation_price_impact() {
        let tiers = vec![
            ("0.005", "Low maintenance rate, far liquidation"),
            ("0.01", "Standard maintenance rate"),
            ("0.025", "High maintenance rate, close liquidation"),
            ("0.05", "Very high maintenance rate, very close liquidation"),
        ];

        for (maintenance_rate_str, _description) in tiers {
            let tier = DeliveryRiskLimitTier {
                tier: 1,
                risk_limit: "1000000".to_string(),
                initial_rate: "0.02".to_string(),
                maintenance_rate: maintenance_rate_str.to_string(),
            };

            let maintenance_rate: f64 = tier.maintenance_rate.parse().unwrap();
            
            // Lower maintenance rate means liquidation price is further from entry
            // Higher maintenance rate means liquidation price is closer to entry
            assert!(maintenance_rate > 0.0 && maintenance_rate < 1.0);
            
            // For long position: liquidation_price = entry_price * (1 - maintenance_rate)
            // For short position: liquidation_price = entry_price * (1 + maintenance_rate)
            let long_liquidation_distance = maintenance_rate;
            let short_liquidation_distance = maintenance_rate;
            
            assert_eq!(long_liquidation_distance, short_liquidation_distance);
        }
    }

    #[test]
    fn test_different_contract_types() {
        let contract_types = vec![
            ("BTC_USDT_20241227", "Weekly"),
            ("BTC_USDT_20250328", "Quarterly"),
            ("ETH_USDT_20241227", "ETH Weekly"),
            ("SOL_USDT_20241227", "SOL Weekly"),
        ];

        for (contract, _description) in contract_types {
            let request = DeliveryRiskLimitTiersRequest {
                settle: "USDT".to_string(),
                contract: contract.to_string(),
                offset: None,
                limit: Some(10),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["contract"], contract);
        }
    }

    #[test]
    fn test_tier_transition_scenarios() {
        // When position size exceeds current tier limit
        let current_tier = DeliveryRiskLimitTier {
            tier: 2,
            risk_limit: "2000000".to_string(),
            initial_rate: "0.015".to_string(),
            maintenance_rate: "0.0075".to_string(),
        };

        let next_tier = DeliveryRiskLimitTier {
            tier: 3,
            risk_limit: "5000000".to_string(),
            initial_rate: "0.02".to_string(),
            maintenance_rate: "0.01".to_string(),
        };

        let current_limit: f64 = current_tier.risk_limit.parse().unwrap();
        let next_limit: f64 = next_tier.risk_limit.parse().unwrap();
        let current_initial: f64 = current_tier.initial_rate.parse().unwrap();
        let next_initial: f64 = next_tier.initial_rate.parse().unwrap();

        // Position at current tier limit
        let position_size = current_limit;
        
        // If we want to increase position, we need to move to next tier
        let new_position_size = current_limit + 100000.0;
        assert!(new_position_size > current_limit);
        assert!(new_position_size <= next_limit);
        
        // New tier requires higher margin
        let current_margin = position_size * current_initial;
        let new_margin = new_position_size * next_initial;
        assert!(new_margin > current_margin);
    }

    #[test]
    fn test_risk_limit_buffer() {
        let tier = DeliveryRiskLimitTier {
            tier: 4,
            risk_limit: "10000000".to_string(),
            initial_rate: "0.025".to_string(),
            maintenance_rate: "0.0125".to_string(),
        };

        let risk_limit: f64 = tier.risk_limit.parse().unwrap();
        
        // Positions close to risk limit
        let positions = vec![
            (risk_limit * 0.8, "80% of limit", true),
            (risk_limit * 0.9, "90% of limit", true),
            (risk_limit * 0.95, "95% of limit - warning zone", true),
            (risk_limit * 0.99, "99% of limit - danger zone", true),
            (risk_limit * 1.0, "At limit", true),
            (risk_limit * 1.01, "Over limit - rejected", false),
        ];

        for (position, _description, allowed) in positions {
            if allowed {
                assert!(position <= risk_limit);
            } else {
                assert!(position > risk_limit);
            }
        }
    }

    #[test]
    fn test_clone_behavior() {
        let request = DeliveryRiskLimitTiersRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT_20241227".to_string(),
            offset: Some(0),
            limit: Some(100),
        };

        let cloned = request.clone();
        assert_eq!(cloned.settle, request.settle);
        assert_eq!(cloned.contract, request.contract);
        assert_eq!(cloned.offset, request.offset);
        assert_eq!(cloned.limit, request.limit);
    }

    #[test]
    fn test_debug_output() {
        let tier = DeliveryRiskLimitTier {
            tier: 5,
            risk_limit: "20000000".to_string(),
            initial_rate: "0.03".to_string(),
            maintenance_rate: "0.015".to_string(),
        };

        let debug_str = format!("{:?}", tier);
        assert!(debug_str.contains("DeliveryRiskLimitTier"));
        assert!(debug_str.contains("tier: 5"));
        assert!(debug_str.contains("20000000"));
    }

    #[test]
    fn test_serialization_round_trip() {
        let tier = DeliveryRiskLimitTier {
            tier: 3,
            risk_limit: "5000000".to_string(),
            initial_rate: "0.02".to_string(),
            maintenance_rate: "0.01".to_string(),
        };

        let json = serde_json::to_string(&tier).unwrap();
        let deserialized: DeliveryRiskLimitTier = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.tier, tier.tier);
        assert_eq!(deserialized.risk_limit, tier.risk_limit);
        assert_eq!(deserialized.initial_rate, tier.initial_rate);
        assert_eq!(deserialized.maintenance_rate, tier.maintenance_rate);
    }

    #[test]
    fn test_tier_list_response() {
        let json = r#"[
            {"tier": 1, "risk_limit": "1000000", "initial_rate": "0.01", "maintenance_rate": "0.005"},
            {"tier": 2, "risk_limit": "2000000", "initial_rate": "0.015", "maintenance_rate": "0.0075"},
            {"tier": 3, "risk_limit": "5000000", "initial_rate": "0.02", "maintenance_rate": "0.01"}
        ]"#;

        let tiers: Vec<DeliveryRiskLimitTier> = serde_json::from_str(json).unwrap();
        assert_eq!(tiers.len(), 3);
        
        // Verify tiers are in order
        for (i, tier) in tiers.iter().enumerate() {
            assert_eq!(tier.tier, (i + 1) as i32);
        }
    }
}
