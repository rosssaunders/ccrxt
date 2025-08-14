use serde::Serialize;

use super::{client::RestClient, get_trading_accounts::TradingAccount};
use crate::bullish::{EndpointType, RestResult};

const SINGLE_TRADING_ACCOUNT_ENDPOINT: &str = "/v1/accounts/trading-accounts";

/// Request parameters for getting a specific trading account by ID.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetTradingAccountRequest {
    /// The trading account ID to retrieve (path parameter)
    #[serde(skip_serializing)]
    pub trading_account_id: String,
}

impl RestClient {
    /// Get trading account by ID
    ///
    /// Gets details for a specific trading account by trading account ID.
    ///
    /// [docs](https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#get-/v1/accounts/trading-accounts/-tradingAccountId-)
    ///
    /// # Arguments
    /// * `request` - Request parameters containing the trading account ID
    ///
    /// # Returns
    /// Trading account information for the specified account
    pub async fn get_trading_account(
        &mut self,
        request: GetTradingAccountRequest,
    ) -> RestResult<TradingAccount> {
        let endpoint = format!(
            "{}/{}",
            SINGLE_TRADING_ACCOUNT_ENDPOINT, request.trading_account_id
        );

        self.send_get_request(&endpoint, (), EndpointType::PrivateTradingAccounts)
            .await
    }
}
