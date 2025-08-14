use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::okx::{EndpointType, InstrumentType, RestResult};

const ACCOUNT_ACCOUNT_POSITION_RISK_ENDPOINT: &str = "api/v5/account/account-position-risk";

/// Request to get account position risk
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountPositionRiskRequest {
    /// Instrument type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<InstrumentType>,
}

/// Account position risk details
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountPositionRisk {
    /// Adjust equity in USD
    pub adj_eq: Option<String>,

    /// Balance details
    pub bal_data: Vec<BalanceRiskData>,

    /// Position details
    pub pos_data: Vec<PositionRiskData>,

    /// Timestamp
    pub ts: String,
}

/// Balance risk data
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceRiskData {
    /// Currency
    pub ccy: String,

    /// Equity
    pub eq: String,

    /// Discount equity of currency in USD
    pub dis_eq: String,
}

/// Position risk data
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionRiskData {
    /// Instrument ID
    pub inst_id: String,

    /// Instrument type
    pub inst_type: String,

    /// Margin mode
    pub mgn_mode: String,

    /// Notional value of currency in USD
    pub notional_ccy: String,

    /// Notional value in USD
    pub notional_usd: String,

    /// Position side
    pub pos_side: String,

    /// Position size
    pub pos: String,

    /// Position size in base currency
    pub base_pos: String,

    /// Currency for position
    pub pos_ccy: String,

    /// Average cost
    pub avg_px: String,

    /// Unrealized PnL
    pub upl: String,

    /// Unrealized PnL ratio
    pub upl_ratio: String,

    /// Underlying
    pub uly: Option<String>,

    /// Delta BS
    pub delta_bs: Option<String>,

    /// Delta PA
    pub delta_pa: Option<String>,

    /// Gamma BS
    pub gamma_bs: Option<String>,

    /// Gamma PA
    pub gamma_pa: Option<String>,

    /// Theta BS
    pub theta_bs: Option<String>,

    /// Theta PA
    pub theta_pa: Option<String>,

    /// Vega BS
    pub vega_bs: Option<String>,

    /// Vega PA
    pub vega_pa: Option<String>,

    /// Currency
    pub ccy: String,
}

impl RestClient {
    /// Get account position risk
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#trading-account-rest-api-get-account-and-position-risk)
    ///
    /// # Arguments
    /// * `request` - The get account position risk request
    ///
    /// # Returns
    /// A result containing the account position risk or an error
    pub async fn get_account_position_risk(
        &self,
        request: &GetAccountPositionRiskRequest,
    ) -> RestResult<AccountPositionRisk> {
        self.send_get_request(
            ACCOUNT_ACCOUNT_POSITION_RISK_ENDPOINT,
            Some(request),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::okx::response::OkxApiResponse;

    #[test]
    fn test_get_account_position_risk_request_serialization() {
        let request = GetAccountPositionRiskRequest {
            inst_type: Some(InstrumentType::Swap),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("instType=SWAP"));
    }

    #[test]
    fn test_get_account_position_risk_minimal_request() {
        let request = GetAccountPositionRiskRequest { inst_type: None };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_account_position_risk_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "adjEq": "9999.12345",
                    "balData": [
                        {
                            "ccy": "USDT",
                            "eq": "9999.12345",
                            "disEq": "9999.12345"
                        }
                    ],
                    "posData": [
                        {
                            "instId": "BTC-USDT-SWAP",
                            "instType": "SWAP",
                            "mgnMode": "cross",
                            "notionalCcy": "USD",
                            "notionalUsd": "9999.12345",
                            "posSide": "long",
                            "pos": "100",
                            "basePos": "100",
                            "posCcy": "BTC",
                            "avgPx": "50000",
                            "upl": "100.5",
                            "uplRatio": "0.01",
                            "uly": "BTC-USD",
                            "deltaBs": "",
                            "deltaPa": "",
                            "gammaBs": "",
                            "gammaPa": "",
                            "thetaBs": "",
                            "thetaPa": "",
                            "vegaBs": "",
                            "vegaPa": "",
                            "ccy": "USDT"
                        }
                    ],
                    "ts": "1597026383085"
                }
            ]
        }"#;

        let response: OkxApiResponse<AccountPositionRisk> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let risk = &response.data[0];
        assert_eq!(risk.adj_eq, Some("9999.12345".to_string()));
        assert_eq!(risk.bal_data.len(), 1);
        assert_eq!(risk.pos_data.len(), 1);
        assert_eq!(risk.bal_data[0].ccy, "USDT");
        assert_eq!(risk.pos_data[0].inst_id, "BTC-USDT-SWAP");
    }
}
