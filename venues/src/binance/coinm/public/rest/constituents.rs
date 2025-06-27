use crate::binance::coinm::RestResult;
use crate::binance::coinm::public::rest::RestClient;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Parameters for Query Index Price Constituents
#[derive(Debug, Clone, Serialize, Default)]
pub struct ConstituentsRequest {
    /// Symbol name
    pub symbol: String,
}

/// Index price constituent
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Constituent {
    /// Exchange name
    pub exchange: String,

    /// Symbol name
    pub symbol: String,

    /// Price
    pub price: Decimal,

    /// Weight
    pub weight: Decimal,
}

/// Index price constituents response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Constituents {
    /// Symbol name
    pub symbol: String,

    /// Timestamp
    pub time: i64,

    /// Constituents
    pub constituents: Vec<Constituent>,
}

impl RestClient {
    /// Query index price constituents
    ///
    /// Weight: 2
    pub async fn get_constituents(&self, params: ConstituentsRequest) -> RestResult<Constituents> {
        self.send_request(
            "/dapi/v1/constituents",
            reqwest::Method::GET,
            Some(params),
            2,
        )
        .await
    }
}
