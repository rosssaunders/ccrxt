use super::{RestClient, RestResult};
use serde::Deserialize;

const ETH2_RATE_RECORDS_ENDPOINT: &str = "/earn/staking/eth2/rate_records";

/// Response for a single ETH2 rate record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Eth2RateRecord {
    /// Date Timestamp
    pub date_time: i64,

    /// Date (YYYY-MM-DD)
    pub date: String,

    /// Percentage Rate
    pub rate: String,
}

impl RestClient {
    /// ETH2 historical return rate query endpoint
    ///
    /// Query ETH earnings rate records for the last 31 days.
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#eth2-historical-return-rate-query)
    ///
    /// Rate limit: per venue config
    ///
    /// # Returns
    /// List of rate records
    pub async fn eth2_rate_records(&self) -> RestResult<Vec<Eth2RateRecord>> {
        self.send_get_request::<Vec<Eth2RateRecord>, ()>(ETH2_RATE_RECORDS_ENDPOINT, None)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eth2_rate_record_deserialization() {
        let json = r#"{
            "date_time": 1690348815,
            "date": "2023-07-26",
            "rate": "60.00"
        }"#;
        let record: Eth2RateRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.date, "2023-07-26");
        assert_eq!(record.rate, "60.00");
    }
}
