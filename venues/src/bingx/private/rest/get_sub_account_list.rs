use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSubAccountListRequest {
    /// Sub-account email (optional filter)
    pub email: Option<String>,
    /// Page number, starting from 1
    pub page: Option<i32>,
    /// Number of items per page, max 200
    pub size: Option<i32>,
    /// Timestamp in ms
    pub recv_window: Option<i64>,
    /// Timestamp in ms
    pub timestamp: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSubAccountListResponse {
    /// List of sub-accounts
    pub sub_accounts: Vec<SubAccountInfo>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountInfo {
    /// Sub-account email
    pub email: String,
    /// Sub-account UID
    pub sub_account_id: String,
    /// Account status
    pub status: String,
    /// Whether account is activated
    pub activated: bool,
    /// Whether account has mobile verification
    pub mobile: bool,
    /// Whether account has Google 2FA
    pub gauth: bool,
    /// Creation time
    pub create_time: i64,
}

impl GetSubAccountListRequest {
    pub fn new(timestamp: i64) -> Self {
        Self {
            email: None,
            page: None,
            size: None,
            recv_window: None,
            timestamp,
        }
    }

    pub fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    pub fn page(mut self, page: i32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sub_account_list_request_serialization() {
        let request = GetSubAccountListRequest::new(1640995200000)
            .email("test@example.com".to_string())
            .page(1)
            .size(10)
            .recv_window(5000);

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"email\":\"test@example.com\""));
        assert!(json.contains("\"page\":1"));
        assert!(json.contains("\"size\":10"));
        assert!(json.contains("\"recvWindow\":5000"));
        assert!(json.contains("\"timestamp\":1640995200000"));
    }

    #[test]
    fn test_get_sub_account_list_response_deserialization() {
        let json = r#"
        {
            "subAccounts": [
                {
                    "email": "test1@example.com",
                    "subAccountId": "12345678",
                    "status": "normal",
                    "activated": true,
                    "mobile": true,
                    "gauth": false,
                    "createTime": 1640995200000
                },
                {
                    "email": "test2@example.com",
                    "subAccountId": "87654321",
                    "status": "normal",
                    "activated": true,
                    "mobile": false,
                    "gauth": true,
                    "createTime": 1640995200000
                }
            ]
        }
        "#;

        let response: GetSubAccountListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.sub_accounts.len(), 2);
        assert_eq!(response.sub_accounts[0].email, "test1@example.com");
        assert_eq!(response.sub_accounts[0].sub_account_id, "12345678");
        assert!(response.sub_accounts[0].activated);
        assert!(response.sub_accounts[0].mobile);
        assert!(!response.sub_accounts[0].gauth);
    }
}
