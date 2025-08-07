use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::gateio::delivery::RestResult;

const DELIVERY_CONTRACTS_ENDPOINT: &str = "/delivery/{}/contracts";
const DELIVERY_CONTRACT_ENDPOINT: &str = "/delivery/{}/contracts/{}";

/// Request parameters for delivery contracts
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeliveryContractsRequest {
    /// Settlement currency
    pub settle: String,
}

/// Request parameters for single delivery contract
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeliveryContractRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
}

/// Delivery contract information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryContract {
    /// Futures contract
    pub name: String,

    /// Underlying
    pub underlying: String,

    /// Cycle: 'WEEK', 'MONTH', 'QUARTER'
    pub cycle: String,

    /// Contract type (inverse, linear)
    #[serde(rename = "type")]
    pub contract_type: String,

    /// Quantitative scale
    pub quanto_multiplier: String,

    /// Leverage amount
    pub leverage_min: String,

    /// Leverage amount
    pub leverage_max: String,

    /// Maintenance rate
    pub maintenance_rate: String,

    /// Mark price
    pub mark_price: String,

    /// Index price
    pub index_price: String,

    /// Last trading time
    pub last_price: String,

    /// Maker fee rate
    pub maker_fee_rate: String,

    /// Taker fee rate  
    pub taker_fee_rate: String,

    /// Value of each contract
    pub order_price_round: String,

    /// Minimum order price increment
    pub mark_price_round: String,

    /// Risk limit base
    pub risk_limit_base: String,

    /// Risk limit step
    pub risk_limit_step: String,

    /// Maximum risk limit  
    pub risk_limit_max: String,

    /// Minimum order size
    pub order_size_min: i64,

    /// Maximum order size
    pub order_size_max: i64,

    /// Order price deviation from current mark price
    pub order_price_deviate: String,

    /// Reference discount rate for buying
    pub ref_discount_rate: String,

    /// Reference rebate rate for selling
    pub ref_rebate_rate: String,

    /// Current orderbook ID
    pub orderbook_id: i64,

    /// Trade ID
    pub trade_id: i64,

    /// Trade size
    pub trade_size: i64,

    /// Position size
    pub position_size: i64,

    /// Configuration change time
    pub config_change_time: f64,

    /// Whether the contract is delisted
    pub in_delisting: bool,

    /// Total traded volume in quote currency
    pub orders_limit: i32,

    /// Whether inverse contract
    pub enable_bonus: Option<bool>,

    /// Enable credit trading
    pub enable_credit: Option<bool>,

    /// Create time
    pub create_time: Option<f64>,

    /// Expiration time
    pub expire_time: Option<i64>,

    /// Settlement price
    pub settle_price: Option<String>,

    /// Settlement size
    pub settle_size: Option<i64>,
}

impl RestClient {
    /// List all delivery contracts
    ///
    /// Retrieves all available delivery contracts for the specified settlement currency.
    ///
    /// [docs]: https://www.gate.io/docs/developers/apiv4/en/#list-all-futures-contracts-2
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `params` - The delivery contracts request parameters
    ///
    /// # Returns
    /// List of delivery contracts for the specified settlement currency
    pub async fn get_delivery_contracts(
        &self,
        params: DeliveryContractsRequest,
    ) -> RestResult<Vec<DeliveryContract>> {
        let endpoint = DELIVERY_CONTRACTS_ENDPOINT.replace("{}", &params.settle);
        self.get_with_query(&endpoint, Some(&params)).await
    }

    /// Get a single delivery contract
    ///
    /// Retrieves detailed information about a specific delivery contract.
    ///
    /// [docs]: https://www.gate.io/docs/developers/apiv4/en/#get-a-single-contract-2
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `params` - The delivery contract request parameters
    ///
    /// # Returns
    /// Detailed information about the specific delivery contract
    pub async fn get_delivery_contract(
        &self,
        params: DeliveryContractRequest,
    ) -> RestResult<DeliveryContract> {
        let endpoint = DELIVERY_CONTRACT_ENDPOINT
            .replace("{}", &params.settle)
            .replace("{}", &params.contract);
        self.get_with_query(&endpoint, Some(&params)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delivery_contracts_request() {
        let request = DeliveryContractsRequest {
            settle: "USDT".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
    }

    #[test]
    fn test_different_settlement_currencies() {
        let currencies = vec!["USDT", "BTC"];

        for settle in currencies {
            let request = DeliveryContractsRequest {
                settle: settle.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["settle"], settle);
        }
    }

    #[test]
    fn test_delivery_contract_request() {
        let request = DeliveryContractRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT_20241227".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT_20241227");
    }

    #[test]
    fn test_delivery_contract_deserialization() {
        let json = r#"{
            "name": "BTC_USDT_20241227",
            "underlying": "BTC_USDT",
            "cycle": "WEEK",
            "type": "linear",
            "quanto_multiplier": "0.0001",
            "leverage_min": "1",
            "leverage_max": "100",
            "maintenance_rate": "0.005",
            "mark_price": "43250.5",
            "index_price": "43245.0",
            "last_price": "43252.0",
            "maker_fee_rate": "0.00015",
            "taker_fee_rate": "0.0005",
            "order_price_round": "0.5",
            "mark_price_round": "0.01",
            "risk_limit_base": "1000000",
            "risk_limit_step": "1000000",
            "risk_limit_max": "50000000",
            "order_size_min": 1,
            "order_size_max": 1000000,
            "order_price_deviate": "0.2",
            "ref_discount_rate": "0",
            "ref_rebate_rate": "0.2",
            "orderbook_id": 1234567890,
            "trade_id": 9876543210,
            "trade_size": 5000000,
            "position_size": 1000000,
            "config_change_time": 1640995200.0,
            "in_delisting": false,
            "orders_limit": 100,
            "enable_bonus": true,
            "enable_credit": false,
            "create_time": 1640000000.0,
            "expire_time": 1703721600,
            "settle_price": "43300.0",
            "settle_size": 500000
        }"#;

        let contract: DeliveryContract = serde_json::from_str(json).unwrap();
        assert_eq!(contract.name, "BTC_USDT_20241227");
        assert_eq!(contract.underlying, "BTC_USDT");
        assert_eq!(contract.cycle, "WEEK");
        assert_eq!(contract.contract_type, "linear");
        assert_eq!(contract.quanto_multiplier, "0.0001");
        assert_eq!(contract.leverage_min, "1");
        assert_eq!(contract.leverage_max, "100");
        assert_eq!(contract.maintenance_rate, "0.005");
        assert_eq!(contract.mark_price, "43250.5");
        assert_eq!(contract.index_price, "43245.0");
        assert_eq!(contract.last_price, "43252.0");
        assert_eq!(contract.maker_fee_rate, "0.00015");
        assert_eq!(contract.taker_fee_rate, "0.0005");
        assert_eq!(contract.order_price_round, "0.5");
        assert_eq!(contract.mark_price_round, "0.01");
        assert_eq!(contract.risk_limit_base, "1000000");
        assert_eq!(contract.risk_limit_step, "1000000");
        assert_eq!(contract.risk_limit_max, "50000000");
        assert_eq!(contract.order_size_min, 1);
        assert_eq!(contract.order_size_max, 1000000);
        assert_eq!(contract.order_price_deviate, "0.2");
        assert_eq!(contract.ref_discount_rate, "0");
        assert_eq!(contract.ref_rebate_rate, "0.2");
        assert_eq!(contract.orderbook_id, 1234567890);
        assert_eq!(contract.trade_id, 9876543210);
        assert_eq!(contract.trade_size, 5000000);
        assert_eq!(contract.position_size, 1000000);
        assert_eq!(contract.config_change_time, 1640995200.0);
        assert_eq!(contract.in_delisting, false);
        assert_eq!(contract.orders_limit, 100);
        assert_eq!(contract.enable_bonus, Some(true));
        assert_eq!(contract.enable_credit, Some(false));
        assert_eq!(contract.create_time, Some(1640000000.0));
        assert_eq!(contract.expire_time, Some(1703721600));
        assert_eq!(contract.settle_price, Some("43300.0".to_string()));
        assert_eq!(contract.settle_size, Some(500000));
    }

    #[test]
    fn test_weekly_contracts() {
        let weekly_contracts = vec![
            ("BTC_USDT_20241227", "WEEK", "Friday expiry"),
            ("ETH_USDT_20241227", "WEEK", "Friday expiry"),
            ("BTC_USDT_20250103", "WEEK", "Next Friday"),
        ];

        for (name, cycle, _description) in weekly_contracts {
            let json = format!(
                r#"{{
                "name": "{}",
                "underlying": "BTC_USDT",
                "cycle": "{}",
                "type": "linear",
                "quanto_multiplier": "0.0001",
                "leverage_min": "1",
                "leverage_max": "100",
                "maintenance_rate": "0.005",
                "mark_price": "43250.5",
                "index_price": "43245.0",
                "last_price": "43252.0",
                "maker_fee_rate": "0.00015",
                "taker_fee_rate": "0.0005",
                "order_price_round": "0.5",
                "mark_price_round": "0.01",
                "risk_limit_base": "1000000",
                "risk_limit_step": "1000000",
                "risk_limit_max": "50000000",
                "order_size_min": 1,
                "order_size_max": 1000000,
                "order_price_deviate": "0.2",
                "ref_discount_rate": "0",
                "ref_rebate_rate": "0.2",
                "orderbook_id": 1234567890,
                "trade_id": 9876543210,
                "trade_size": 5000000,
                "position_size": 1000000,
                "config_change_time": 1640995200.0,
                "in_delisting": false,
                "orders_limit": 100
            }}"#,
                name, cycle
            );

            let contract: DeliveryContract = serde_json::from_str(&json).unwrap();
            assert_eq!(contract.name, name);
            assert_eq!(contract.cycle, cycle);
        }
    }

    #[test]
    fn test_monthly_contracts() {
        let monthly_contracts = vec![
            ("BTC_USDT_20241231", "MONTH", "Month end"),
            ("ETH_USDT_20241231", "MONTH", "Month end"),
            ("BTC_USDT_20250131", "MONTH", "Next month end"),
        ];

        for (name, cycle, _description) in monthly_contracts {
            let json = format!(
                r#"{{
                "name": "{}",
                "underlying": "BTC_USDT",
                "cycle": "{}",
                "type": "linear",
                "quanto_multiplier": "0.0001",
                "leverage_min": "1",
                "leverage_max": "100",
                "maintenance_rate": "0.005",
                "mark_price": "43250.5",
                "index_price": "43245.0",
                "last_price": "43252.0",
                "maker_fee_rate": "0.00015",
                "taker_fee_rate": "0.0005",
                "order_price_round": "0.5",
                "mark_price_round": "0.01",
                "risk_limit_base": "1000000",
                "risk_limit_step": "1000000",
                "risk_limit_max": "50000000",
                "order_size_min": 1,
                "order_size_max": 1000000,
                "order_price_deviate": "0.2",
                "ref_discount_rate": "0",
                "ref_rebate_rate": "0.2",
                "orderbook_id": 1234567890,
                "trade_id": 9876543210,
                "trade_size": 5000000,
                "position_size": 1000000,
                "config_change_time": 1640995200.0,
                "in_delisting": false,
                "orders_limit": 100
            }}"#,
                name, cycle
            );

            let contract: DeliveryContract = serde_json::from_str(&json).unwrap();
            assert_eq!(contract.name, name);
            assert_eq!(contract.cycle, cycle);
        }
    }

    #[test]
    fn test_quarterly_contracts() {
        let quarterly_contracts = vec![
            ("BTC_USDT_20241227", "QUARTER", "Q4 2024"),
            ("ETH_USDT_20241227", "QUARTER", "Q4 2024"),
            ("BTC_USDT_20250328", "QUARTER", "Q1 2025"),
        ];

        for (name, cycle, _description) in quarterly_contracts {
            let json = format!(
                r#"{{
                "name": "{}",
                "underlying": "BTC_USDT",
                "cycle": "{}",
                "type": "linear",
                "quanto_multiplier": "0.0001",
                "leverage_min": "1",
                "leverage_max": "100",
                "maintenance_rate": "0.005",
                "mark_price": "43250.5",
                "index_price": "43245.0",
                "last_price": "43252.0",
                "maker_fee_rate": "0.00015",
                "taker_fee_rate": "0.0005",
                "order_price_round": "0.5",
                "mark_price_round": "0.01",
                "risk_limit_base": "1000000",
                "risk_limit_step": "1000000",
                "risk_limit_max": "50000000",
                "order_size_min": 1,
                "order_size_max": 1000000,
                "order_price_deviate": "0.2",
                "ref_discount_rate": "0",
                "ref_rebate_rate": "0.2",
                "orderbook_id": 1234567890,
                "trade_id": 9876543210,
                "trade_size": 5000000,
                "position_size": 1000000,
                "config_change_time": 1640995200.0,
                "in_delisting": false,
                "orders_limit": 100
            }}"#,
                name, cycle
            );

            let contract: DeliveryContract = serde_json::from_str(&json).unwrap();
            assert_eq!(contract.name, name);
            assert_eq!(contract.cycle, cycle);
        }
    }

    #[test]
    fn test_contract_types() {
        let contract_types = vec![
            ("linear", "Linear contract"),
            ("inverse", "Inverse contract"),
        ];

        for (contract_type, _description) in contract_types {
            let json = format!(
                r#"{{
                "name": "BTC_USDT_20241227",
                "underlying": "BTC_USDT",
                "cycle": "WEEK",
                "type": "{}",
                "quanto_multiplier": "0.0001",
                "leverage_min": "1",
                "leverage_max": "100",
                "maintenance_rate": "0.005",
                "mark_price": "43250.5",
                "index_price": "43245.0",
                "last_price": "43252.0",
                "maker_fee_rate": "0.00015",
                "taker_fee_rate": "0.0005",
                "order_price_round": "0.5",
                "mark_price_round": "0.01",
                "risk_limit_base": "1000000",
                "risk_limit_step": "1000000",
                "risk_limit_max": "50000000",
                "order_size_min": 1,
                "order_size_max": 1000000,
                "order_price_deviate": "0.2",
                "ref_discount_rate": "0",
                "ref_rebate_rate": "0.2",
                "orderbook_id": 1234567890,
                "trade_id": 9876543210,
                "trade_size": 5000000,
                "position_size": 1000000,
                "config_change_time": 1640995200.0,
                "in_delisting": false,
                "orders_limit": 100
            }}"#,
                contract_type
            );

            let contract: DeliveryContract = serde_json::from_str(&json).unwrap();
            assert_eq!(contract.contract_type, contract_type);
        }
    }

    #[test]
    fn test_leverage_ranges() {
        let leverage_scenarios = vec![
            ("1", "100", "Standard range"),
            ("1", "50", "Conservative range"),
            ("1", "125", "Extended range"),
            ("2", "100", "Higher minimum"),
        ];

        for (min, max, _description) in leverage_scenarios {
            let json = format!(
                r#"{{
                "name": "BTC_USDT_20241227",
                "underlying": "BTC_USDT",
                "cycle": "WEEK",
                "type": "linear",
                "quanto_multiplier": "0.0001",
                "leverage_min": "{}",
                "leverage_max": "{}",
                "maintenance_rate": "0.005",
                "mark_price": "43250.5",
                "index_price": "43245.0",
                "last_price": "43252.0",
                "maker_fee_rate": "0.00015",
                "taker_fee_rate": "0.0005",
                "order_price_round": "0.5",
                "mark_price_round": "0.01",
                "risk_limit_base": "1000000",
                "risk_limit_step": "1000000",
                "risk_limit_max": "50000000",
                "order_size_min": 1,
                "order_size_max": 1000000,
                "order_price_deviate": "0.2",
                "ref_discount_rate": "0",
                "ref_rebate_rate": "0.2",
                "orderbook_id": 1234567890,
                "trade_id": 9876543210,
                "trade_size": 5000000,
                "position_size": 1000000,
                "config_change_time": 1640995200.0,
                "in_delisting": false,
                "orders_limit": 100
            }}"#,
                min, max
            );

            let contract: DeliveryContract = serde_json::from_str(&json).unwrap();
            assert_eq!(contract.leverage_min, min);
            assert_eq!(contract.leverage_max, max);

            let min_val: f64 = min.parse().unwrap();
            let max_val: f64 = max.parse().unwrap();
            assert!(min_val >= 1.0);
            assert!(max_val > min_val);
        }
    }

    #[test]
    fn test_fee_rates() {
        let fee_scenarios = vec![
            ("0.00015", "0.0005", "Standard fees"),
            ("0.0001", "0.0004", "Discount fees"),
            ("0.0002", "0.0006", "Higher fees"),
            ("0", "0.0003", "Zero maker fee"),
        ];

        for (maker, taker, _description) in fee_scenarios {
            let json = format!(
                r#"{{
                "name": "BTC_USDT_20241227",
                "underlying": "BTC_USDT",
                "cycle": "WEEK",
                "type": "linear",
                "quanto_multiplier": "0.0001",
                "leverage_min": "1",
                "leverage_max": "100",
                "maintenance_rate": "0.005",
                "mark_price": "43250.5",
                "index_price": "43245.0",
                "last_price": "43252.0",
                "maker_fee_rate": "{}",
                "taker_fee_rate": "{}",
                "order_price_round": "0.5",
                "mark_price_round": "0.01",
                "risk_limit_base": "1000000",
                "risk_limit_step": "1000000",
                "risk_limit_max": "50000000",
                "order_size_min": 1,
                "order_size_max": 1000000,
                "order_price_deviate": "0.2",
                "ref_discount_rate": "0",
                "ref_rebate_rate": "0.2",
                "orderbook_id": 1234567890,
                "trade_id": 9876543210,
                "trade_size": 5000000,
                "position_size": 1000000,
                "config_change_time": 1640995200.0,
                "in_delisting": false,
                "orders_limit": 100
            }}"#,
                maker, taker
            );

            let contract: DeliveryContract = serde_json::from_str(&json).unwrap();
            assert_eq!(contract.maker_fee_rate, maker);
            assert_eq!(contract.taker_fee_rate, taker);

            let maker_val: f64 = maker.parse().unwrap();
            let taker_val: f64 = taker.parse().unwrap();
            assert!(maker_val <= taker_val); // Maker fee should be <= taker fee
        }
    }

    #[test]
    fn test_order_size_limits() {
        let size_scenarios = vec![
            (1, 1000000, "Standard limits"),
            (10, 500000, "Higher minimum"),
            (1, 2000000, "Higher maximum"),
            (100, 100000, "Restricted range"),
        ];

        for (min, max, _description) in size_scenarios {
            let json = format!(
                r#"{{
                "name": "BTC_USDT_20241227",
                "underlying": "BTC_USDT",
                "cycle": "WEEK",
                "type": "linear",
                "quanto_multiplier": "0.0001",
                "leverage_min": "1",
                "leverage_max": "100",
                "maintenance_rate": "0.005",
                "mark_price": "43250.5",
                "index_price": "43245.0",
                "last_price": "43252.0",
                "maker_fee_rate": "0.00015",
                "taker_fee_rate": "0.0005",
                "order_price_round": "0.5",
                "mark_price_round": "0.01",
                "risk_limit_base": "1000000",
                "risk_limit_step": "1000000",
                "risk_limit_max": "50000000",
                "order_size_min": {},
                "order_size_max": {},
                "order_price_deviate": "0.2",
                "ref_discount_rate": "0",
                "ref_rebate_rate": "0.2",
                "orderbook_id": 1234567890,
                "trade_id": 9876543210,
                "trade_size": 5000000,
                "position_size": 1000000,
                "config_change_time": 1640995200.0,
                "in_delisting": false,
                "orders_limit": 100
            }}"#,
                min, max
            );

            let contract: DeliveryContract = serde_json::from_str(&json).unwrap();
            assert_eq!(contract.order_size_min, min);
            assert_eq!(contract.order_size_max, max);
            assert!(min > 0);
            assert!(max > min);
        }
    }

    #[test]
    fn test_risk_limit_tiers() {
        let risk_scenarios = vec![
            ("1000000", "1000000", "50000000", "Standard tiers"),
            ("500000", "500000", "25000000", "Smaller tiers"),
            ("2000000", "2000000", "100000000", "Larger tiers"),
        ];

        for (base, step, max, _description) in risk_scenarios {
            let json = format!(
                r#"{{
                "name": "BTC_USDT_20241227",
                "underlying": "BTC_USDT",
                "cycle": "WEEK",
                "type": "linear",
                "quanto_multiplier": "0.0001",
                "leverage_min": "1",
                "leverage_max": "100",
                "maintenance_rate": "0.005",
                "mark_price": "43250.5",
                "index_price": "43245.0",
                "last_price": "43252.0",
                "maker_fee_rate": "0.00015",
                "taker_fee_rate": "0.0005",
                "order_price_round": "0.5",
                "mark_price_round": "0.01",
                "risk_limit_base": "{}",
                "risk_limit_step": "{}",
                "risk_limit_max": "{}",
                "order_size_min": 1,
                "order_size_max": 1000000,
                "order_price_deviate": "0.2",
                "ref_discount_rate": "0",
                "ref_rebate_rate": "0.2",
                "orderbook_id": 1234567890,
                "trade_id": 9876543210,
                "trade_size": 5000000,
                "position_size": 1000000,
                "config_change_time": 1640995200.0,
                "in_delisting": false,
                "orders_limit": 100
            }}"#,
                base, step, max
            );

            let contract: DeliveryContract = serde_json::from_str(&json).unwrap();
            assert_eq!(contract.risk_limit_base, base);
            assert_eq!(contract.risk_limit_step, step);
            assert_eq!(contract.risk_limit_max, max);
        }
    }

    #[test]
    fn test_delisting_status() {
        let delisting_scenarios = vec![(false, "Active contract"), (true, "Delisting contract")];

        for (in_delisting, _description) in delisting_scenarios {
            let json = format!(
                r#"{{
                "name": "BTC_USDT_20241227",
                "underlying": "BTC_USDT",
                "cycle": "WEEK",
                "type": "linear",
                "quanto_multiplier": "0.0001",
                "leverage_min": "1",
                "leverage_max": "100",
                "maintenance_rate": "0.005",
                "mark_price": "43250.5",
                "index_price": "43245.0",
                "last_price": "43252.0",
                "maker_fee_rate": "0.00015",
                "taker_fee_rate": "0.0005",
                "order_price_round": "0.5",
                "mark_price_round": "0.01",
                "risk_limit_base": "1000000",
                "risk_limit_step": "1000000",
                "risk_limit_max": "50000000",
                "order_size_min": 1,
                "order_size_max": 1000000,
                "order_price_deviate": "0.2",
                "ref_discount_rate": "0",
                "ref_rebate_rate": "0.2",
                "orderbook_id": 1234567890,
                "trade_id": 9876543210,
                "trade_size": 5000000,
                "position_size": 1000000,
                "config_change_time": 1640995200.0,
                "in_delisting": {},
                "orders_limit": 100
            }}"#,
                in_delisting
            );

            let contract: DeliveryContract = serde_json::from_str(&json).unwrap();
            assert_eq!(contract.in_delisting, in_delisting);
        }
    }

    #[test]
    fn test_settled_contract() {
        let json = r#"{
            "name": "BTC_USDT_20241220",
            "underlying": "BTC_USDT",
            "cycle": "WEEK",
            "type": "linear",
            "quanto_multiplier": "0.0001",
            "leverage_min": "1",
            "leverage_max": "100",
            "maintenance_rate": "0.005",
            "mark_price": "43250.5",
            "index_price": "43245.0",
            "last_price": "43252.0",
            "maker_fee_rate": "0.00015",
            "taker_fee_rate": "0.0005",
            "order_price_round": "0.5",
            "mark_price_round": "0.01",
            "risk_limit_base": "1000000",
            "risk_limit_step": "1000000",
            "risk_limit_max": "50000000",
            "order_size_min": 1,
            "order_size_max": 1000000,
            "order_price_deviate": "0.2",
            "ref_discount_rate": "0",
            "ref_rebate_rate": "0.2",
            "orderbook_id": 1234567890,
            "trade_id": 9876543210,
            "trade_size": 5000000,
            "position_size": 0,
            "config_change_time": 1640995200.0,
            "in_delisting": true,
            "orders_limit": 0,
            "enable_bonus": false,
            "enable_credit": false,
            "create_time": 1639000000.0,
            "expire_time": 1703116800,
            "settle_price": "42800.0",
            "settle_size": 0
        }"#;

        let contract: DeliveryContract = serde_json::from_str(json).unwrap();
        assert_eq!(contract.in_delisting, true);
        assert_eq!(contract.position_size, 0);
        assert_eq!(contract.orders_limit, 0);
        assert_eq!(contract.settle_price, Some("42800.0".to_string()));
        assert_eq!(contract.settle_size, Some(0));
    }

    #[test]
    fn test_altcoin_contracts() {
        let altcoin_contracts = vec![
            ("ETH_USDT_20241227", "ETH_USDT", "Ethereum"),
            ("ADA_USDT_20241227", "ADA_USDT", "Cardano"),
            ("SOL_USDT_20241227", "SOL_USDT", "Solana"),
            ("MATIC_USDT_20241227", "MATIC_USDT", "Polygon"),
        ];

        for (name, underlying, _description) in altcoin_contracts {
            let json = format!(
                r#"{{
                "name": "{}",
                "underlying": "{}",
                "cycle": "WEEK",
                "type": "linear",
                "quanto_multiplier": "0.0001",
                "leverage_min": "1",
                "leverage_max": "50",
                "maintenance_rate": "0.01",
                "mark_price": "100.0",
                "index_price": "100.0",
                "last_price": "100.0",
                "maker_fee_rate": "0.00015",
                "taker_fee_rate": "0.0005",
                "order_price_round": "0.01",
                "mark_price_round": "0.01",
                "risk_limit_base": "500000",
                "risk_limit_step": "500000",
                "risk_limit_max": "10000000",
                "order_size_min": 1,
                "order_size_max": 500000,
                "order_price_deviate": "0.2",
                "ref_discount_rate": "0",
                "ref_rebate_rate": "0.2",
                "orderbook_id": 1234567890,
                "trade_id": 9876543210,
                "trade_size": 1000000,
                "position_size": 100000,
                "config_change_time": 1640995200.0,
                "in_delisting": false,
                "orders_limit": 100
            }}"#,
                name, underlying
            );

            let contract: DeliveryContract = serde_json::from_str(&json).unwrap();
            assert_eq!(contract.name, name);
            assert_eq!(contract.underlying, underlying);

            // Altcoins often have lower leverage limits
            let max_leverage: f64 = contract.leverage_max.parse().unwrap();
            assert!(max_leverage <= 100.0);
        }
    }

    #[test]
    fn test_minimal_contract_info() {
        let json = r#"{
            "name": "BTC_USDT_20241227",
            "underlying": "BTC_USDT",
            "cycle": "WEEK",
            "type": "linear",
            "quanto_multiplier": "0.0001",
            "leverage_min": "1",
            "leverage_max": "100",
            "maintenance_rate": "0.005",
            "mark_price": "43250.5",
            "index_price": "43245.0",
            "last_price": "43252.0",
            "maker_fee_rate": "0.00015",
            "taker_fee_rate": "0.0005",
            "order_price_round": "0.5",
            "mark_price_round": "0.01",
            "risk_limit_base": "1000000",
            "risk_limit_step": "1000000",
            "risk_limit_max": "50000000",
            "order_size_min": 1,
            "order_size_max": 1000000,
            "order_price_deviate": "0.2",
            "ref_discount_rate": "0",
            "ref_rebate_rate": "0.2",
            "orderbook_id": 1234567890,
            "trade_id": 9876543210,
            "trade_size": 5000000,
            "position_size": 1000000,
            "config_change_time": 1640995200.0,
            "in_delisting": false,
            "orders_limit": 100
        }"#;

        let contract: DeliveryContract = serde_json::from_str(json).unwrap();
        assert!(contract.enable_bonus.is_none());
        assert!(contract.enable_credit.is_none());
        assert!(contract.create_time.is_none());
        assert!(contract.expire_time.is_none());
        assert!(contract.settle_price.is_none());
        assert!(contract.settle_size.is_none());
    }

    #[test]
    fn test_price_precision() {
        let precision_scenarios = vec![
            ("0.5", "0.01", "BTC standard"),
            ("0.01", "0.0001", "ETH precision"),
            ("0.0001", "0.000001", "Small altcoin"),
            ("1", "0.1", "Low precision"),
        ];

        for (order_round, mark_round, _description) in precision_scenarios {
            let json = format!(
                r#"{{
                "name": "BTC_USDT_20241227",
                "underlying": "BTC_USDT",
                "cycle": "WEEK",
                "type": "linear",
                "quanto_multiplier": "0.0001",
                "leverage_min": "1",
                "leverage_max": "100",
                "maintenance_rate": "0.005",
                "mark_price": "43250.5",
                "index_price": "43245.0",
                "last_price": "43252.0",
                "maker_fee_rate": "0.00015",
                "taker_fee_rate": "0.0005",
                "order_price_round": "{}",
                "mark_price_round": "{}",
                "risk_limit_base": "1000000",
                "risk_limit_step": "1000000",
                "risk_limit_max": "50000000",
                "order_size_min": 1,
                "order_size_max": 1000000,
                "order_price_deviate": "0.2",
                "ref_discount_rate": "0",
                "ref_rebate_rate": "0.2",
                "orderbook_id": 1234567890,
                "trade_id": 9876543210,
                "trade_size": 5000000,
                "position_size": 1000000,
                "config_change_time": 1640995200.0,
                "in_delisting": false,
                "orders_limit": 100
            }}"#,
                order_round, mark_round
            );

            let contract: DeliveryContract = serde_json::from_str(&json).unwrap();
            assert_eq!(contract.order_price_round, order_round);
            assert_eq!(contract.mark_price_round, mark_round);
        }
    }

    #[test]
    fn test_order_price_deviate() {
        let deviate_scenarios = vec![
            ("0.2", "20% deviation"),
            ("0.1", "10% deviation"),
            ("0.05", "5% deviation"),
            ("0.5", "50% deviation"),
        ];

        for (deviate, _description) in deviate_scenarios {
            let json = format!(
                r#"{{
                "name": "BTC_USDT_20241227",
                "underlying": "BTC_USDT",
                "cycle": "WEEK",
                "type": "linear",
                "quanto_multiplier": "0.0001",
                "leverage_min": "1",
                "leverage_max": "100",
                "maintenance_rate": "0.005",
                "mark_price": "43250.5",
                "index_price": "43245.0",
                "last_price": "43252.0",
                "maker_fee_rate": "0.00015",
                "taker_fee_rate": "0.0005",
                "order_price_round": "0.5",
                "mark_price_round": "0.01",
                "risk_limit_base": "1000000",
                "risk_limit_step": "1000000",
                "risk_limit_max": "50000000",
                "order_size_min": 1,
                "order_size_max": 1000000,
                "order_price_deviate": "{}",
                "ref_discount_rate": "0",
                "ref_rebate_rate": "0.2",
                "orderbook_id": 1234567890,
                "trade_id": 9876543210,
                "trade_size": 5000000,
                "position_size": 1000000,
                "config_change_time": 1640995200.0,
                "in_delisting": false,
                "orders_limit": 100
            }}"#,
                deviate
            );

            let contract: DeliveryContract = serde_json::from_str(&json).unwrap();
            assert_eq!(contract.order_price_deviate, deviate);

            let deviate_val: f64 = deviate.parse().unwrap();
            assert!(deviate_val > 0.0 && deviate_val <= 1.0);
        }
    }

    #[test]
    fn test_clone_behavior() {
        let request = DeliveryContractsRequest {
            settle: "USDT".to_string(),
        };

        let cloned = request.clone();
        assert_eq!(cloned.settle, request.settle);
    }

    #[test]
    fn test_debug_output() {
        let contract = DeliveryContract {
            name: "BTC_USDT_20241227".to_string(),
            underlying: "BTC_USDT".to_string(),
            cycle: "WEEK".to_string(),
            contract_type: "linear".to_string(),
            quanto_multiplier: "0.0001".to_string(),
            leverage_min: "1".to_string(),
            leverage_max: "100".to_string(),
            maintenance_rate: "0.005".to_string(),
            mark_price: "43250.5".to_string(),
            index_price: "43245.0".to_string(),
            last_price: "43252.0".to_string(),
            maker_fee_rate: "0.00015".to_string(),
            taker_fee_rate: "0.0005".to_string(),
            order_price_round: "0.5".to_string(),
            mark_price_round: "0.01".to_string(),
            risk_limit_base: "1000000".to_string(),
            risk_limit_step: "1000000".to_string(),
            risk_limit_max: "50000000".to_string(),
            order_size_min: 1,
            order_size_max: 1000000,
            order_price_deviate: "0.2".to_string(),
            ref_discount_rate: "0".to_string(),
            ref_rebate_rate: "0.2".to_string(),
            orderbook_id: 1234567890,
            trade_id: 9876543210,
            trade_size: 5000000,
            position_size: 1000000,
            config_change_time: 1640995200.0,
            in_delisting: false,
            orders_limit: 100,
            enable_bonus: Some(true),
            enable_credit: Some(false),
            create_time: Some(1640000000.0),
            expire_time: Some(1703721600),
            settle_price: Some("43300.0".to_string()),
            settle_size: Some(500000),
        };

        let debug_str = format!("{:?}", contract);
        assert!(debug_str.contains("DeliveryContract"));
        assert!(debug_str.contains("BTC_USDT_20241227"));
        assert!(debug_str.contains("43250.5"));
    }

    #[test]
    fn test_serialization_round_trip() {
        let contract = DeliveryContract {
            name: "BTC_USDT_20241227".to_string(),
            underlying: "BTC_USDT".to_string(),
            cycle: "WEEK".to_string(),
            contract_type: "linear".to_string(),
            quanto_multiplier: "0.0001".to_string(),
            leverage_min: "1".to_string(),
            leverage_max: "100".to_string(),
            maintenance_rate: "0.005".to_string(),
            mark_price: "43250.5".to_string(),
            index_price: "43245.0".to_string(),
            last_price: "43252.0".to_string(),
            maker_fee_rate: "0.00015".to_string(),
            taker_fee_rate: "0.0005".to_string(),
            order_price_round: "0.5".to_string(),
            mark_price_round: "0.01".to_string(),
            risk_limit_base: "1000000".to_string(),
            risk_limit_step: "1000000".to_string(),
            risk_limit_max: "50000000".to_string(),
            order_size_min: 1,
            order_size_max: 1000000,
            order_price_deviate: "0.2".to_string(),
            ref_discount_rate: "0".to_string(),
            ref_rebate_rate: "0.2".to_string(),
            orderbook_id: 1234567890,
            trade_id: 9876543210,
            trade_size: 5000000,
            position_size: 1000000,
            config_change_time: 1640995200.0,
            in_delisting: false,
            orders_limit: 100,
            enable_bonus: Some(true),
            enable_credit: Some(false),
            create_time: Some(1640000000.0),
            expire_time: Some(1703721600),
            settle_price: Some("43300.0".to_string()),
            settle_size: Some(500000),
        };

        let json = serde_json::to_string(&contract).unwrap();
        let deserialized: DeliveryContract = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.name, contract.name);
        assert_eq!(deserialized.underlying, contract.underlying);
        assert_eq!(deserialized.cycle, contract.cycle);
        assert_eq!(deserialized.contract_type, contract.contract_type);
        assert_eq!(deserialized.leverage_max, contract.leverage_max);
        assert_eq!(deserialized.expire_time, contract.expire_time);
    }
}
