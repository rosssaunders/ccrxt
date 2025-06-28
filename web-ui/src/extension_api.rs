use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, js_sys};
use crate::endpoints::{ApiEndpoint, Venue, AuthType, HttpMethod};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["chrome", "runtime"])]
    fn sendMessage(message: &JsValue) -> js_sys::Promise;
    
    #[wasm_bindgen(js_namespace = ["chrome", "runtime"])]
    static id: Option<String>;
}

#[derive(Serialize, Deserialize)]
struct ApiCallRequest {
    #[serde(rename = "type")]
    request_type: String,
    venue: String,
    endpoint: String,
    method: String,
    path: String,
    parameters: HashMap<String, String>,
    #[serde(rename = "pathParameters")]
    path_parameters: HashMap<String, String>,
    #[serde(rename = "authType")]
    auth_type: String,
}

#[derive(Serialize, Deserialize)]
struct CredentialsRequest {
    #[serde(rename = "type")]
    request_type: String,
    venue: String,
    action: String, // "get", "set", "delete"
    credentials: Option<VenueCredentials>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VenueCredentials {
    #[serde(rename = "apiKey")]
    pub api_key: Option<String>,
    #[serde(rename = "apiSecret")]
    pub api_secret: Option<String>,
    pub passphrase: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct ExtensionResponse {
    success: bool,
    data: Option<serde_json::Value>,
    error: Option<String>,
    credentials: Option<VenueCredentials>,
}

pub fn is_extension_context() -> bool {
    // Check if we're running in a Chrome extension context
    id.is_some()
}

pub async fn call_exchange_api(
    endpoint: &ApiEndpoint,
    parameters: HashMap<String, String>,
    path_parameters: HashMap<String, String>,
) -> Result<serde_json::Value, String> {
    if !is_extension_context() {
        return Ok(create_mock_response(endpoint, &parameters));
    }

    let request = ApiCallRequest {
        request_type: "apiCall".to_string(),
        venue: format!("{:?}", endpoint.venue),
        endpoint: endpoint.name.clone(),
        method: format!("{:?}", endpoint.method),
        path: endpoint.path.clone(),
        parameters,
        path_parameters,
        auth_type: match endpoint.auth_type {
            crate::endpoints::AuthType::None => "none".to_string(),
            crate::endpoints::AuthType::Public => "none".to_string(),
            crate::endpoints::AuthType::Private => "private".to_string(),
            crate::endpoints::AuthType::Trade => "trade".to_string(),
            crate::endpoints::AuthType::View => "view".to_string(),
        },
    };

    match send_message_to_extension(&request).await {
        Ok(response) => {
            if response.success {
                response.data.ok_or_else(|| "No data in response".to_string())
            } else {
                Err(response.error.unwrap_or_else(|| "Unknown error".to_string()))
            }
        }
        Err(e) => Err(format!("Extension communication error: {}", e)),
    }
}

pub async fn store_credentials(venue: Venue, credentials: VenueCredentials) -> Result<(), String> {
    if !is_extension_context() {
        return Err("Chrome extension required for secure credential storage".to_string());
    }

    let request = CredentialsRequest {
        request_type: "credentials".to_string(),
        venue: format!("{:?}", venue),
        action: "set".to_string(),
        credentials: Some(credentials),
    };

    match send_message_to_extension(&request).await {
        Ok(response) => {
            if response.success {
                Ok(())
            } else {
                Err(response.error.unwrap_or_else(|| "Failed to store credentials".to_string()))
            }
        }
        Err(e) => Err(format!("Extension communication error: {}", e)),
    }
}

pub async fn get_credentials(venue: Venue) -> Result<Option<VenueCredentials>, String> {
    if !is_extension_context() {
        return Ok(None);
    }

    let request = CredentialsRequest {
        request_type: "credentials".to_string(),
        venue: format!("{:?}", venue),
        action: "get".to_string(),
        credentials: None,
    };

    match send_message_to_extension(&request).await {
        Ok(response) => Ok(response.credentials),
        Err(e) => Err(format!("Extension communication error: {}", e)),
    }
}

pub async fn delete_credentials(venue: Venue) -> Result<(), String> {
    if !is_extension_context() {
        return Ok(());
    }

    let request = CredentialsRequest {
        request_type: "credentials".to_string(),
        venue: format!("{:?}", venue),
        action: "delete".to_string(),
        credentials: None,
    };

    match send_message_to_extension(&request).await {
        Ok(response) => {
            if response.success {
                Ok(())
            } else {
                Err(response.error.unwrap_or_else(|| "Failed to delete credentials".to_string()))
            }
        }
        Err(e) => Err(format!("Extension communication error: {}", e)),
    }
}

async fn send_message_to_extension<T: Serialize>(message: &T) -> Result<ExtensionResponse, String> {
    let js_message = serde_wasm_bindgen::to_value(message)
        .map_err(|e| format!("Serialization error: {}", e))?;
    
    let promise = sendMessage(&js_message);
    let js_response = JsFuture::from(promise)
        .await
        .map_err(|e| format!("Message sending failed: {:?}", e))?;
    
    let response: ExtensionResponse = serde_wasm_bindgen::from_value(js_response)
        .map_err(|e| format!("Response deserialization error: {}", e))?;
    
    Ok(response)
}

// Mock API call for non-extension environments (development)
pub fn create_mock_response(
    endpoint: &ApiEndpoint,
    parameters: &HashMap<String, String>,
) -> serde_json::Value {
    serde_json::json!({
        "status": "success",
        "endpoint": endpoint.name,
        "venue": format!("{:?}", endpoint.venue),
        "method": format!("{:?}", endpoint.method),
        "path": endpoint.path,
        "parameters": parameters,
        "mock_data": {
            "timestamp": "2023-12-01T12:00:00Z",
            "message": "This is a mock response. Install the Chrome extension for real API calls.",
            "note": "Real API functionality requires the Chrome extension environment."
        }
    })
}