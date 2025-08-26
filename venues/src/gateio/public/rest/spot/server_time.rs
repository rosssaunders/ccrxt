use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::gateio::RestResult;

const SERVER_TIME_ENDPOINT: &str = "/spot/time";

/// Server time response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerTime {
    /// Current server timestamp in seconds
    pub server_time: i64,
}

impl RestClient {
    /// Get current server time
    ///
    /// This endpoint returns the current server time as a Unix timestamp.
    /// Useful for synchronizing client time with the server.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#get-server-current-time)
    pub async fn get_server_time(&self) -> RestResult<ServerTime> {
        self.get(SERVER_TIME_ENDPOINT).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_time_deserialization() {
        let json = r#"{
            "server_time": 1640995200
        }"#;

        let server_time: ServerTime = serde_json::from_str(json).unwrap();
        assert_eq!(server_time.server_time, 1640995200);
    }

    #[test]
    fn test_server_time_recent_timestamp() {
        let json = r#"{
            "server_time": 1700000000
        }"#;

        let server_time: ServerTime = serde_json::from_str(json).unwrap();
        assert_eq!(server_time.server_time, 1700000000);
        // Verify it's a reasonable timestamp (after 2020)
        assert!(server_time.server_time > 1577836800); // 2020-01-01
    }

    #[test]
    fn test_server_time_zero_timestamp() {
        let json = r#"{
            "server_time": 0
        }"#;

        let server_time: ServerTime = serde_json::from_str(json).unwrap();
        assert_eq!(server_time.server_time, 0);
    }

    #[test]
    fn test_server_time_negative_timestamp() {
        let json = r#"{
            "server_time": -1640995200
        }"#;

        let server_time: ServerTime = serde_json::from_str(json).unwrap();
        assert_eq!(server_time.server_time, -1640995200);
    }

    #[test]
    fn test_server_time_max_timestamp() {
        let json = format!(
            r#"{{
            "server_time": {}
        }}"#,
            i64::MAX
        );

        let server_time: ServerTime = serde_json::from_str(&json).unwrap();
        assert_eq!(server_time.server_time, i64::MAX);
    }

    #[test]
    fn test_server_time_min_timestamp() {
        let json = format!(
            r#"{{
            "server_time": {}
        }}"#,
            i64::MIN
        );

        let server_time: ServerTime = serde_json::from_str(&json).unwrap();
        assert_eq!(server_time.server_time, i64::MIN);
    }

    #[test]
    fn test_server_time_serialization() {
        let server_time = ServerTime {
            server_time: 1640995200,
        };

        let json = serde_json::to_value(&server_time).unwrap();
        assert_eq!(json["server_time"], 1640995200);
    }

    #[test]
    fn test_server_time_serialization_round_trip() {
        let original = ServerTime {
            server_time: 1700000000,
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ServerTime = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.server_time, original.server_time);
    }

    #[test]
    fn test_server_time_realistic_values() {
        let timestamps = vec![
            1640995200, // 2022-01-01
            1672531200, // 2023-01-01
            1704067200, // 2024-01-01
            1735689600, // 2025-01-01
        ];

        for ts in timestamps {
            let json = format!(
                r#"{{
                "server_time": {}
            }}"#,
                ts
            );

            let server_time: ServerTime = serde_json::from_str(&json).unwrap();
            assert_eq!(server_time.server_time, ts);

            // Verify it's in a reasonable range
            assert!(server_time.server_time > 1600000000); // After 2020
            assert!(server_time.server_time < 2000000000); // Before 2033
        }
    }

    #[test]
    fn test_server_time_clone() {
        let original = ServerTime {
            server_time: 1640995200,
        };

        let cloned = original.clone();
        assert_eq!(cloned.server_time, original.server_time);
    }

    #[test]
    fn test_server_time_debug() {
        let server_time = ServerTime {
            server_time: 1640995200,
        };

        let debug_str = format!("{:?}", server_time);
        assert!(debug_str.contains("ServerTime"));
        assert!(debug_str.contains("1640995200"));
    }
}
