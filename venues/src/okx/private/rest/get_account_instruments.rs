use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, InstrumentType, RestResult};

/// Request to get account instruments
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountInstrumentsRequest {
    /// Instrument type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<InstrumentType>,

    /// Underlying, e.g. "BTC-USD"
    /// Only applicable to FUTURES/SWAP/OPTION
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,

    /// Instrument family, e.g. "BTC-USD"
    /// Only applicable to FUTURES/SWAP/OPTION
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,

    /// Instrument ID, e.g. "BTC-USDT"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
}

/// Account instrument details
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInstrument {
    /// Instrument ID
    pub inst_id: String,

    /// Instrument type
    pub inst_type: String,

    /// Underlying
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,

    /// Instrument family
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,

    /// Category
    pub category: String,

    /// Base currency
    pub base_ccy: String,

    /// Quote currency
    pub quote_ccy: String,

    /// Settlement currency
    pub settle_ccy: String,

    /// Contract value
    pub ct_val: String,

    /// Contract multiplier
    pub ct_mult: String,

    /// Contract value currency
    pub ct_val_ccy: String,

    /// Option type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_type: Option<String>,

    /// Strike price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stk: Option<String>,

    /// Listing date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_time: Option<String>,

    /// Expiry date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_time: Option<String>,

    /// Leverage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lever: Option<String>,

    /// Tick size
    pub tick_sz: String,

    /// Lot size
    pub lot_sz: String,

    /// Minimum size
    pub min_sz: String,

    /// Contract type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ct_type: Option<String>,

    /// Alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,

    /// State
    pub state: String,

    /// Maximum leverage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_lmsr: Option<String>,

    /// Maximum iceberg size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_twap_sz: Option<String>,

    /// Maximum trigger size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_trigger_sz: Option<String>,

    /// Maximum stop market size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_stop_sz: Option<String>,
}

impl RestClient {
    /// Get account instruments
    ///
    /// # Arguments
    /// * `request` - The get account instruments request
    ///
    /// # Returns
    /// A result containing the account instruments or an error
    pub async fn get_account_instruments(&self, request: &GetAccountInstrumentsRequest) -> RestResult<OkxApiResponse<AccountInstrument>> {
        self.send_request(
            "api/v5/account/instruments",
            reqwest::Method::GET,
            Some(request),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_account_instruments_request_serialization() {
        let request = GetAccountInstrumentsRequest {
            inst_type: Some(InstrumentType::Spot),
            uly: None,
            inst_family: None,
            inst_id: Some("BTC-USDT".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("instType=SPOT"));
        assert!(serialized.contains("instId=BTC-USDT"));
    }

    #[test]
    fn test_get_account_instruments_minimal_request() {
        let request = GetAccountInstrumentsRequest {
            inst_type: None,
            uly: None,
            inst_family: None,
            inst_id: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_account_instrument_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instId": "BTC-USDT",
                    "instType": "SPOT",
                    "category": "1",
                    "baseCcy": "BTC",
                    "quoteCcy": "USDT",
                    "settleCcy": "",
                    "ctVal": "",
                    "ctMult": "",
                    "ctValCcy": "",
                    "optType": "",
                    "stk": "",
                    "listTime": "",
                    "expTime": "",
                    "lever": "10",
                    "tickSz": "0.1",
                    "lotSz": "0.00000001",
                    "minSz": "0.00001",
                    "ctType": "",
                    "alias": "",
                    "state": "live",
                    "maxLmsr": "0.1",
                    "maxTwapSz": "999999999",
                    "maxTriggerSz": "999999999",
                    "maxStopSz": "999999999"
                }
            ]
        }"#;

        let response: OkxApiResponse<AccountInstrument> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let instrument = &response.data[0];
        assert_eq!(instrument.inst_id, "BTC-USDT");
        assert_eq!(instrument.inst_type, "SPOT");
        assert_eq!(instrument.base_ccy, "BTC");
        assert_eq!(instrument.quote_ccy, "USDT");
        assert_eq!(instrument.state, "live");
    }
}