use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::{ContractType, Period, RestResult};

/// Endpoint path for futures basis data
const BASIS_ENDPOINT: &str = "/futures/data/basis";

/// Request parameters for the Basis endpoint.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BasisRequest<'a> {
    /// The pair to query (e.g., "BTCUSDT").
    pub pair: Cow<'a, str>,

    /// The contract type (PERPETUAL, CURRENT_QUARTER, NEXT_QUARTER).
    pub contract_type: ContractType,

    /// The period interval (e.g., "5m", "1h").
    pub period: Period,

    /// Number of data points to return (default 30, max 500).
    pub limit: u32,

    /// Start time in milliseconds since epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time in milliseconds since epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
}

/// Response for futures basis endpoint.
///
/// Contains basis data for a symbol over time.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasisResponse<'a> {
    /// Index price at the timestamp.
    pub index_price: Cow<'a, str>,

    /// Contract type (PERPETUAL, CURRENT_QUARTER, NEXT_QUARTER).
    pub contract_type: ContractType,

    /// Basis rate as a decimal string.
    pub basis_rate: Cow<'a, str>,

    /// Futures price at the timestamp.
    pub futures_price: Cow<'a, str>,

    /// Annualized basis rate as a decimal string.
    pub annualized_basis_rate: Cow<'a, str>,

    /// Basis value as a decimal string.
    pub basis: Cow<'a, str>,

    /// Trading pair symbol (e.g., "BTCUSDT").
    pub pair: Cow<'a, str>,

    /// Timestamp in milliseconds since epoch.
    pub timestamp: u64,
}

impl RestClient {
    /// Query future basis (GET /futures/data/basis)
    ///
    /// Returns futures basis data for a symbol and contract type over a period.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Basis
    ///
    /// Rate limit: 1 request per second
    ///
    /// # Arguments
    /// * `params` - BasisRequest parameters
    ///
    /// # Returns
    /// Vec<BasisResponse> - list of basis data points
    pub async fn basis<'a>(&self, params: BasisRequest<'a>) -> RestResult<Vec<BasisResponse<'a>>> {
        self.send_public_request(BASIS_ENDPOINT, reqwest::Method::GET, Some(params), 1)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basis_request_serialization() {
        let request = BasisRequest {
            pair: "BTCUSDT".into(),
            contract_type: ContractType::Perpetual,
            period: Period::I5m,
            limit: 100,
            start_time: Some(1625097600000),
            end_time: Some(1625184000000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("pair=BTCUSDT"));
        assert!(serialized.contains("contractType=PERPETUAL"));
        assert!(serialized.contains("period=5m"));
        assert!(serialized.contains("limit=100"));
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625184000000"));
    }

    #[test]
    fn test_basis_request_serialization_minimal() {
        let request = BasisRequest {
            pair: "ETHUSDT".into(),
            contract_type: ContractType::CurrentQuarter,
            period: Period::I1h,
            limit: 30,
            start_time: None,
            end_time: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("pair=ETHUSDT"));
        assert!(serialized.contains("contractType=CURRENT_QUARTER"));
        assert!(serialized.contains("period=1h"));
        assert!(serialized.contains("limit=30"));
        assert!(!serialized.contains("startTime"));
        assert!(!serialized.contains("endTime"));
    }

    #[test]
    fn test_basis_response_deserialization() {
        let json = r#"[
            {
                "indexPrice": "45000.00000000",
                "contractType": "PERPETUAL",
                "basisRate": "0.00100000",
                "futuresPrice": "45045.00000000",
                "annualizedBasisRate": "0.36500000",
                "basis": "45.00000000",
                "pair": "BTCUSDT",
                "timestamp": 1625097600000
            },
            {
                "indexPrice": "45100.00000000",
                "contractType": "PERPETUAL",
                "basisRate": "-0.00050000",
                "futuresPrice": "45077.45000000",
                "annualizedBasisRate": "-0.18250000",
                "basis": "-22.55000000",
                "pair": "BTCUSDT",
                "timestamp": 1625098500000
            }
        ]"#;

        let basis_data: Vec<BasisResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(basis_data.len(), 2);

        let first = &basis_data[0];
        assert_eq!(first.index_price, "45000.00000000");
        assert_eq!(first.contract_type, ContractType::Perpetual);
        assert_eq!(first.basis_rate, "0.00100000");
        assert_eq!(first.futures_price, "45045.00000000");
        assert_eq!(first.annualized_basis_rate, "0.36500000");
        assert_eq!(first.basis, "45.00000000");
        assert_eq!(first.pair, "BTCUSDT");
        assert_eq!(first.timestamp, 1625097600000);

        let second = &basis_data[1];
        assert_eq!(second.basis_rate, "-0.00050000");
        assert_eq!(second.annualized_basis_rate, "-0.18250000");
        assert_eq!(second.basis, "-22.55000000");
    }

    #[test]
    fn test_basis_different_contract_types() {
        let json = r#"[
            {
                "indexPrice": "45000.00000000",
                "contractType": "CURRENT_QUARTER",
                "basisRate": "0.00200000",
                "futuresPrice": "45090.00000000",
                "annualizedBasisRate": "0.73000000",
                "basis": "90.00000000",
                "pair": "BTCUSDT",
                "timestamp": 1625097600000
            },
            {
                "indexPrice": "45000.00000000",
                "contractType": "NEXT_QUARTER",
                "basisRate": "0.00300000",
                "futuresPrice": "45135.00000000",
                "annualizedBasisRate": "1.09500000",
                "basis": "135.00000000",
                "pair": "BTCUSDT",
                "timestamp": 1625097600000
            }
        ]"#;

        let basis_data: Vec<BasisResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(basis_data.len(), 2);
        assert_eq!(basis_data[0].contract_type, ContractType::CurrentQuarter);
        assert_eq!(basis_data[1].contract_type, ContractType::NextQuarter);
    }

    #[test]
    fn test_basis_negative_values() {
        let json = r#"[
            {
                "indexPrice": "3000.00000000",
                "contractType": "PERPETUAL",
                "basisRate": "-0.00250000",
                "futuresPrice": "2992.50000000",
                "annualizedBasisRate": "-0.91250000",
                "basis": "-7.50000000",
                "pair": "ETHUSDT",
                "timestamp": 1625097600000
            }
        ]"#;

        let basis_data: Vec<BasisResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(basis_data.len(), 1);
        assert_eq!(basis_data[0].basis_rate, "-0.00250000");
        assert_eq!(basis_data[0].annualized_basis_rate, "-0.91250000");
        assert_eq!(basis_data[0].basis, "-7.50000000");
    }
}
