//! Custody history endpoint for Bullish Exchange API

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bullish::{
    DataOrPaginated, EndpointType, PaginatedResult, PaginationParams, RestResult,
};

/// Endpoint URL path for custody transaction history
const CUSTODY_HISTORY_ENDPOINT: &str = "/v1/wallets/transactions";

/// Direction of custody transaction from the user's perspective
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CustodyDirection {
    Deposit,
    Withdrawal,
}

/// Status of a custody transaction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CustodyTransactionStatus {
    Pending,
    Complete,
    Cancelled,
    Failed,
}

/// Details for a custody transaction (network or bank reference info)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustodyTransactionDetails {
    /// Crypto network address
    pub address: Option<String>,

    /// Blockchain transaction id (hash)
    pub blockchain_tx_id: Option<String>,

    /// SWIFT unique end-to-end transaction reference
    pub swift_uetr: Option<String>,
}

/// Custody history record
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustodyHistory {
    /// Unique identifier for tracking a deposit or withdrawal
    pub custody_transaction_id: String,

    /// Direction of transaction from user's perspective
    pub direction: CustodyDirection,

    /// Total quantity of symbol; fee is subtracted for received quantity
    pub quantity: String,

    /// Asset symbol (non-multiplied), e.g. USDC, BTC, ETH, SHIB
    pub symbol: String,

    /// Network of the asset (e.g., BTC, ETH, EOS)
    pub network: Option<String>,

    /// Withdrawal fee charged (units of the asset)
    pub fee: Option<String>,

    /// Memo or destination tag used during deposit
    pub memo: Option<String>,

    /// Time of initial transaction (ISO 8601 with milliseconds)
    #[serde(rename = "createdAtDateTime")]
    pub created_at_datetime: String,

    /// Status of the custody transaction
    pub status: CustodyTransactionStatus,

    /// Extra details (address, chain tx id, swift uetr)
    #[serde(default)]
    pub transaction_details: Option<CustodyTransactionDetails>,
}

/// Query parameters for custody history
#[derive(Debug, Clone, Default, Serialize)]
pub struct CustodyHistoryParams {
    /// Start timestamp of period, ISO 8601 with millisecond as string
    #[serde(
        rename = "createdAtDatetime[gte]",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at_datetime_gte: Option<String>,

    /// End timestamp of period, ISO 8601 with millisecond as string
    #[serde(
        rename = "createdAtDatetime[lte]",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at_datetime_lte: Option<String>,

    /// Pagination controls
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

impl RestClient {
    /// Get custody transaction history
    ///
    /// Requires a valid Bearer token. Supports cursor pagination when `_metaData=true`.
    pub async fn get_custody_history(
        &mut self,
        params: CustodyHistoryParams,
    ) -> RestResult<PaginatedResult<CustodyHistory>> {
        let wire: DataOrPaginated<CustodyHistory> = self
            .send_get_authenticated_request(
                CUSTODY_HISTORY_ENDPOINT,
                params,
                EndpointType::PrivateCustody,
            )
            .await?;

        Ok(PaginatedResult::from(wire))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enums_serialize_uppercase() {
        assert_eq!(
            serde_json::to_string(&CustodyDirection::Deposit).unwrap(),
            "\"DEPOSIT\""
        );
        assert_eq!(
            serde_json::to_string(&CustodyDirection::Withdrawal).unwrap(),
            "\"WITHDRAWAL\""
        );
        assert_eq!(
            serde_json::to_string(&CustodyTransactionStatus::Complete).unwrap(),
            "\"COMPLETE\""
        );
    }

    #[test]
    fn test_params_query_keys() {
        let params = CustodyHistoryParams {
            created_at_datetime_gte: Some("2023-01-01T00:00:00.000Z".into()),
            created_at_datetime_lte: Some("2023-02-01T00:00:00.000Z".into()),
            pagination: PaginationParams {
                meta_data: Some(true),
                page_size: Some(25),
                next_page: None,
                previous_page: None,
            },
        };
        let qs = serde_urlencoded::to_string(&params).unwrap();
        assert!(qs.contains("createdAtDatetime%5Bgte%5D=2023-01-01T00%3A00%3A00.000Z"));
        assert!(qs.contains("createdAtDatetime%5Blte%5D=2023-02-01T00%3A00%3A00.000Z"));
        assert!(qs.contains("_pageSize=25"));
        assert!(qs.contains("_metaData=true"));
    }
}
