#[cfg(test)]
mod tests {
    #[test]
    fn test_disable_cancel_on_disconnect_integration() {
        // Test disable_cancel_on_disconnect endpoint types are properly exported and accessible
        use venues::deribit::private::rest::{
            DisableCancelOnDisconnectRequest, 
            DisableCancelOnDisconnectResponse,
            CancelOnDisconnectScope
        };
        
        // Test request structure creation with no scope (defaults to connection)
        let request_default = DisableCancelOnDisconnectRequest { scope: None };
        
        // Test request structure creation with connection scope
        let request_connection = DisableCancelOnDisconnectRequest {
            scope: Some(CancelOnDisconnectScope::Connection),
        };
        
        // Test request structure creation with account scope
        let request_account = DisableCancelOnDisconnectRequest {
            scope: Some(CancelOnDisconnectScope::Account),
        };
        
        // Test serialization
        let json_default = serde_json::to_string(&request_default).unwrap();
        let json_connection = serde_json::to_string(&request_connection).unwrap();
        let json_account = serde_json::to_string(&request_account).unwrap();
        
        // Verify serialization results
        assert_eq!(json_default, "{}"); // scope should be omitted when None
        assert!(json_connection.contains("\"scope\":\"connection\""));
        assert!(json_account.contains("\"scope\":\"account\""));
        
        // Test response structure deserialization
        let response_json = serde_json::json!({
            "id": 789,
            "jsonrpc": "2.0",
            "result": "ok"
        });
        
        let response: DisableCancelOnDisconnectResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.id, 789);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result, "ok");
        
        println!("✓ DisableCancelOnDisconnectRequest (default) serialization: {}", json_default);
        println!("✓ DisableCancelOnDisconnectRequest (connection) serialization: {}", json_connection);
        println!("✓ DisableCancelOnDisconnectRequest (account) serialization: {}", json_account);
        println!("✓ DisableCancelOnDisconnectResponse deserialization successful");
        
        assert!(true, "Disable cancel on disconnect endpoint types are properly exported");
    }
}