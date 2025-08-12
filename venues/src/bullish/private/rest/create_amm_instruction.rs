use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bullish::{EndpointType, RestResult};

/// Endpoint URL for AMM instructions
const AMM_INSTRUCTIONS_ENDPOINT: &str = "/v2/amm-instructions";

/// Command type for creating an AMM instruction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum CreateAmmCommandType {
    #[serde(rename = "V3CreateAMMInstruction")]
    V3CreateAMMInstruction,
}

/// Create AMM instruction request
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAmmInstructionRequest {
    /// The command type, must be 'V3CreateAMMInstruction'.
    #[serde(rename = "commandType")]
    pub command_type: CreateAmmCommandType,

    /// Market symbol (e.g., BTCUSDC).
    pub symbol: String,

    /// Base quantity to provide.
    #[serde(rename = "baseQuantity")]
    pub base_quantity: String,

    /// Quote quantity to provide.
    #[serde(rename = "quoteQuantity")]
    pub quote_quantity: String,

    /// Upper price bound.
    #[serde(rename = "upperBound")]
    pub upper_bound: String,

    /// Lower price bound.
    #[serde(rename = "lowerBound")]
    pub lower_bound: String,

    /// Fee tier identifier.
    #[serde(rename = "feeTierId")]
    pub fee_tier_id: String,

    /// Trading account ID which owns the AMM.
    #[serde(rename = "tradingAccountId")]
    pub trading_account_id: String,
}

/// Response for create AMM instruction (V3)
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAmmInstructionResponseV3 {
    /// Acknowledgement message.
    pub message: String,

    /// Request ID.
    #[serde(rename = "requestId")]
    pub request_id: String,

    /// Generated instruction ID.
    #[serde(rename = "instructionId")]
    pub instruction_id: String,
}

impl RestClient {
    /// Create AMM Instruction (V3CreateAMMInstruction)
    ///
    /// [docs]: https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#post-/v2/amm-instructions
    pub async fn create_amm_instruction(
        &mut self,
        request: CreateAmmInstructionRequest,
    ) -> RestResult<CreateAmmInstructionResponseV3> {
        self.send_post_signed_request(
            AMM_INSTRUCTIONS_ENDPOINT,
            request,
            EndpointType::PrivateOrders,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_amm_instruction_request_serialization() {
        let req = CreateAmmInstructionRequest {
            command_type: CreateAmmCommandType::V3CreateAMMInstruction,
            symbol: "BTCUSDC".to_string(),
            base_quantity: "0".to_string(),
            quote_quantity: "50000.1".to_string(),
            upper_bound: "25000".to_string(),
            lower_bound: "20000".to_string(),
            fee_tier_id: "1".to_string(),
            trading_account_id: "111000000000001".to_string(),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("V3CreateAMMInstruction"));
        assert!(json.contains("feeTierId"));
    }
}
