use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

const GET_ECONOMIC_CALENDAR_ENDPOINT: &str = "api/v5/public/economic-calendar";

/// Request parameters for economic calendar
#[derive(Debug, Clone, Serialize)]
pub struct GetEconomicCalendarRequest {
    /// Country/Region, e.g. "US", "JP", "CN"
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// Importance level of the event
    /// "1": High importance
    /// "2": Medium importance  
    /// "3": Low importance
    #[serde(rename = "importance", skip_serializing_if = "Option::is_none")]
    pub importance: Option<String>,

    /// Return events before this timestamp (Unix timestamp format in milliseconds)
    #[serde(rename = "before", skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Return events after this timestamp (Unix timestamp format in milliseconds)
    #[serde(rename = "after", skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

/// Economic calendar event data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EconomicCalendarEvent {
    /// Country/Region of the event
    #[serde(rename = "region")]
    pub region: String,

    /// Event name
    #[serde(rename = "event")]
    pub event: String,

    /// Scheduled time of the event (Unix timestamp format in milliseconds)
    #[serde(rename = "dateTime")]
    pub date_time: String,

    /// Importance level: "1" (High), "2" (Medium), "3" (Low)
    #[serde(rename = "importance")]
    pub importance: String,

    /// Previous value
    #[serde(rename = "previous")]
    pub previous: Option<String>,

    /// Forecasted value
    #[serde(rename = "forecast")]
    pub forecast: Option<String>,

    /// Actual value (may be empty if not yet released)
    #[serde(rename = "actual")]
    pub actual: Option<String>,

    /// Currency unit
    #[serde(rename = "unit")]
    pub unit: Option<String>,
}

impl RestClient {
    /// Get economic calendar
    ///
    /// Retrieve economic calendar events with market impact information.
    /// This endpoint requires authentication.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#public-data-rest-api-get-economic-calendar)
    ///
    /// Rate limit: 20 requests per 2 seconds
    /// Rate limit rule: User ID
    ///
    /// # Arguments
    /// * `request` - Optional request parameters for filtering events
    ///
    /// # Returns
    /// A list of economic calendar events
    pub async fn get_economic_calendar(
        &self,
        request: Option<GetEconomicCalendarRequest>,
    ) -> RestResult<EconomicCalendarEvent> {
        self.send_get_request(
            GET_ECONOMIC_CALENDAR_ENDPOINT,
            request.as_ref(),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::okx::response::ApiResponse;

    #[test]
    fn test_get_economic_calendar_request_serialization() {
        let request = GetEconomicCalendarRequest {
            region: Some("US".to_string()),
            importance: Some("1".to_string()),
            before: None,
            after: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("region=US"));
        assert!(serialized.contains("importance=1"));
    }

    #[test]
    fn test_economic_calendar_event_deserialization() {
        let event_json = json!({
            "region": "US",
            "event": "Non-Farm Payrolls",
            "dateTime": "1597026383085",
            "importance": "1",
            "previous": "1600",
            "forecast": "1700",
            "actual": "1650",
            "unit": "K"
        });

        let event: EconomicCalendarEvent = serde_json::from_value(event_json).unwrap();
        assert_eq!(event.region, "US");
        assert_eq!(event.event, "Non-Farm Payrolls");
        assert_eq!(event.date_time, "1597026383085");
        assert_eq!(event.importance, "1");
        assert_eq!(event.previous, Some("1600".to_string()));
        assert_eq!(event.forecast, Some("1700".to_string()));
        assert_eq!(event.actual, Some("1650".to_string()));
        assert_eq!(event.unit, Some("K".to_string()));
    }

    #[test]
    fn test_economic_calendar_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "region": "US",
                    "event": "GDP Growth Rate",
                    "dateTime": "1597026383085",
                    "importance": "1",
                    "previous": "2.1",
                    "forecast": "2.3",
                    "actual": null,
                    "unit": "%"
                },
                {
                    "region": "EU",
                    "event": "Inflation Rate",
                    "dateTime": "1597112783085",
                    "importance": "2",
                    "previous": "1.8",
                    "forecast": "1.9",
                    "actual": "1.9",
                    "unit": "%"
                }
            ]
        });

        let response: ApiResponse<EconomicCalendarEvent> =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data[0].region, "US");
        assert_eq!(response.data[0].event, "GDP Growth Rate");
        assert_eq!(response.data[1].region, "EU");
        assert_eq!(response.data[1].actual, Some("1.9".to_string()));
    }

    #[test]
    fn test_request_serialization_minimal() {
        let request = GetEconomicCalendarRequest {
            region: None,
            importance: None,
            before: None,
            after: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_importance_levels() {
        let high_importance = GetEconomicCalendarRequest {
            region: None,
            importance: Some("1".to_string()),
            before: None,
            after: None,
        };

        let medium_importance = GetEconomicCalendarRequest {
            region: None,
            importance: Some("2".to_string()),
            before: None,
            after: None,
        };

        let low_importance = GetEconomicCalendarRequest {
            region: None,
            importance: Some("3".to_string()),
            before: None,
            after: None,
        };

        let high_serialized = serde_urlencoded::to_string(&high_importance).unwrap();
        let medium_serialized = serde_urlencoded::to_string(&medium_importance).unwrap();
        let low_serialized = serde_urlencoded::to_string(&low_importance).unwrap();

        assert!(high_serialized.contains("importance=1"));
        assert!(medium_serialized.contains("importance=2"));
        assert!(low_serialized.contains("importance=3"));
    }
}
