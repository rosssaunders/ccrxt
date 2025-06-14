use serde::{Deserialize, Serialize};
use super::client::RestClient;
use crate::okx::{EndpointType, InstrumentType, InstrumentState, RestResult};

/// Request parameters for getting instruments
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInstrumentsRequest {
    /// Instrument type filter
    #[serde(rename = "instType")]
    pub inst_type: Option<InstrumentType>,
    /// Underlying (for SWAP/FUTURES/OPTION)
    #[serde(rename = "uly")]
    pub underlying: Option<String>,
    /// Instrument family (for FUTURES/SWAP/OPTION)
    #[serde(rename = "instFamily")]
    pub inst_family: Option<String>,
    /// Instrument ID
    #[serde(rename = "instId")]
    pub inst_id: Option<String>,
}

/// Individual instrument details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instrument {
    /// Instrument type
    #[serde(rename = "instType")]
    pub inst_type: InstrumentType,
    /// Instrument ID (e.g., "BTC-USDT")
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Underlying (e.g., "BTC-USD")
    #[serde(rename = "uly")]
    pub underlying: Option<String>,
    /// Category (e.g., "1")
    pub category: String,
    /// Base currency (e.g., "BTC")
    #[serde(rename = "baseCcy")]
    pub base_ccy: String,
    /// Quote currency (e.g., "USDT")
    #[serde(rename = "quoteCcy")]
    pub quote_ccy: String,
    /// Settlement currency (e.g., "BTC")
    #[serde(rename = "settleCcy")]
    pub settle_ccy: String,
    /// Contract value (for derivatives)
    #[serde(rename = "ctVal")]
    pub ct_val: Option<String>,
    /// Contract multiplier (for derivatives)
    #[serde(rename = "ctMult")]
    pub ct_mult: Option<String>,
    /// Contract value currency (for derivatives)
    #[serde(rename = "ctValCcy")]
    pub ct_val_ccy: Option<String>,
    /// Option type ("C" for call, "P" for put)
    #[serde(rename = "optType")]
    pub opt_type: Option<String>,
    /// Strike price (for options)
    #[serde(rename = "stk")]
    pub strike: Option<String>,
    /// Listing time (Unix timestamp in milliseconds)
    #[serde(rename = "listTime")]
    pub list_time: String,
    /// Expiry time (Unix timestamp in milliseconds)
    #[serde(rename = "expTime")]
    pub exp_time: Option<String>,
    /// Leverage (for margin trading)
    pub lever: Option<String>,
    /// Tick size
    #[serde(rename = "tickSz")]
    pub tick_sz: String,
    /// Lot size
    #[serde(rename = "lotSz")]
    pub lot_sz: String,
    /// Minimum order size
    #[serde(rename = "minSz")]
    pub min_sz: String,
    /// Contract type ("linear" or "inverse")
    #[serde(rename = "ctType")]
    pub ct_type: Option<String>,
    /// Alias ("this_week", "next_week", "quarter", "next_quarter")
    pub alias: Option<String>,
    /// Instrument state
    pub state: InstrumentState,
    /// Maximum order quantity for buy orders
    #[serde(rename = "maxBuyQty")]
    pub max_buy_qty: Option<String>,
    /// Maximum order quantity for sell orders
    #[serde(rename = "maxSellQty")]
    pub max_sell_qty: Option<String>,
    /// Maximum order amount for buy orders
    #[serde(rename = "maxBuyAmt")]
    pub max_buy_amt: Option<String>,
    /// Maximum order amount for sell orders
    #[serde(rename = "maxSellAmt")]
    pub max_sell_amt: Option<String>,
}

/// Response for getting instruments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInstrumentsResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Instrument data
    pub data: Vec<Instrument>,
}

impl RestClient {
    /// Get available trading instruments
    ///
    /// Retrieve a list of instruments with open contracts.
    ///
    /// See: https://www.okx.com/docs-v5/en/#rest-api-public-data-get-instruments
    ///
    /// Rate limit: 20 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The instruments request parameters
    ///
    /// # Returns
    /// Response containing the list of available instruments
    pub async fn get_instruments(
        &self,
        request: Option<GetInstrumentsRequest>,
    ) -> RestResult<GetInstrumentsResponse> {
        self.send_request(
            "api/v5/public/instruments",
            reqwest::Method::GET,
            request.as_ref(),
            EndpointType::PublicMarketData,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_instruments_request_structure() {
        let request = GetInstrumentsRequest {
            inst_type: Some(InstrumentType::Spot),
            underlying: None,
            inst_family: None,
            inst_id: Some("BTC-USDT".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("instType").and_then(|v| v.as_str()), Some("SPOT"));
        assert_eq!(serialized.get("instId").and_then(|v| v.as_str()), Some("BTC-USDT"));
    }

    #[test]
    fn test_instrument_structure() {
        let instrument_json = json!({
            "instType": "SPOT",
            "instId": "BTC-USDT",
            "baseCcy": "BTC",
            "quoteCcy": "USDT",
            "settleCcy": "USDT",
            "category": "1",
            "tickSz": "0.1",
            "lotSz": "0.00000001",
            "minSz": "0.00001",
            "state": "live",
            "listTime": "1606276800000"
        });

        let instrument: Instrument = serde_json::from_value(instrument_json).unwrap();
        assert_eq!(instrument.inst_type, InstrumentType::Spot);
        assert_eq!(instrument.inst_id, "BTC-USDT");
        assert_eq!(instrument.base_ccy, "BTC");
        assert_eq!(instrument.quote_ccy, "USDT");
        assert_eq!(instrument.state, InstrumentState::Live);
    }

    #[test]
    fn test_get_instruments_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instType": "SPOT",
                    "instId": "BTC-USDT",
                    "baseCcy": "BTC",
                    "quoteCcy": "USDT",
                    "settleCcy": "USDT",
                    "category": "1",
                    "tickSz": "0.1",
                    "lotSz": "0.00000001",
                    "minSz": "0.00001",
                    "state": "live",
                    "listTime": "1606276800000"
                }
            ]
        });

        let response: GetInstrumentsResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data.first().unwrap().inst_id, "BTC-USDT");
    }
}