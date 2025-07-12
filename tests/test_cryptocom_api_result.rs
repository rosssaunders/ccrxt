use serde_json::json;
use venues::cryptocom::GetAnnouncementsResponse;

#[test]
fn test_api_result_structure() {
    // Test that the new ApiResult structure works correctly
    let response_json = json!({
        "code": 0,
        "result": {
            "data": []
        },
        "id": -1
    });

    let response: GetAnnouncementsResponse = serde_json::from_value(response_json).unwrap();
    assert_eq!(response.code, 0);
    assert_eq!(response.id, -1);
    assert!(response.result.data.is_empty());
}

#[test]
fn test_api_result_with_method() {
    // Test that the ApiResult structure works with optional method field
    let response_json = json!({
        "code": 0,
        "result": {
            "data": []
        },
        "id": -1,
        "method": "public/get-announcements"
    });

    let response: GetAnnouncementsResponse = serde_json::from_value(response_json).unwrap();
    assert_eq!(response.code, 0);
    assert_eq!(response.id, -1);
    assert_eq!(response.method, Some("public/get-announcements".to_string()));
    assert!(response.result.data.is_empty());
}
