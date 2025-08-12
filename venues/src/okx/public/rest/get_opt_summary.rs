use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::okx::{EndpointType, InstrumentType, RestResult};

const PUBLIC_OPT_SUMMARY_ENDPOINT: &str = "api/v5/public/opt-summary";

/// Request parameters for getting option summary data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOptSummaryRequest {
    /// Underlying, only applicable to OPTION
    /// Either uly or instFamily is required. If both are passed, instFamily will be used.
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,

    /// Instrument family, only applicable to OPTION
    /// Either uly or instFamily is required. If both are passed, instFamily will be used.
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,

    /// Contract expiry date, the format is "YYMMDD", e.g. "200527"
    #[serde(rename = "expTime", skip_serializing_if = "Option::is_none")]
    pub exp_time: Option<String>,
}

/// Individual option summary data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptSummary {
    /// Instrument type (OPTION)
    #[serde(rename = "instType")]
    pub inst_type: InstrumentType,

    /// Instrument ID, e.g. BTC-USD-200103-5500-C
    #[serde(rename = "instId")]
    pub inst_id: String,

    /// Underlying
    #[serde(rename = "uly")]
    pub underlying: String,

    /// Sensitivity of option price to uly price
    pub delta: String,

    /// The delta is sensitivity to uly price
    pub gamma: String,

    /// Sensitivity of option price to implied volatility
    pub vega: String,

    /// Sensitivity of option price to remaining maturity
    pub theta: String,

    /// Sensitivity of option price to uly price in BS mode
    #[serde(rename = "deltaBS")]
    pub delta_bs: String,

    /// The delta is sensitivity to uly price in BS mode
    #[serde(rename = "gammaBS")]
    pub gamma_bs: String,

    /// Sensitivity of option price to implied volatility in BS mode
    #[serde(rename = "vegaBS")]
    pub vega_bs: String,

    /// Sensitivity of option price to remaining maturity in BS mode
    #[serde(rename = "thetaBS")]
    pub theta_bs: String,

    /// Leverage
    pub lever: String,

    /// Mark volatility
    #[serde(rename = "markVol")]
    pub mark_vol: String,

    /// Bid volatility
    #[serde(rename = "bidVol")]
    pub bid_vol: String,

    /// Ask volatility
    #[serde(rename = "askVol")]
    pub ask_vol: String,

    /// Realized volatility (not currently used)
    #[serde(rename = "realVol")]
    pub real_vol: String,

    /// Implied volatility of at-the-money options
    #[serde(rename = "volLv")]
    pub vol_lv: String,

    /// Forward price
    #[serde(rename = "fwdPx")]
    pub fwd_px: String,

    /// Data update time, Unix timestamp format in milliseconds, e.g. 1597026383085
    pub ts: String,
}

impl RestClient {
    /// Get option market data
    ///
    /// Retrieve option market data.
    ///
    /// [docs]: https://www.okx.com/docs-v5/en/#rest-api-public-rest-api-get-option-market-data
    ///
    /// Rate limit: 20 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The option summary request parameters
    ///
    /// # Returns
    /// Response containing option market data
    pub async fn get_opt_summary(
        &self,
        request: GetOptSummaryRequest,
    ) -> RestResult<OptSummary> {
        self.send_get_request(
            PUBLIC_OPT_SUMMARY_ENDPOINT,
            Some(&request),
            EndpointType::PublicMarketData,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::okx::response::OkxApiResponse;

    #[test]
    fn test_get_opt_summary_request_with_underlying() {
        let request = GetOptSummaryRequest {
            underlying: Some("BTC-USD".to_string()),
            inst_family: None,
            exp_time: Some("200527".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("uly").and_then(|v| v.as_str()),
            Some("BTC-USD")
        );
        assert_eq!(
            serialized.get("expTime").and_then(|v| v.as_str()),
            Some("200527")
        );
    }

    #[test]
    fn test_get_opt_summary_request_with_inst_family() {
        let request = GetOptSummaryRequest {
            underlying: None,
            inst_family: Some("BTC-USD".to_string()),
            exp_time: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instFamily").and_then(|v| v.as_str()),
            Some("BTC-USD")
        );
    }

    #[test]
    fn test_get_opt_summary_request_both_parameters() {
        let request = GetOptSummaryRequest {
            underlying: Some("BTC-USD".to_string()),
            inst_family: Some("BTC-USD-FAMILY".to_string()),
            exp_time: Some("200527".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("uly").and_then(|v| v.as_str()),
            Some("BTC-USD")
        );
        assert_eq!(
            serialized.get("instFamily").and_then(|v| v.as_str()),
            Some("BTC-USD-FAMILY")
        );
        assert_eq!(
            serialized.get("expTime").and_then(|v| v.as_str()),
            Some("200527")
        );
    }

    #[test]
    fn test_opt_summary_structure() {
        let opt_summary_json = json!({
            "instType": "OPTION",
            "instId": "BTC-USD-200103-5500-C",
            "uly": "BTC-USD",
            "delta": "0.7551",
            "gamma": "0.0001",
            "vega": "0.0923",
            "theta": "-0.0417",
            "deltaBS": "0.7551",
            "gammaBS": "0.0001",
            "vegaBS": "0.0923",
            "thetaBS": "-0.0417",
            "lever": "5.2",
            "markVol": "0.8321",
            "bidVol": "0.8201",
            "askVol": "0.8441",
            "realVol": "0.8100",
            "volLv": "0.8300",
            "fwdPx": "45123.34",
            "ts": "1597026383085"
        });

        let opt_summary: OptSummary = serde_json::from_value(opt_summary_json).unwrap();
        assert_eq!(opt_summary.inst_type, InstrumentType::Option);
        assert_eq!(opt_summary.inst_id, "BTC-USD-200103-5500-C");
        assert_eq!(opt_summary.underlying, "BTC-USD");
        assert_eq!(opt_summary.delta, "0.7551");
        assert_eq!(opt_summary.gamma, "0.0001");
        assert_eq!(opt_summary.vega, "0.0923");
        assert_eq!(opt_summary.theta, "-0.0417");
        assert_eq!(opt_summary.delta_bs, "0.7551");
        assert_eq!(opt_summary.gamma_bs, "0.0001");
        assert_eq!(opt_summary.vega_bs, "0.0923");
        assert_eq!(opt_summary.theta_bs, "-0.0417");
        assert_eq!(opt_summary.lever, "5.2");
        assert_eq!(opt_summary.mark_vol, "0.8321");
        assert_eq!(opt_summary.bid_vol, "0.8201");
        assert_eq!(opt_summary.ask_vol, "0.8441");
        assert_eq!(opt_summary.real_vol, "0.8100");
        assert_eq!(opt_summary.vol_lv, "0.8300");
        assert_eq!(opt_summary.fwd_px, "45123.34");
        assert_eq!(opt_summary.ts, "1597026383085");
    }

    #[test]
    fn test_get_opt_summary_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instType": "OPTION",
                    "instId": "BTC-USD-200103-5500-C",
                    "uly": "BTC-USD",
                    "delta": "0.7551",
                    "gamma": "0.0001",
                    "vega": "0.0923",
                    "theta": "-0.0417",
                    "deltaBS": "0.7551",
                    "gammaBS": "0.0001",
                    "vegaBS": "0.0923",
                    "thetaBS": "-0.0417",
                    "lever": "5.2",
                    "markVol": "0.8321",
                    "bidVol": "0.8201",
                    "askVol": "0.8441",
                    "realVol": "0.8100",
                    "volLv": "0.8300",
                    "fwdPx": "45123.34",
                    "ts": "1597026383085"
                }
            ]
        });

        let response: OkxApiResponse<OptSummary> = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.msg, "");
        assert_eq!(response.data.len(), 1);
        assert_eq!(
            response.data.first().unwrap().inst_id,
            "BTC-USD-200103-5500-C"
        );
    }

    #[test]
    fn test_opt_summary_serialization_roundtrip() {
        let original = GetOptSummaryRequest {
            underlying: Some("BTC-USD".to_string()),
            inst_family: Some("BTC-USD".to_string()),
            exp_time: Some("220325".to_string()),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: GetOptSummaryRequest = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.underlying, deserialized.underlying);
        assert_eq!(original.inst_family, deserialized.inst_family);
        assert_eq!(original.exp_time, deserialized.exp_time);
    }

    #[test]
    fn test_multiple_opt_summaries_response() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instType": "OPTION",
                    "instId": "BTC-USD-200103-5500-C",
                    "uly": "BTC-USD",
                    "delta": "0.7551",
                    "gamma": "0.0001",
                    "vega": "0.0923",
                    "theta": "-0.0417",
                    "deltaBS": "0.7551",
                    "gammaBS": "0.0001",
                    "vegaBS": "0.0923",
                    "thetaBS": "-0.0417",
                    "lever": "5.2",
                    "markVol": "0.8321",
                    "bidVol": "0.8201",
                    "askVol": "0.8441",
                    "realVol": "0.8100",
                    "volLv": "0.8300",
                    "fwdPx": "45123.34",
                    "ts": "1597026383085"
                },
                {
                    "instType": "OPTION",
                    "instId": "BTC-USD-200103-6000-P",
                    "uly": "BTC-USD",
                    "delta": "-0.2449",
                    "gamma": "0.0001",
                    "vega": "0.0923",
                    "theta": "-0.0417",
                    "deltaBS": "-0.2449",
                    "gammaBS": "0.0001",
                    "vegaBS": "0.0923",
                    "thetaBS": "-0.0417",
                    "lever": "2.1",
                    "markVol": "0.8521",
                    "bidVol": "0.8401",
                    "askVol": "0.8641",
                    "realVol": "0.8300",
                    "volLv": "0.8500",
                    "fwdPx": "45123.34",
                    "ts": "1597026383086"
                }
            ]
        });

        let response: OkxApiResponse<OptSummary> = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.data.len(), 2);
        assert_eq!(
            response.data.first().unwrap().inst_id,
            "BTC-USD-200103-5500-C"
        );
        assert_eq!(
            response.data.get(1).unwrap().inst_id,
            "BTC-USD-200103-6000-P"
        );
        assert_eq!(response.data.first().unwrap().delta, "0.7551");
        assert_eq!(response.data.get(1).unwrap().delta, "-0.2449");
    }
}
