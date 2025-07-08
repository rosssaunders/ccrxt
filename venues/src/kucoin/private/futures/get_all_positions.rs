use serde::{Deserialize, Serialize};

use super::Position;
use crate::kucoin::{ResponseHeaders, RestResponse, Result};

/// Endpoint URL for get all positions
pub const GET_ALL_POSITIONS_ENDPOINT: &str = "/api/v1/positions";

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAllPositionsRequest;

pub type GetAllPositionsResponse = Vec<Position>;

impl super::RestClient {
    /// Get all positions
    pub async fn get_all_positions(
        &self,
        _request: GetAllPositionsRequest,
    ) -> Result<(RestResponse<GetAllPositionsResponse>, ResponseHeaders)> {
        self.get(GET_ALL_POSITIONS_ENDPOINT, None).await
    }
}
