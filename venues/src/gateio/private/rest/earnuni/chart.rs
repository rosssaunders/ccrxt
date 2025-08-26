use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const CHART_ENDPOINT: &str = "/earn/uni/chart";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChartPoint {
    /// timestamp in ms
    pub timestamp: i64,

    /// daily APY as a string decimal
    pub apy: String,

    /// optional label or note
    #[serde(default)]
    pub note: Option<String>,
}

impl RestClient {
    /// GET /earn/uni/chart
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#uniloan-currency-annualized-trend-chart)
    pub async fn get_earnuni_chart(&self) -> RestResult<Vec<ChartPoint>> {
        self.send_get_request(CHART_ENDPOINT, Option::<&()>::None)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chart_point_deserializes() {
        let json = r#"[{"timestamp":1673247054000,"apy":"0.01","note":"sample"}]"#;
        let v: Vec<ChartPoint> = serde_json::from_str(json).expect("deserialize");
        assert_eq!(v[0].apy, "0.01");
        assert_eq!(v[0].note.as_deref(), Some("sample"));
    }
}
