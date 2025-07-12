use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bybit::{EndpointType, RestResult, MaintenanceType, NetworkType, ServiceType, Status};

/// Response for system status endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatusResponse {
    /// Response code (0 for success)
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    /// Response message
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    /// Extended response information
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    /// Response data
    pub result: SystemStatusResult,
    /// Response timestamp
    pub time: u64,
}

/// Result data for system status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatusResult {
    /// List of system status entries
    pub list: Vec<SystemStatusEntry>,
}

/// Individual system status entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatusEntry {
    /// Service name/type
    #[serde(rename = "serviceName")]
    pub service_name: String,
    /// Current status of the service
    pub status: Status,
    /// Service type classification
    #[serde(rename = "serviceType")]
    pub service_type: ServiceType,
    /// Network environment (mainnet, testnet, etc.)
    #[serde(rename = "network")]
    pub network: NetworkType,
    /// Maintenance type if applicable
    #[serde(rename = "maintenanceType", skip_serializing_if = "Option::is_none")]
    pub maintenance_type: Option<MaintenanceType>,
    /// Start time for maintenance/incident (Unix timestamp)
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    /// End time for maintenance/incident (Unix timestamp)
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// Description of the status or maintenance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl RestClient {
    /// Get system status
    ///
    /// This endpoint retrieves the current system status for various services.
    /// It provides information about ongoing maintenance, system failures, and
    /// service availability across different networks.
    ///
    /// This endpoint is useful for:
    /// - Checking service availability before attempting operations
    /// - Monitoring planned maintenance windows
    /// - Getting real-time status of trading services
    ///
    /// # Rate Limit
    /// 10 requests per second
    ///
    /// # Returns
    /// A result containing the system status response or an error
    pub async fn get_system_status(&self) -> RestResult<SystemStatusResponse> {
        // System status endpoint typically doesn't require parameters
        self.send_public_request(
            "/v5/status",
            None::<std::collections::HashMap<String, String>>,
            EndpointType::Market,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_status_entry_structure() {
        let entry = SystemStatusEntry {
            service_name: "Trading Service".to_string(),
            status: Status::Ongoing,
            service_type: ServiceType::TradingService,
            network: NetworkType::Mainnet,
            maintenance_type: Some(MaintenanceType::PlannedMaintenance),
            start_time: Some(1672734174000),
            end_time: Some(1672737774000),
            description: Some("Scheduled maintenance for trading engine upgrade".to_string()),
        };
        
        assert_eq!(entry.service_name, "Trading Service");
        assert_eq!(entry.status, Status::Ongoing);
        assert_eq!(entry.service_type, ServiceType::TradingService);
        assert_eq!(entry.network, NetworkType::Mainnet);
        assert_eq!(entry.maintenance_type, Some(MaintenanceType::PlannedMaintenance));
    }

    #[test]
    fn test_system_status_response_structure() {
        let response_json = r#"
        {
            "retCode": 0,
            "retMsg": "OK",
            "retExtInfo": {},
            "result": {
                "list": [
                    {
                        "serviceName": "Spot Trading",
                        "status": "ongoing",
                        "serviceType": "Trading service",
                        "network": "mainnet",
                        "maintenanceType": "Planned maintenance",
                        "startTime": 1672734174000,
                        "endTime": 1672737774000,
                        "description": "Routine maintenance"
                    }
                ]
            },
            "time": 1672734174346
        }
        "#;

        let response: SystemStatusResponse = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.ret_code, 0);
        assert_eq!(response.ret_msg, "OK");
        assert_eq!(response.result.list.len(), 1);
        
        let entry = &response.result.list[0];
        assert_eq!(entry.service_name, "Spot Trading");
        assert_eq!(entry.status, Status::Ongoing);
        assert_eq!(entry.service_type, ServiceType::TradingService);
        assert_eq!(entry.network, NetworkType::Mainnet);
        assert_eq!(entry.maintenance_type, Some(MaintenanceType::PlannedMaintenance));
    }

    #[test]
    fn test_system_status_serialization() {
        let entry = SystemStatusEntry {
            service_name: "WebSocket Service".to_string(),
            status: Status::Completed,
            service_type: ServiceType::WebsocketTradingService,
            network: NetworkType::MainnetDemo,
            maintenance_type: None,
            start_time: None,
            end_time: None,
            description: None,
        };

        let serialized = serde_json::to_string(&entry).unwrap();
        assert!(serialized.contains("\"serviceName\":\"WebSocket Service\""));
        assert!(serialized.contains("\"status\":\"completed\""));
        assert!(serialized.contains("\"serviceType\":\"Websocket trading service\""));
        assert!(serialized.contains("\"network\":\"mainnet demo\""));
        // Optional fields should not be present when None
        assert!(!serialized.contains("maintenanceType"));
        assert!(!serialized.contains("startTime"));
        assert!(!serialized.contains("description"));
    }

    #[test]
    fn test_all_status_types() {
        let statuses = vec![
            Status::Scheduled,
            Status::Ongoing,
            Status::Completed,
            Status::Canceled,
        ];

        for status in statuses {
            let serialized = serde_json::to_string(&status).unwrap();
            assert!(!serialized.is_empty());
        }
    }

    #[test]
    fn test_all_service_types() {
        let services = vec![
            ServiceType::TradingService,
            ServiceType::HttpTradingService,
            ServiceType::WebsocketTradingService,
            ServiceType::DerivativesTradingService,
            ServiceType::SpotTradingService,
            ServiceType::OptionsTradingService,
        ];

        for service in services {
            let serialized = serde_json::to_string(&service).unwrap();
            assert!(!serialized.is_empty());
        }
    }
}