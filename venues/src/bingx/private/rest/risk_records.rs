use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

/// Request to get account risk records
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskRecordsRequest {
    /// Start time in milliseconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// End time in milliseconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// Page number (default: 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    /// Page size (default: 10, max: 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

/// Risk record entry
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskRecord {
    /// Record ID
    pub id: String,
    /// Risk event type
    pub risk_type: String,
    /// Symbol involved (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Risk level (LOW, MEDIUM, HIGH, CRITICAL)
    pub risk_level: String,
    /// Risk amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Decimal>,
    /// Asset involved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    /// Risk description
    pub description: String,
    /// Risk status (ACTIVE, RESOLVED, MITIGATED)
    pub status: String,
    /// Risk detection timestamp
    pub detection_time: i64,
    /// Risk resolution timestamp (if resolved)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_time: Option<i64>,
    /// Action taken to mitigate risk
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_taken: Option<String>,
}

/// Response for risk records
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskRecordsResponse {
    /// Success indicator
    pub success: bool,
    /// Risk records data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<RiskRecordsData>,
}

/// Risk records data
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskRecordsData {
    /// List of risk records
    pub records: Vec<RiskRecord>,
    /// Total count
    pub total_count: i32,
    /// Current page
    pub page: i32,
    /// Page size
    pub size: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_risk_records_request_serialization() {
        let request = RiskRecordsRequest {
            start_time: Some(1640995200000),
            end_time: Some(1641081600000),
            page: Some(1),
            size: Some(20),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"startTime\":1640995200000"));
        assert!(json.contains("\"endTime\":1641081600000"));
        assert!(json.contains("\"page\":1"));
        assert!(json.contains("\"size\":20"));
    }

    #[test]
    fn test_empty_request() {
        let request = RiskRecordsRequest {
            start_time: None,
            end_time: None,
            page: None,
            size: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_risk_records_response_deserialization() {
        let json = r#"
        {
            "success": true,
            "data": {
                "records": [
                    {
                        "id": "RISK123456",
                        "riskType": "LIQUIDATION_RISK",
                        "symbol": "BTCUSDT",
                        "riskLevel": "HIGH",
                        "amount": "1000.50",
                        "asset": "USDT",
                        "description": "Position approaching liquidation threshold",
                        "status": "ACTIVE",
                        "detectionTime": 1640995200000,
                        "resolutionTime": null,
                        "actionTaken": null
                    },
                    {
                        "id": "RISK123457",
                        "riskType": "SUSPICIOUS_ACTIVITY",
                        "riskLevel": "MEDIUM",
                        "description": "Unusual trading pattern detected",
                        "status": "RESOLVED",
                        "detectionTime": 1640995000000,
                        "resolutionTime": 1640995300000,
                        "actionTaken": "Account verified, no action needed"
                    }
                ],
                "totalCount": 2,
                "page": 1,
                "size": 10
            }
        }
        "#;

        let response: RiskRecordsResponse = serde_json::from_str(json).unwrap();
        assert!(response.success);
        
        let data = response.data.unwrap();
        assert_eq!(data.records.len(), 2);
        assert_eq!(data.total_count, 2);
        assert_eq!(data.page, 1);
        assert_eq!(data.size, 10);

        let first_record = &data.records[0];
        assert_eq!(first_record.id, "RISK123456");
        assert_eq!(first_record.risk_type, "LIQUIDATION_RISK");
        assert_eq!(first_record.symbol.as_ref().unwrap(), "BTCUSDT");
        assert_eq!(first_record.risk_level, "HIGH");
        assert_eq!(first_record.amount, Some(dec!(1000.50)));
        assert_eq!(first_record.asset.as_ref().unwrap(), "USDT");
        assert_eq!(first_record.status, "ACTIVE");
        assert_eq!(first_record.detection_time, 1640995200000);
        assert!(first_record.resolution_time.is_none());
        assert!(first_record.action_taken.is_none());

        let second_record = &data.records[1];
        assert_eq!(second_record.id, "RISK123457");
        assert_eq!(second_record.risk_type, "SUSPICIOUS_ACTIVITY");
        assert_eq!(second_record.risk_level, "MEDIUM");
        assert!(second_record.symbol.is_none());
        assert!(second_record.amount.is_none());
        assert!(second_record.asset.is_none());
        assert_eq!(second_record.status, "RESOLVED");
        assert_eq!(second_record.resolution_time, Some(1640995300000));
        assert_eq!(second_record.action_taken.as_ref().unwrap(), "Account verified, no action needed");
    }
}
