use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::coinm::{
    RestResult,
    enums::{ContractType, Period},
    public::rest::RestClient,
};

/// Parameters for Basis
#[derive(Debug, Clone, Serialize)]
pub struct BasisRequest {
    /// Pair name
    pub pair: String,

    /// Contract type
    #[serde(rename = "contractType")]
    pub contract_type: ContractType,

    /// The time interval
    pub period: Period,

    /// Maximum 500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Start time
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,

    /// End time
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
}

/// Basis data
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Basis {
    /// Pair name
    pub pair: String,
    /// Contract type
    pub contract_type: ContractType,
    /// Futures price
    pub futures_price: Decimal,
    /// Index price
    pub index_price: Decimal,
    /// Basis
    pub basis: Decimal,
    /// Basis rate
    pub basis_rate: Decimal,
    /// Timestamp
    pub timestamp: i64,
}

impl RestClient {
    /// Get basis
    ///
    /// Weight: 1
    pub async fn get_basis(&self, params: BasisRequest) -> RestResult<Vec<Basis>> {
        self.send_request("/futures/data/basis", reqwest::Method::GET, Some(params), 1)
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
