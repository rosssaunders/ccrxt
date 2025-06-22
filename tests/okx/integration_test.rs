use crate::okx::{GetMarkPriceCandlesHistoryRequest, GetMarkPriceCandlesRequest};

#[test]
fn test_mark_price_candles_types_are_exported() {
    // Test that we can create instances of the new types
    let request = GetMarkPriceCandlesRequest {
        inst_id: "BTC-USD-SWAP".to_string(),
        after: None,
        before: None,
        bar: Some("1H".to_string()),
        limit: Some("100".to_string()),
    };

    assert_eq!(request.inst_id, "BTC-USD-SWAP");
    assert_eq!(request.bar, Some("1H".to_string()));
}

#[test]
fn test_mark_price_candles_history_types_are_exported() {
    // Test that we can create instances of the new history types
    let request = GetMarkPriceCandlesHistoryRequest {
        inst_id: "ETH-USD-SWAP".to_string(),
        after: None,
        before: None,
        bar: Some("1D".to_string()),
        limit: Some("50".to_string()),
    };

    assert_eq!(request.inst_id, "ETH-USD-SWAP");
    assert_eq!(request.bar, Some("1D".to_string()));
}

#[test]
fn test_mark_price_candles_response() {
    let response_json = serde_json::json!([
        "BTC-USD-SWAP",
        "1H",
        "100",
        "other",
        "fields",
        "here"
    ]);
    let response: Vec<String> = serde_json::from_value(response_json).unwrap();
    assert_eq!(response.first().map(|d| d.as_str()), Some("BTC-USD-SWAP"));
}

#[test]
fn test_mark_price_candles_history_response() {
    let history_response_json = serde_json::json!([
        "BTC-USD-SWAP",
        "1H",
        "100",
        "other",
        "fields",
        "here"
    ]);
    let response: Vec<String> = serde_json::from_value(history_response_json).unwrap();
    assert_eq!(response.first().map(|d| d.as_str()), Some("BTC-USD-SWAP"));
}
