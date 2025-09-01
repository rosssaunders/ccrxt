use serde::Serialize;

use crate::bullish::{EndpointType, PrivateRestClient as RestClient, RestResult};

/// Endpoint URL for AMM instructions
const AMM_INSTRUCTIONS_ENDPOINT: &str = "/v2/amm-instructions";

impl RestClient {
    /// Get AMM Instruction by ID
    ///
    /// Returns details for a specific AMM instruction.
    ///
    /// [docs](https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#get-/v2/amm-instructions/-instructionId-)
    pub async fn get_amm_instruction(
        &mut self,
        instruction_id: &str,
        trading_account_id: &str,
    ) -> RestResult<super::amm_types::AmmInstruction> {
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(rename = "tradingAccountId")]
            trading_account_id: &'a str,
        }

        let endpoint = format!("{}/{}", AMM_INSTRUCTIONS_ENDPOINT, instruction_id);
        self.send_get_authenticated_request(
            &endpoint,
            Query { trading_account_id },
            EndpointType::PrivateOrders,
        )
        .await
    }
}
