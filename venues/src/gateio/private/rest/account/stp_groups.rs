use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

/// STP user group information returned by Gate.io
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StpGroup {
    /// STP Group ID
    pub id: i64,

    /// STP Group name
    #[serde(default)]
    pub name: Option<String>,

    /// Creator ID
    #[serde(default)]
    pub creator_id: Option<i64>,

    /// Creation time (seconds since epoch)
    #[serde(default)]
    pub create_time: Option<i64>,
}

/// Member of an STP group
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StpGroupUser {
    /// User ID
    pub user_id: i64,

    /// STP Group ID
    pub stp_id: i64,

    /// Creation time (seconds since epoch)
    #[serde(default)]
    pub create_time: Option<i64>,
}

/// Request to create a new STP group
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateStpGroupRequest {
    /// STP Group name
    pub name: String,
}

/// Query parameters for listing STP groups
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ListStpGroupsQuery {
    /// Fuzzy search by name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Query parameters for deleting users from an STP group
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct DeleteStpUsersQuery {
    /// STP user IDs, multiple IDs can be separated by commas
    pub user_id: String,
}

const STP_GROUPS_ENDPOINT: &str = "/account/stp_groups";

impl RestClient {
    /// Create STP user group
    ///
    /// Only the main account is allowed to create a new STP user group.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#create-stp-user-group)
    ///
    /// Rate limit: varies by endpoint type
    ///
    /// # Arguments
    /// * `req` - Parameters for creating the STP group
    ///
    /// # Returns
    /// The created `StpGroup` (may contain only partial fields depending on the API response)
    pub async fn create_stp_group(&self, req: CreateStpGroupRequest) -> RestResult<StpGroup> {
        self.send_post_request(STP_GROUPS_ENDPOINT, Some(&req))
            .await
    }

    /// Query STP user groups created by the current main account
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#query-stp-user-groups-created-by-the-user)
    ///
    /// Rate limit: varies by endpoint type
    ///
    /// # Arguments
    /// * `query` - Optional fuzzy name filter
    ///
    /// # Returns
    /// List of `StpGroup` objects
    pub async fn list_stp_groups(&self, query: ListStpGroupsQuery) -> RestResult<Vec<StpGroup>> {
        self.send_get_request(STP_GROUPS_ENDPOINT, Some(&query))
            .await
    }

    /// Query users in the STP user group
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#query-users-in-the-stp-user-group)
    ///
    /// Rate limit: varies by endpoint type
    ///
    /// # Arguments
    /// * `stp_id` - STP group id (path parameter)
    ///
    /// # Returns
    /// List of `StpGroupUser` objects
    pub async fn list_stp_group_users(&self, stp_id: i64) -> RestResult<Vec<StpGroupUser>> {
        let endpoint = format!("{}/{}/users", STP_GROUPS_ENDPOINT, stp_id);
        self.send_get_request(&endpoint, Option::<&()>::None).await
    }

    /// Add users to the STP user group
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#add-users-to-the-stp-user-group)
    ///
    /// Rate limit: varies by endpoint type
    ///
    /// # Arguments
    /// * `stp_id` - STP group id (path parameter)
    /// * `users` - Slice of user ids to add
    ///
    /// # Returns
    /// List of `StpGroupUser` objects representing the users added
    pub async fn add_stp_group_users(
        &self,
        stp_id: i64,
        users: &[i64],
    ) -> RestResult<Vec<StpGroupUser>> {
        let endpoint = format!("{}/{}/users", STP_GROUPS_ENDPOINT, stp_id);
        self.send_post_request(&endpoint, Some(&users)).await
    }

    /// Delete users from the STP user group
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#delete-user-in-stp-group)
    ///
    /// Rate limit: varies by endpoint type
    ///
    /// # Arguments
    /// * `stp_id` - STP group id (path parameter)
    /// * `query` - Query specifying `user_id` (comma-separated)
    ///
    /// # Returns
    /// List of `StpGroupUser` objects representing the remaining users
    pub async fn delete_stp_group_users(
        &self,
        stp_id: i64,
        query: DeleteStpUsersQuery,
    ) -> RestResult<Vec<StpGroupUser>> {
        let endpoint = format!("{}/{}/users", STP_GROUPS_ENDPOINT, stp_id);
        self.send_delete_request(&endpoint, Some(&query)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoints() {
        assert_eq!(STP_GROUPS_ENDPOINT, "/account/stp_groups");
        let users_ep = format!("{}/{}/users", STP_GROUPS_ENDPOINT, 1);
        assert_eq!(users_ep, "/account/stp_groups/1/users");
    }

    #[test]
    fn test_deserialize_create_response() {
        let json = r#"{ "id": 123435, "creator_id": 10000 }"#;
        let group: StpGroup = serde_json::from_str(json).unwrap();
        assert_eq!(group.id, 123435);
        assert_eq!(group.creator_id, Some(10000));
        assert!(group.name.is_none());
    }

    #[test]
    fn test_serialize_create_request() {
        let req = CreateStpGroupRequest {
            name: "stp_name".into(),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("stp_name"));
    }

    #[test]
    fn test_list_query_serialization() {
        let query = ListStpGroupsQuery {
            name: Some("abc".into()),
        };
        let qs = serde_urlencoded::to_string(&query).unwrap();
        assert_eq!(qs, "name=abc");
    }
}
