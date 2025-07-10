use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::okx::{EndpointType, RestResult};

/// Request parameters for getting economic calendar data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetEconomicCalendarRequest {
    /// Country, region or entity filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Level of importance filter ("1": low, "2": medium, "3": high)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub importance: Option<String>,
    /// Pagination of data to return records newer than the requested timestamp (Unix timestamp in milliseconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Pagination of data to return records earlier than the requested timestamp (Unix timestamp in milliseconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

/// Individual economic calendar event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EconomicEvent {
    /// Event date and time (Unix timestamp in milliseconds)
    pub date: String,
    /// Country or region
    pub region: String,
    /// Event importance level ("1": low, "2": medium, "3": high)
    pub importance: String,
    /// Event name/title
    pub event: String,
    /// Expected value/forecast
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast: Option<String>,
    /// Previous value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    /// Actual value (if released)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual: Option<String>,
    /// Currency affected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Event unit (e.g., "%", "K", "M")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

/// Response for getting economic calendar data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEconomicCalendarResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Economic calendar event data
    pub data: Vec<EconomicEvent>,
}

impl RestClient {
    /// Get economic calendar data
    ///
    ///  Authentication is required for this endpoint. This endpoint is only supported in production environment.
    ///
    /// Get the macro-economic calendar data within 3 months. Historical data from 3
    /// months ago is only available to users with trading fee tier VIP1 and above.
    ///
    /// See: https://www.okx.com/docs-v5/en/#public-data-rest-api-get-economic-calendar-data
    ///
    /// Rate limit: 1 request per 5 seconds
    /// Rate limit rule: IP
    ///
    /// # Arguments
    /// * `request` - The economic calendar request parameters
    ///
    /// # Returns
    /// Response containing the economic calendar events
    pub async fn get_economic_calendar(
        &self,
        request: Option<GetEconomicCalendarRequest>,
    ) -> RestResult<GetEconomicCalendarResponse> {
        self.send_request(
            "api/v5/public/economic-calendar",
            reqwest::Method::GET,
            request.as_ref(),
            EndpointType::PublicMarketData,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_get_economic_calendar_request_structure() {
        let request = GetEconomicCalendarRequest {
            region: Some("united_states".to_string()),
            importance: Some("3".to_string()),
            before: None,
            after: Some("1640995200000".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("region").and_then(|v| v.as_str()),
            Some("united_states")
        );
        assert_eq!(
            serialized.get("importance").and_then(|v| v.as_str()),
            Some("3")
        );
        assert_eq!(
            serialized.get("after").and_then(|v| v.as_str()),
            Some("1640995200000")
        );
        assert!(serialized.get("before").is_none());
    }

    #[test]
    fn test_economic_event_structure() {
        let event_json = json!({
            "date": "1640995200000",
            "region": "united_states",
            "importance": "3",
            "event": "Non-Farm Payrolls",
            "forecast": "200K",
            "previous": "180K",
            "actual": "210K",
            "currency": "USD",
            "unit": "K"
        });

        let event: EconomicEvent = serde_json::from_value(event_json).unwrap();
        assert_eq!(event.date, "1640995200000");
        assert_eq!(event.region, "united_states");
        assert_eq!(event.importance, "3");
        assert_eq!(event.event, "Non-Farm Payrolls");
        assert_eq!(event.forecast, Some("200K".to_string()));
        assert_eq!(event.previous, Some("180K".to_string()));
        assert_eq!(event.actual, Some("210K".to_string()));
        assert_eq!(event.currency, Some("USD".to_string()));
        assert_eq!(event.unit, Some("K".to_string()));
    }

    #[test]
    fn test_get_economic_calendar_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "date": "1640995200000",
                    "region": "united_states",
                    "importance": "3",
                    "event": "Non-Farm Payrolls",
                    "forecast": "200K",
                    "previous": "180K",
                    "actual": "210K",
                    "currency": "USD",
                    "unit": "K"
                }
            ]
        });

        let response: GetEconomicCalendarResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data.first().unwrap().event, "Non-Farm Payrolls");
    }

    #[test]
    fn test_economic_event_minimal_structure() {
        let event_json = json!({
            "date": "1640995200000",
            "region": "european_union",
            "importance": "2",
            "event": "ECB Interest Rate Decision"
        });

        let event: EconomicEvent = serde_json::from_value(event_json).unwrap();
        assert_eq!(event.date, "1640995200000");
        assert_eq!(event.region, "european_union");
        assert_eq!(event.importance, "2");
        assert_eq!(event.event, "ECB Interest Rate Decision");
        assert_eq!(event.forecast, None);
        assert_eq!(event.previous, None);
        assert_eq!(event.actual, None);
        assert_eq!(event.currency, None);
        assert_eq!(event.unit, None);
    }

    #[test]
    fn test_request_default() {
        let request = GetEconomicCalendarRequest::default();

        let serialized = serde_json::to_value(&request).unwrap();
        // All fields should be omitted when they are None
        assert!(serialized.as_object().unwrap().is_empty());
    }

    #[test]
    fn test_request_serialization_roundtrip() {
        let original = GetEconomicCalendarRequest {
            region: Some("germany".to_string()),
            importance: Some("1".to_string()),
            before: Some("1640995200000".to_string()),
            after: Some("1640908800000".to_string()),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: GetEconomicCalendarRequest = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.region, deserialized.region);
        assert_eq!(original.importance, deserialized.importance);
        assert_eq!(original.before, deserialized.before);
        assert_eq!(original.after, deserialized.after);
    }
}
