use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const SUB_ACCOUNTS_ENDPOINT: &str = "/sub_accounts";

/// Create sub-account request
#[derive(Debug, Clone, Serialize)]
pub struct CreateSubAccountRequest {
    /// Sub-account login name
    pub login_name: String,

    /// Note or remark for the sub-account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,

    /// Sub-account password
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// Sub-account email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// Sub-account creation response
#[derive(Debug, Clone, Deserialize)]
pub struct CreateSubAccountResponse {
    /// Sub-account user ID
    pub user_id: String,

    /// Login name
    pub login_name: String,

    /// Account status
    pub status: String,

    /// Creation timestamp
    pub create_time: i64,

    /// Remark or note
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
}

impl RestClient {
    /// Create Sub-Account
    ///
    /// Create a new sub-account under the main account with specified login credentials.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#create-sub-account)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `req` - Sub-account creation request with login name and optional details
    ///
    /// # Returns
    /// Sub-account creation response with user ID and status
    pub async fn create_sub_account(
        &self,
        req: CreateSubAccountRequest,
    ) -> RestResult<CreateSubAccountResponse> {
        self.send_post_request(SUB_ACCOUNTS_ENDPOINT, Some(&req))
            .await
    }
}
