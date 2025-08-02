use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, enums::IncomeType, private::rest::client::RestClient};

const INCOME_ENDPOINT: &str = "/dapi/v1/income";

/// Request parameters for income history.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct IncomeHistoryRequest {
    /// Trading symbol, e.g. BTCUSD_PERP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Income type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub income_type: Option<IncomeType>,

    /// Timestamp in ms to get income starting from INCLUSIVE
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// Timestamp in ms to get income ending from INCLUSIVE
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Default 100; max 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// This parameter cannot be used in combination with other parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Response for income history.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomeHistoryEntry {
    /// Symbol
    pub symbol: String,

    /// Income type
    pub income_type: IncomeType,

    /// Income amount
    pub income: String,

    /// Asset
    pub asset: String,

    /// Income info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,

    /// Time
    pub time: u64,

    /// Transaction id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tran_id: Option<u64>,

    /// Trade id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
}

impl RestClient {
    /// Get income history on Binance Coin-M Futures.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/coin-margined-futures/account/rest-api/Get-Income-History
    ///
    /// GET /dapi/v1/income
    /// Weight: 20
    /// Requires API key and signature.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`IncomeHistoryRequest`])
    ///
    /// # Returns
    /// A list of [`IncomeHistoryEntry`] objects with income history details.
    pub async fn get_income_history(
        &self,
        params: IncomeHistoryRequest,
    ) -> RestResult<Vec<IncomeHistoryEntry>> {
        let weight = 20;
        self.send_get_signed_request(INCOME_ENDPOINT, params, weight, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_income_history_request_serialization_minimal() {
        let request = IncomeHistoryRequest {
            symbol: None,
            income_type: None,
            start_time: None,
            end_time: None,
            limit: None,
            recv_window: None,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_income_history_request_serialization_with_symbol() {
        let request = IncomeHistoryRequest {
            symbol: Some("BTCUSD_PERP".to_string()),
            income_type: None,
            start_time: None,
            end_time: None,
            limit: None,
            recv_window: None,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTCUSD_PERP");
    }

    #[test]
    fn test_income_history_request_serialization_full() {
        let request = IncomeHistoryRequest {
            symbol: Some("BTCUSD_PERP".to_string()),
            income_type: Some(IncomeType::RealizedPnl),
            start_time: Some(1625097600000),
            end_time: Some(1625184000000),
            limit: Some(100),
            recv_window: Some(5000),
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSD_PERP"));
        assert!(serialized.contains("incomeType=REALIZED_PNL"));
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625184000000"));
        assert!(serialized.contains("limit=100"));
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_income_history_response_deserialization() {
        let json = r#"[
            {
                "symbol": "BTCUSD_PERP",
                "incomeType": "REALIZED_PNL",
                "income": "-0.12345678",
                "asset": "BTC",
                "info": "COMMISSION",
                "time": 1625097600000,
                "tranId": 7957183248,
                "tradeId": "123456"
            },
            {
                "symbol": "ETHUSD_PERP",
                "incomeType": "FUNDING_FEE",
                "income": "0.00234567",
                "asset": "ETH",
                "time": 1625097600001
            }
        ]"#;
        let response: Vec<IncomeHistoryEntry> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 2);

        let first_entry = &response[0];
        assert_eq!(first_entry.symbol, "BTCUSD_PERP");
        assert_eq!(first_entry.income_type, IncomeType::RealizedPnl);
        assert_eq!(first_entry.income, "-0.12345678");
        assert_eq!(first_entry.asset, "BTC");
        assert_eq!(first_entry.info, Some("COMMISSION".to_string()));
        assert_eq!(first_entry.time, 1625097600000);
        assert_eq!(first_entry.tran_id, Some(7957183248));
        assert_eq!(first_entry.trade_id, Some("123456".to_string()));

        let second_entry = &response[1];
        assert_eq!(second_entry.symbol, "ETHUSD_PERP");
        assert_eq!(second_entry.income_type, IncomeType::FundingFee);
        assert_eq!(second_entry.income, "0.00234567");
        assert_eq!(second_entry.asset, "ETH");
        assert_eq!(second_entry.info, None);
        assert_eq!(second_entry.tran_id, None);
        assert_eq!(second_entry.trade_id, None);
    }

    #[test]
    fn test_income_history_response_deserialization_empty() {
        let json = r#"[]"#;
        let response: Vec<IncomeHistoryEntry> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 0);
    }
}
