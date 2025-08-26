use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const SUB_ACCOUNT_TRANSFERS_ENDPOINT: &str = "/wallet/sub_account_transfers";
const SUB_ACCOUNT_TO_SUB_ACCOUNT_ENDPOINT: &str = "/wallet/sub_account_to_sub_account";

/// Request to transfer between main and sub accounts
#[derive(Debug, Clone, Serialize)]
pub struct SubAccountTransferRequest {
    /// Currency to transfer
    pub currency: String,

    /// Sub-account user ID
    pub sub_uid: String,

    /// Direction: "to" (main to sub) or "from" (sub to main)
    pub direction: String,

    /// Amount to transfer
    pub amount: String,

    /// Client order ID for tracking
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
}

/// Request to transfer between sub-accounts
#[derive(Debug, Clone, Serialize)]
pub struct SubToSubTransferRequest {
    /// Currency to transfer
    pub currency: String,

    /// Source sub-account user ID
    pub from_uid: String,

    /// Destination sub-account user ID
    pub to_uid: String,

    /// Amount to transfer
    pub amount: String,
}

/// Request parameters for querying sub-account transfer records
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetSubAccountTransfersRequest {
    /// Sub-account user ID filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_uid: Option<String>,

    /// Start time filter (Unix timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time filter (Unix timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Page size limit (max 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Page offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

/// Sub-account transfer record
#[derive(Debug, Clone, Deserialize)]
pub struct SubAccountTransferRecord {
    /// Transfer record ID
    pub id: String,

    /// Currency transferred
    pub currency: String,

    /// Sub-account user ID
    pub sub_uid: String,

    /// Transfer direction
    pub direction: String,

    /// Transfer amount
    pub amount: String,

    /// Transfer timestamp
    pub create_time: i64,

    /// Client order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
}

/// Transfer response
#[derive(Debug, Clone, Deserialize)]
pub struct TransferResponse {
    /// Transfer transaction ID
    pub tx_id: String,
}

impl RestClient {
    /// Transfer Between Main and Sub Accounts
    ///
    /// Transfer funds between main account and sub-account.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#transfer-between-main-and-sub-accounts)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `req` - Transfer request with currency, sub-account, direction and amount
    ///
    /// # Returns
    /// Transfer response with transaction ID
    pub async fn create_sub_account_transfer(
        &self,
        req: SubAccountTransferRequest,
    ) -> RestResult<TransferResponse> {
        self.send_post_request(SUB_ACCOUNT_TRANSFERS_ENDPOINT, Some(&req))
            .await
    }

    /// Get Transfer Records Between Main and Sub Accounts
    ///
    /// Retrieve historical transfer records between main account and sub-accounts.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#get-transfer-records-between-main-and-sub-accounts)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `req` - Optional request parameters for filtering and pagination
    ///
    /// # Returns
    /// List of sub-account transfer records
    pub async fn get_sub_account_transfers(
        &self,
        req: Option<GetSubAccountTransfersRequest>,
    ) -> RestResult<Vec<SubAccountTransferRecord>> {
        self.send_get_request(SUB_ACCOUNT_TRANSFERS_ENDPOINT, req.as_ref())
            .await
    }

    /// Transfer Between Sub-Accounts
    ///
    /// Transfer funds directly between two sub-accounts.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#transfer-between-sub-accounts)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `req` - Transfer request with currency, source and destination sub-accounts, and amount
    ///
    /// # Returns
    /// Transfer response with transaction ID
    pub async fn create_sub_to_sub_transfer(
        &self,
        req: SubToSubTransferRequest,
    ) -> RestResult<TransferResponse> {
        self.send_post_request(SUB_ACCOUNT_TO_SUB_ACCOUNT_ENDPOINT, Some(&req))
            .await
    }
}
