use super::client::RestClient;
use crate::okx::{DeliveryExerciseType, EndpointType, InstrumentType, RestResult};
use serde::{Deserialize, Serialize};

/// Request parameters for getting delivery/exercise history
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDeliveryExerciseHistoryRequest {
    /// Instrument type (required)
    #[serde(rename = "instType")]
    pub inst_type: InstrumentType,
    /// Underlying, only applicable to FUTURES/OPTION
    /// Either uly or instFamily is required. If both are passed, instFamily will be used.
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,
    /// Instrument family, only applicable to FUTURES/OPTION
    /// Either uly or instFamily is required. If both are passed, instFamily will be used.
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    /// Pagination of data to return records earlier than the requested ts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination of data to return records newer than the requested ts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Number of results per request. The maximum is 100. The default is 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Individual delivery/exercise detail entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryExerciseDetail {
    /// Delivery/exercise contract ID
    #[serde(rename = "insId")]
    pub ins_id: String,
    /// Delivery/exercise price
    pub px: String,
    /// Type (delivery, exercised, expired_otm)
    #[serde(rename = "type")]
    pub delivery_type: DeliveryExerciseType,
}

/// Individual delivery/exercise history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryExerciseHistory {
    /// Delivery/exercise time, Unix timestamp format in milliseconds, e.g. "1597026383085"
    pub ts: String,
    /// Delivery/exercise details
    pub details: Vec<DeliveryExerciseDetail>,
}

/// Response for getting delivery/exercise history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDeliveryExerciseHistoryResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Delivery/exercise history data
    pub data: Vec<DeliveryExerciseHistory>,
}

impl RestClient {
    /// Get delivery/exercise history
    ///
    /// Retrieve delivery records of Futures and exercise records of Options in the last 3 months.
    ///
    /// See: https://www.okx.com/docs-v5/en/#rest-api-public-data-get-delivery-exercise-history
    ///
    /// Rate limit: 40 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The delivery/exercise history request parameters
    ///
    /// # Returns
    /// Response containing the list of delivery/exercise history entries
    pub async fn get_delivery_exercise_history(&self, request: &GetDeliveryExerciseHistoryRequest) -> RestResult<GetDeliveryExerciseHistoryResponse> {
        self.send_request(
            "api/v5/public/delivery-exercise-history",
            reqwest::Method::GET,
            Some(request),
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
    fn test_get_delivery_exercise_history_request_with_underlying() {
        let request = GetDeliveryExerciseHistoryRequest {
            inst_type: InstrumentType::Futures,
            underlying: Some("BTC-USD".to_string()),
            inst_family: None,
            after: None,
            before: None,
            limit: Some("50".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instType").and_then(|v| v.as_str()),
            Some("FUTURES")
        );
        assert_eq!(
            serialized.get("uly").and_then(|v| v.as_str()),
            Some("BTC-USD")
        );
        assert_eq!(serialized.get("limit").and_then(|v| v.as_str()), Some("50"));
        assert!(serialized.get("instFamily").is_none());
        assert!(serialized.get("after").is_none());
        assert!(serialized.get("before").is_none());
    }

    #[test]
    fn test_get_delivery_exercise_history_request_with_inst_family() {
        let request = GetDeliveryExerciseHistoryRequest {
            inst_type: InstrumentType::Option,
            underlying: None,
            inst_family: Some("BTC-USD".to_string()),
            after: None,
            before: None,
            limit: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instType").and_then(|v| v.as_str()),
            Some("OPTION")
        );
        assert_eq!(
            serialized.get("instFamily").and_then(|v| v.as_str()),
            Some("BTC-USD")
        );
        assert!(serialized.get("uly").is_none());
        assert!(serialized.get("after").is_none());
        assert!(serialized.get("before").is_none());
        assert!(serialized.get("limit").is_none());
    }

    #[test]
    fn test_get_delivery_exercise_history_request_with_both_parameters() {
        let request = GetDeliveryExerciseHistoryRequest {
            inst_type: InstrumentType::Futures,
            underlying: Some("ETH-USD".to_string()),
            inst_family: Some("ETH-USD-FAMILY".to_string()),
            after: Some("1597026383085".to_string()),
            before: Some("1597026483085".to_string()),
            limit: Some("100".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instType").and_then(|v| v.as_str()),
            Some("FUTURES")
        );
        assert_eq!(
            serialized.get("uly").and_then(|v| v.as_str()),
            Some("ETH-USD")
        );
        assert_eq!(
            serialized.get("instFamily").and_then(|v| v.as_str()),
            Some("ETH-USD-FAMILY")
        );
        assert_eq!(
            serialized.get("after").and_then(|v| v.as_str()),
            Some("1597026383085")
        );
        assert_eq!(
            serialized.get("before").and_then(|v| v.as_str()),
            Some("1597026483085")
        );
        assert_eq!(
            serialized.get("limit").and_then(|v| v.as_str()),
            Some("100")
        );
    }

    #[test]
    fn test_delivery_exercise_detail_structure() {
        let detail_json = json!({
            "insId": "BTC-USD-230630",
            "px": "30250.5",
            "type": "delivery"
        });

        let detail: DeliveryExerciseDetail = serde_json::from_value(detail_json).unwrap();
        assert_eq!(detail.ins_id, "BTC-USD-230630");
        assert_eq!(detail.px, "30250.5");
        assert_eq!(detail.delivery_type, DeliveryExerciseType::Delivery);
    }

    #[test]
    fn test_delivery_exercise_detail_with_exercised_type() {
        let detail_json = json!({
            "insId": "BTC-USD-230630-C-30000",
            "px": "30250.5",
            "type": "exercised"
        });

        let detail: DeliveryExerciseDetail = serde_json::from_value(detail_json).unwrap();
        assert_eq!(detail.ins_id, "BTC-USD-230630-C-30000");
        assert_eq!(detail.px, "30250.5");
        assert_eq!(detail.delivery_type, DeliveryExerciseType::Exercised);
    }

    #[test]
    fn test_delivery_exercise_detail_with_expired_otm_type() {
        let detail_json = json!({
            "insId": "BTC-USD-230630-P-35000",
            "px": "0",
            "type": "expired_otm"
        });

        let detail: DeliveryExerciseDetail = serde_json::from_value(detail_json).unwrap();
        assert_eq!(detail.ins_id, "BTC-USD-230630-P-35000");
        assert_eq!(detail.px, "0");
        assert_eq!(detail.delivery_type, DeliveryExerciseType::ExpiredOtm);
    }

    #[test]
    fn test_delivery_exercise_history_structure() {
        let history_json = json!({
            "ts": "1597026383085",
            "details": [
                {
                    "insId": "BTC-USD-230630",
                    "px": "30250.5",
                    "type": "delivery"
                },
                {
                    "insId": "ETH-USD-230630",
                    "px": "1890.75",
                    "type": "delivery"
                }
            ]
        });

        let history: DeliveryExerciseHistory = serde_json::from_value(history_json).unwrap();
        assert_eq!(history.ts, "1597026383085");
        assert_eq!(history.details.len(), 2);
        assert_eq!(history.details.first().unwrap().ins_id, "BTC-USD-230630");
        assert_eq!(history.details.first().unwrap().px, "30250.5");
        assert_eq!(
            history.details.first().unwrap().delivery_type,
            DeliveryExerciseType::Delivery
        );
        assert_eq!(history.details.get(1).unwrap().ins_id, "ETH-USD-230630");
        assert_eq!(history.details.get(1).unwrap().px, "1890.75");
        assert_eq!(
            history.details.get(1).unwrap().delivery_type,
            DeliveryExerciseType::Delivery
        );
    }

    #[test]
    fn test_get_delivery_exercise_history_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "ts": "1597026383085",
                    "details": [
                        {
                            "insId": "BTC-USD-230630",
                            "px": "30250.5",
                            "type": "delivery"
                        }
                    ]
                },
                {
                    "ts": "1597026483085",
                    "details": [
                        {
                            "insId": "ETH-USD-230630-C-2000",
                            "px": "1890.75",
                            "type": "exercised"
                        }
                    ]
                }
            ]
        });

        let response: GetDeliveryExerciseHistoryResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.msg, "");
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data.first().unwrap().ts, "1597026383085");
        assert_eq!(response.data.first().unwrap().details.len(), 1);
        assert_eq!(
            response
                .data
                .first()
                .unwrap()
                .details
                .first()
                .unwrap()
                .ins_id,
            "BTC-USD-230630"
        );
        assert_eq!(
            response.data.first().unwrap().details.first().unwrap().px,
            "30250.5"
        );
        assert_eq!(
            response
                .data
                .first()
                .unwrap()
                .details
                .first()
                .unwrap()
                .delivery_type,
            DeliveryExerciseType::Delivery
        );
        assert_eq!(response.data.get(1).unwrap().ts, "1597026483085");
        assert_eq!(
            response
                .data
                .get(1)
                .unwrap()
                .details
                .first()
                .unwrap()
                .ins_id,
            "ETH-USD-230630-C-2000"
        );
        assert_eq!(
            response.data.get(1).unwrap().details.first().unwrap().px,
            "1890.75"
        );
        assert_eq!(
            response
                .data
                .get(1)
                .unwrap()
                .details
                .first()
                .unwrap()
                .delivery_type,
            DeliveryExerciseType::Exercised
        );
    }

    #[test]
    fn test_delivery_exercise_history_serialization_roundtrip() {
        let original = GetDeliveryExerciseHistoryRequest {
            inst_type: InstrumentType::Option,
            underlying: Some("SOL-USD".to_string()),
            inst_family: None,
            after: Some("1597026383085".to_string()),
            before: None,
            limit: Some("25".to_string()),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: GetDeliveryExerciseHistoryRequest = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.inst_type, deserialized.inst_type);
        assert_eq!(original.underlying, deserialized.underlying);
        assert_eq!(original.inst_family, deserialized.inst_family);
        assert_eq!(original.after, deserialized.after);
        assert_eq!(original.before, deserialized.before);
        assert_eq!(original.limit, deserialized.limit);
    }

    #[test]
    fn test_delivery_exercise_history_minimal_request() {
        let request = GetDeliveryExerciseHistoryRequest {
            inst_type: InstrumentType::Futures,
            underlying: Some("BTC-USD".to_string()),
            inst_family: None,
            after: None,
            before: None,
            limit: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instType").and_then(|v| v.as_str()),
            Some("FUTURES")
        );
        assert_eq!(
            serialized.get("uly").and_then(|v| v.as_str()),
            Some("BTC-USD")
        );
        // Optional fields should not be present when None
        assert!(serialized.get("instFamily").is_none());
        assert!(serialized.get("after").is_none());
        assert!(serialized.get("before").is_none());
        assert!(serialized.get("limit").is_none());
    }

    #[test]
    fn test_delivery_exercise_types_serialization() {
        let delivery = DeliveryExerciseType::Delivery;
        let exercised = DeliveryExerciseType::Exercised;
        let expired_otm = DeliveryExerciseType::ExpiredOtm;

        assert_eq!(serde_json::to_value(&delivery).unwrap(), "delivery");
        assert_eq!(serde_json::to_value(&exercised).unwrap(), "exercised");
        assert_eq!(serde_json::to_value(&expired_otm).unwrap(), "expired_otm");
    }
}
