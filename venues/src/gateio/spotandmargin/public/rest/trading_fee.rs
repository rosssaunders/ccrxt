use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for trading fee inquiry
#[derive(Debug, Clone, Serialize, Default)]
pub struct TradingFeeRequest {
    /// Currency pair to query fee for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,
}

/// Trading fee information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingFee {
    /// Currency pair
    pub currency_pair: String,

    /// Maker fee rate
    pub maker_fee: String,

    /// Taker fee rate
    pub taker_fee: String,

    /// GT deduction enabled
    pub gt_deduction: bool,

    /// GT taker fee rate
    pub gt_taker_fee: String,

    /// GT maker fee rate
    pub gt_maker_fee: String,

    /// Loan fee rate
    pub loan_fee: String,

    /// Point type (0: GT, 1: Point card, 2: Disabled)
    pub point_type: i32,
}

/// Batch trading fee information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchTradingFee {
    /// Currency pair
    pub currency_pair: String,

    /// Maker fee rate
    pub maker_fee: String,

    /// Taker fee rate  
    pub taker_fee: String,
}

impl RestClient {
    /// Get trading fee information for a currency pair
    ///
    /// This endpoint returns the current trading fees for a specific currency pair
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#query-user-trading-fee-rates>
    /// or all pairs if no pair is specified.
    pub async fn get_trading_fee(
        &self,
        params: TradingFeeRequest,
    ) -> crate::gateio::spotandmargin::Result<TradingFee> {
        self.get_with_query("/spot/fee", Some(&params)).await
    }

    /// Get batch trading fee information
    ///
    /// This endpoint returns trading fees for multiple currency pairs at once.
    pub async fn get_batch_trading_fee(
        &self,
    ) -> crate::gateio::spotandmargin::Result<Vec<BatchTradingFee>> {
        self.get("/spot/batch_fee").await
    }
}
