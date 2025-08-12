use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bullish::{EndpointType, RestResult};

/// Endpoint URL for AMM instructions
const AMM_INSTRUCTIONS_ENDPOINT: &str = "/v2/amm-instructions";

/// Command type for terminating an AMM instruction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum TerminateAmmCommandType {
    #[serde(rename = "V3TerminateAMMInstruction")]
    V3TerminateAMMInstruction,
}

/// Terminate AMM instruction request
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminateAmmInstructionRequest {
    /// The command type, must be 'V3TerminateAMMInstruction'.
    #[serde(rename = "commandType")]
    pub command_type: TerminateAmmCommandType,

    /// Instruction ID to terminate.
    #[serde(rename = "instructionId")]
    pub instruction_id: String,

    /// Market symbol (e.g., BTCUSDC).
    pub symbol: String,

    /// Trading account ID which owns the AMM.
    #[serde(rename = "tradingAccountId")]
    pub trading_account_id: String,
}

/// Response for terminate AMM instruction (V3)
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminateAmmInstructionResponseV3 {
    /// Acknowledgement message.
    pub message: String,

    /// Request ID.
    #[serde(rename = "requestId")]
    pub request_id: String,

    /// Instruction ID.
    #[serde(rename = "instructionId")]
    pub instruction_id: String,
}

impl RestClient {
    /// Terminate AMM Instruction (V3TerminateAMMInstruction)
    ///
    /// [docs]: https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#post-/v2/amm-instructions
    pub async fn terminate_amm_instruction(
        &mut self,
        request: TerminateAmmInstructionRequest,
    ) -> RestResult<TerminateAmmInstructionResponseV3> {
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
    fn test_terminate_amm_instruction_request_serialization() {
        let req = TerminateAmmInstructionRequest {
            command_type: TerminateAmmCommandType::V3TerminateAMMInstruction,
            instruction_id: "633906221577404424".to_string(),
            symbol: "BTCUSDC".to_string(),
            trading_account_id: "111000000000001".to_string(),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("instructionId"));
    }
}
