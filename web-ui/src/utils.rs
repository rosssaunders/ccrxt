use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use crate::types::{ApiCredentials, VenueType};
use std::collections::HashMap;

const CREDENTIALS_STORAGE_KEY: &str = "ccrxt_api_credentials";

pub fn save_credentials(venue: &VenueType, credentials: &ApiCredentials) -> Result<(), String> {
    let mut all_credentials: HashMap<String, ApiCredentials> = 
        load_all_credentials().unwrap_or_default();
    
    let venue_key = format!("{:?}", venue);
    all_credentials.insert(venue_key, credentials.clone());
    
    LocalStorage::set(CREDENTIALS_STORAGE_KEY, &all_credentials)
        .map_err(|e| format!("Failed to save credentials: {}", e))
}

pub fn load_credentials(venue: &VenueType) -> Option<ApiCredentials> {
    let all_credentials = load_all_credentials().ok()?;
    let venue_key = format!("{:?}", venue);
    all_credentials.get(&venue_key).cloned()
}

pub fn load_all_credentials() -> Result<HashMap<String, ApiCredentials>, String> {
    LocalStorage::get(CREDENTIALS_STORAGE_KEY)
        .map_err(|e| format!("Failed to load credentials: {}", e))
        .or_else(|_| Ok(HashMap::new()))
}

pub fn delete_credentials(venue: &VenueType) -> Result<(), String> {
    let mut all_credentials: HashMap<String, ApiCredentials> = 
        load_all_credentials().unwrap_or_default();
    
    let venue_key = format!("{:?}", venue);
    all_credentials.remove(&venue_key);
    
    LocalStorage::set(CREDENTIALS_STORAGE_KEY, &all_credentials)
        .map_err(|e| format!("Failed to delete credentials: {}", e))
}

pub fn format_json(json_str: &str) -> String {
    match serde_json::from_str::<serde_json::Value>(json_str) {
        Ok(value) => serde_json::to_string_pretty(&value).unwrap_or_else(|_| json_str.to_string()),
        Err(_) => json_str.to_string(),
    }
}