use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::coinm::{
    RestResult,
    enums::{ContractType, Period},
    public::rest::RestClient,
};

const BASIS_ENDPOINT: &str = "/futures/data/basis";

/// Request parameters for the basis endpoint.
///
/// Used to query futures basis data, which shows the difference between
/// futures and index prices over time for specific contract types.
#[derive(Debug, Clone, Serialize)]
pub struct BasisRequest {
    /// Trading pair name (e.g., "BTCUSD", "ETHUSD").
    pub pair: String,

    /// Contract type for the futures position.
    #[serde(rename = "contractType")]
    pub contract_type: ContractType,

    /// Time interval for basis data aggregation.
    pub period: Period,

    /// Maximum number of data points to return. Default 30, maximum 500.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Start time for filtering results (milliseconds since epoch).
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,

    /// End time for filtering results (milliseconds since epoch).
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
}

/// Basis data point containing futures and index price information.
///
/// Represents a single data point from the basis endpoint, showing the
/// relationship between futures and index prices at a specific timestamp.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Basis {
    /// Trading pair name.
    pub pair: String,

    /// Contract type for this basis calculation.
    pub contract_type: ContractType,

    /// Current futures price for the contract.
    pub futures_price: Decimal,

    /// Current index price for the underlying asset.
    pub index_price: Decimal,

    /// Basis value (futures_price - index_price).
    pub basis: Decimal,

    /// Basis rate as a decimal (basis / index_price).
    pub basis_rate: Decimal,

    /// Timestamp when this basis data was recorded (milliseconds since epoch).
    pub timestamp: i64,
}

impl RestClient {
    /// Basis
    ///
    /// Queries basis data for futures contracts, showing the difference between
    /// futures and index prices over time.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Basis
    ///
    /// Rate limit: Weight 1
    ///
    /// # Arguments
    /// * `params` - The request parameters including pair, contract type, and time range
    ///
    /// # Returns
    /// A vector of basis data points with futures prices, index prices, and calculated basis values
    pub async fn get_basis(&self, params: BasisRequest) -> RestResult<Vec<Basis>> {
        self.send_request(BASIS_ENDPOINT, reqwest::Method::GET, Some(params), 1)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::prelude::FromPrimitive;

    #[test]
    fn test_basis_request_serialization() {
        let request = BasisRequest {
            pair: "BTCUSD".to_string(),
            contract_type: ContractType::Perpetual,
            period: Period::I5m,
            limit: None,
            start_time: None,
            end_time: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("pair=BTCUSD"));
        assert!(serialized.contains("contractType=PERPETUAL"));
        assert!(serialized.contains("period=5m"));
    }

    #[test]
    fn test_basis_request_serialization_with_all_params() {
        let request = BasisRequest {
            pair: "ETHUSD".to_string(),
            contract_type: ContractType::CurrentQuarter,
            period: Period::I1d,
            limit: Some(100),
            start_time: Some(1625097600000),
            end_time: Some(1625184000000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("pair=ETHUSD"));
        assert!(serialized.contains("contractType=CURRENT_QUARTER"));
        assert!(serialized.contains("period=1d"));
        assert!(serialized.contains("limit=100"));
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625184000000"));
    }

    #[test]
    fn test_basis_deserialization() {
        let json = r#"{
            "pair": "BTCUSD",
            "contractType": "PERPETUAL",
            "futuresPrice": 50000.50,
            "indexPrice": 49950.25,
            "basis": 50.25,
            "basisRate": 0.001006,
            "timestamp": 1625097600000
        }"#;

        let basis: Basis = serde_json::from_str(json).unwrap();
        assert_eq!(basis.pair, "BTCUSD");
        assert_eq!(basis.contract_type, ContractType::Perpetual);
        assert_eq!(basis.futures_price, Decimal::from_f64(50000.50).unwrap());
        assert_eq!(basis.index_price, Decimal::from_f64(49950.25).unwrap());
        assert_eq!(basis.basis, Decimal::from_f64(50.25).unwrap());
        assert_eq!(basis.basis_rate, Decimal::from_f64(0.001006).unwrap());
        assert_eq!(basis.timestamp, 1625097600000);
    }

    #[test]
    fn test_basis_list_deserialization() {
        let json = r#"[
            {
                "pair": "BTCUSD",
                "contractType": "PERPETUAL",
                "futuresPrice": 50000.50,
                "indexPrice": 49950.25,
                "basis": 50.25,
                "basisRate": 0.001006,
                "timestamp": 1625097600000
            },
            {
                "pair": "BTCUSD",
                "contractType": "PERPETUAL",
                "futuresPrice": 50100.00,
                "indexPrice": 50000.00,
                "basis": 100.00,
                "basisRate": 0.002000,
                "timestamp": 1625097900000
            }
        ]"#;

        let basis_list: Vec<Basis> = serde_json::from_str(json).unwrap();
        assert_eq!(basis_list.len(), 2);

        assert_eq!(
            basis_list[0].futures_price,
            Decimal::from_f64(50000.50).unwrap()
        );
        assert_eq!(basis_list[0].timestamp, 1625097600000);

        assert_eq!(
            basis_list[1].futures_price,
            Decimal::from_f64(50100.00).unwrap()
        );
        assert_eq!(
            basis_list[1].basis_rate,
            Decimal::from_f64(0.002000).unwrap()
        );
    }
}
