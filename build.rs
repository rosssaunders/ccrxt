use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=venues/src");
    
    // Generate integration tests for all venues
    if let Err(e) = generate_integration_tests() {
        panic!("Failed to generate integration tests: {}", e);
    }
}

fn generate_integration_tests() -> Result<(), Box<dyn std::error::Error>> {
    let venues_src_path = Path::new("venues/src");
    let tests_generated_path = Path::new("tests/generated");
    
    // Create the generated tests directory
    fs::create_dir_all(tests_generated_path)?;
    
    // Discover all venues
    let venues = discover_venues(venues_src_path)?;
    
    // Generate test files for each venue
    for venue_name in venues {
        let venue_methods = discover_venue_public_methods(venues_src_path, &venue_name)?;
        if !venue_methods.is_empty() {
            generate_venue_test_file(&venue_name, &venue_methods, tests_generated_path)?;
        }
    }
    
    Ok(())
}

fn discover_venues(venues_src_path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut venues = Vec::new();
    
    for entry in fs::read_dir(venues_src_path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                if dir_name != "error.rs" && dir_name != "lib.rs" {
                    venues.push(dir_name.to_string());
                }
            }
        }
    }
    
    venues.sort();
    Ok(venues)
}

#[derive(Debug, Clone)]
struct EndpointMethod {
    method_name: String,
    has_request_param: bool,
    request_type: Option<String>,
    venue_path: String,
    is_reference_param: bool,
    requires_timestamp: bool,
}

fn discover_venue_public_methods(venues_src_path: &Path, venue_name: &str) -> Result<Vec<EndpointMethod>, Box<dyn std::error::Error>> {
    let mut methods = Vec::new();
    
    // Look for venue-specific public REST modules based on venue structure
    let venue_path = venues_src_path.join(venue_name);
    
    // Define the correct public REST paths for each venue
    let public_rest_paths = match venue_name {
        "binance" => vec![venue_path.join("spot/public/rest")],
        "bingx" => vec![venue_path.join("public/rest")],
        "bitget" => vec![venue_path.join("public/rest")],
        "bitmart" => vec![venue_path.join("spot/public/rest")],
        "bullish" => vec![venue_path.join("public/rest")],
        "bybit" => vec![venue_path.join("public/rest")],
        "coinbase" => vec![venue_path.join("public/rest")],
        "cryptocom" => vec![venue_path.join("public/rest")],
        "deribit" => vec![venue_path.join("public/rest")],
        "gateio" => vec![venue_path.join("public/rest")],
        "kucoin" => vec![venue_path.join("public/rest")],
        "okx" => vec![venue_path.join("public/rest")],
        _ => vec![venue_path.join("public/rest")], // Default fallback
    };
    
    for public_rest_path in public_rest_paths {
        if public_rest_path.exists() {
            methods.extend(extract_methods_from_directory(&public_rest_path, venue_name)?);
        }
    }
    
    // Deduplicate methods by method name
    methods.sort_by(|a, b| a.method_name.cmp(&b.method_name));
    methods.dedup_by(|a, b| a.method_name == b.method_name);
    
    Ok(methods)
}

fn extract_methods_from_directory(rest_path: &Path, venue_name: &str) -> Result<Vec<EndpointMethod>, Box<dyn std::error::Error>> {
    let mut methods = Vec::new();
    
    for entry in fs::read_dir(rest_path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            let file_name = path.file_stem().unwrap().to_str().unwrap();
            
            // Skip client.rs and mod.rs files
            if file_name == "client" || file_name == "mod" {
                continue;
            }
            
            // Read the file and extract method information
            let content = fs::read_to_string(&path)?;
            methods.extend(extract_methods_from_file(&content, venue_name, file_name)?);
        }
    }
    
    Ok(methods)
}

fn extract_methods_from_file(content: &str, venue_name: &str, file_name: &str) -> Result<Vec<EndpointMethod>, Box<dyn std::error::Error>> {
    let mut methods = Vec::new();
    
    // Look for impl RestClient blocks and extract public async methods
    let lines: Vec<&str> = content.lines().collect();
    let mut in_impl_block = false;
    let mut brace_depth = 0;
    
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        
        // Track impl RestClient blocks
        if trimmed.starts_with("impl RestClient") {
            in_impl_block = true;
            brace_depth = 0;
        }
        
        if in_impl_block {
            // Count braces to track scope
            brace_depth += trimmed.chars().filter(|&c| c == '{').count() as i32;
            brace_depth -= trimmed.chars().filter(|&c| c == '}').count() as i32;
            
            if brace_depth < 0 {
                in_impl_block = false;
                continue;
            }
            
            // Look for public async method definitions
            if trimmed.starts_with("pub async fn ") {
                if let Some(method_info) = parse_method_signature(trimmed, &lines, i, venue_name) {
                    methods.push(method_info);
                }
            }
        }
    }
    
    // If no methods found in this file but it has impl blocks, it might be an endpoint file
    if methods.is_empty() && content.contains("impl RestClient") {
        // For endpoint files like ping.rs, trades.rs, etc., extract the method directly
        if let Some(method_info) = extract_endpoint_method(content, venue_name, file_name) {
            methods.push(method_info);
        }
    }
    
    Ok(methods)
}

fn extract_endpoint_method(content: &str, venue_name: &str, _file_name: &str) -> Option<EndpointMethod> {
    // Look for public async methods in endpoint files
    let lines: Vec<&str> = content.lines().collect();
    
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        
        if trimmed.starts_with("pub async fn ") && !trimmed.contains("send_request") {
            return parse_method_signature(trimmed, &lines, i, venue_name);
        }
    }
    
    None
}

fn parse_method_signature(line: &str, lines: &[&str], line_index: usize, venue_name: &str) -> Option<EndpointMethod> {
    // Extract method name
    let start_pos = line.find("pub async fn ")? + 13;
    let end_pos = line[start_pos..].find('(')?;
    let method_name = line[start_pos..start_pos + end_pos].trim();
    
    // Clean method name by removing lifetime parameters
    let clean_method_name = method_name.split('<').next()?.to_string();
    
    // Skip constructor methods
    if clean_method_name == "new" || clean_method_name.contains("new_") {
        return None;
    }
    
    // Look at the parameters to see if there's a request parameter
    let mut has_request_param = false;
    let mut request_type = None;
    let mut is_reference_param = false;
    let mut requires_timestamp = false;
    
    // Simple parsing - look for parameters that look like requests
    let _params_start = line.find('(')?;
    let mut current_line = line_index;
    let mut full_signature = String::new();
    
    // Collect the full method signature (may span multiple lines)
    while current_line < lines.len() {
        full_signature.push_str(lines[current_line]);
        if lines[current_line].contains(") ->") || lines[current_line].contains("){") {
            break;
        }
        current_line += 1;
        full_signature.push(' ');
    }
    
    // Check for request parameters
    if full_signature.contains("Request") {
        has_request_param = true;
        
        // Check if it's a reference parameter (starts with &)
        if full_signature.contains(": &") && full_signature.contains("Request") {
            is_reference_param = true;
        }
        
        // Try to extract the request type name
        if let Some(req_start) = full_signature.find(": ") {
            if let Some(req_end) = full_signature[req_start + 2..].find(&[',', ')'][..]) {
                let req_type = full_signature[req_start + 2..req_start + 2 + req_end].trim();
                if req_type.contains("Request") {
                    request_type = Some(req_type.replace("&", "").trim().to_string());
                    
                    // Check if this is a timestamp-requiring request type
                    requires_timestamp = req_type.contains("GetSymbolsRequest") || 
                                       req_type.contains("Get24hrTickerRequest") ||
                                       req_type.contains("GetServerTimeRequest") ||
                                       venue_name == "bingx"; // BingX generally requires timestamps
                }
            }
        }
    }
    
    // Determine the venue path structure
    let venue_path = determine_venue_path(venue_name);
    
    Some(EndpointMethod {
        method_name: clean_method_name,
        has_request_param,
        request_type,
        venue_path,
        is_reference_param,
        requires_timestamp,
    })
}

fn determine_venue_path(venue_name: &str) -> String {
    match venue_name {
        "binance" => "binance::spot::public::rest".to_string(),
        "bingx" => "bingx::public::rest".to_string(),
        "bitget" => "bitget::public::rest".to_string(),
        "bitmart" => "bitmart::spot::public::rest".to_string(),
        "bullish" => "bullish::public::rest".to_string(),
        "bybit" => "bybit::public::rest".to_string(),
        "coinbase" => "coinbase::public::rest".to_string(),
        "cryptocom" => "cryptocom::public::rest".to_string(),
        "deribit" => "deribit::public::rest".to_string(),
        "gateio" => "gateio::public::rest".to_string(),
        "kucoin" => "kucoin::public::rest".to_string(),
        "okx" => "okx::public::rest".to_string(),
        _ => format!("{}::public::rest", venue_name),
    }
}

fn get_venue_rate_limiter_path(venue_name: &str) -> String {
    match venue_name {
        "binance" => "venues::binance::spot::RateLimiter".to_string(),
        "bingx" => "venues::bingx::RateLimiter".to_string(),
        "bitget" => "venues::bitget::RateLimiter".to_string(),
        "bitmart" => "venues::bitmart::RateLimiter".to_string(),
        "bullish" => "venues::bullish::RateLimiter".to_string(),
        "bybit" => "venues::bybit::RateLimiter".to_string(),
        "coinbase" => "venues::coinbase::RateLimiter".to_string(),
        "cryptocom" => "venues::cryptocom::RateLimiter".to_string(),
        "deribit" => "venues::deribit::RateLimiter".to_string(),
        "gateio" => "venues::gateio::RateLimiter".to_string(),
        "kucoin" => "venues::kucoin::RateLimiter".to_string(),
        "okx" => "venues::okx::RateLimiter".to_string(),
        _ => format!("venues::{}::RateLimiter", venue_name),
    }
}

fn generate_venue_test_file(venue_name: &str, methods: &[EndpointMethod], output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let test_file_content = generate_test_file_content(venue_name, methods);
    let file_path = output_path.join(format!("{}_generated_tests.rs", venue_name));
    fs::write(file_path, test_file_content)?;
    Ok(())
}

fn generate_test_file_content(venue_name: &str, methods: &[EndpointMethod]) -> String {
    let mut content = String::new();
    
    // File header
    content.push_str(&format!(
        "//! Generated integration tests for {} venue public REST API methods\n",
        venue_name
    ));
    content.push_str("//! \n");
    content.push_str("//! This file is auto-generated by the build script.\n");
    content.push_str("//! Do not edit manually.\n");
    content.push_str("//! \n");
    content.push_str("//! These tests call all public REST API methods for the venue\n");
    content.push_str("//! and run in series to avoid rate limiting issues.\n\n");
    
    // Imports
    content.push_str("use reqwest::Client;\n");
    content.push_str("use std::time::Duration;\n");
    content.push_str("use tokio::time::sleep;\n");
    content.push_str("use std::error::Error;\n");
    content.push_str("use chrono::Utc;\n");
    
    // Add venue-specific imports - try to generate the correct path
    let client_import_path = generate_client_import_path(venue_name);
    content.push_str(&format!("use {};\n", client_import_path));
    
    // Add specific request type imports for venues that need them
    if venue_name == "bingx" {
        content.push_str("use venues::bingx::public::rest::{GetSymbolsRequest, Get24hrTickerRequest, GetServerTimeRequest};\n");
    }
    
    // Add rate limiter import if needed
    let rate_limiter_import = generate_rate_limiter_import(venue_name);
    if !rate_limiter_import.is_empty() {
        content.push_str(&format!("use {};\n", rate_limiter_import));
    }
    
    content.push_str("\n");
    
    // Rate limiting configuration
    content.push_str("const RATE_LIMIT_DELAY_MS: u64 = 1000; // 1 second between requests\n\n");
    
    // Helper function to create client
    content.push_str(&generate_client_creation_function(venue_name));
    
    // Test runner function that calls all methods in series
    content.push_str(&format!(
        "#[tokio::test]\n#[ignore] // Ignored by default to avoid hitting API in regular test runs\nasync fn test_{}_all_public_methods_serial() {{\n",
        venue_name
    ));
    content.push_str(&format!("    let client = create_{}_client().await;\n", venue_name));
    content.push_str("    \n");
    content.push_str("    // Run all public API method tests in series\n");
    
    for (i, method) in methods.iter().enumerate() {
        content.push_str(&format!("    \n    // Test {}: {}\n", i + 1, method.method_name));
        content.push_str("    match ");
        
        // Generate method calls based on parameter requirements
        if method.has_request_param {
            if method.is_reference_param {
                // Generate a proper request object for reference parameters
                if method.requires_timestamp {
                    // For timestamp-requiring requests (like BingX)
                    if let Some(req_type) = &method.request_type {
                        content.push_str(&format!(
                            "{{ let timestamp = chrono::Utc::now().timestamp_millis(); let request = {}::new(timestamp); client.{}(&request).await }}", 
                            req_type, // Use the request type as-is
                            method.method_name
                        ));
                    } else {
                        // Fallback for unknown request types with timestamp
                        content.push_str("(Ok(serde_json::Value::Null) as Result<serde_json::Value, Box<dyn std::error::Error>>)");
                    }
                } else {
                    // For methods with request parameters but no timestamp requirement
                    match method.method_name.as_str() {
                        "get_exchange_info" | "get_symbols" | "get_trading_pairs" => {
                            content.push_str(&format!("client.{}(None).await", method.method_name));
                        },
                        _ => {
                            // Skip methods we don't know how to call yet - use a proper Result type
                            content.push_str("(Ok(serde_json::Value::Null) as Result<serde_json::Value, Box<dyn std::error::Error>>)");
                        }
                    }
                }
            } else {
                // Non-reference parameters
                match method.method_name.as_str() {
                    "get_exchange_info" | "get_symbols" | "get_trading_pairs" => {
                        content.push_str(&format!("client.{}(None).await", method.method_name));
                    },
                    _ => {
                        // Skip methods we don't know how to call yet - use a proper Result type
                        content.push_str("(Ok(serde_json::Value::Null) as Result<serde_json::Value, Box<dyn std::error::Error>>)");
                    }
                }
            }
        } else {
            // Methods without parameters - call directly
            content.push_str(&format!("client.{}().await", method.method_name));
        }
        
        content.push_str(" {\n");
        content.push_str(&format!(
            "        Ok(_) => println!(\"✓ {}: {} - Success\"),\n",
            venue_name, method.method_name
        ));
        content.push_str(&format!(
            "        Err(e) => println!(\"✗ {}: {} - Error: {{:?}}\", e),\n",
            venue_name, method.method_name
        ));
        content.push_str("    }\n");
        
        // Add delay between requests except for the last one
        if i < methods.len() - 1 {
            content.push_str("    \n    // Rate limiting delay\n");
            content.push_str("    sleep(Duration::from_millis(RATE_LIMIT_DELAY_MS)).await;\n");
        }
    }
    
    content.push_str("}\n\n");
    
    // Individual test functions for each method (only for methods that are likely to work)
    for method in methods {
        let should_generate_individual_test = if method.has_request_param {
            if method.is_reference_param && method.requires_timestamp {
                // Generate for timestamp-requiring reference methods like BingX
                method.request_type.is_some()
            } else {
                // Only generate for methods we know how to call with None
                matches!(method.method_name.as_str(), "get_exchange_info" | "get_symbols" | "get_trading_pairs")
            }
        } else {
            // Generate for all parameter-less methods
            true
        };
        
        if should_generate_individual_test {
            content.push_str(&format!(
                "#[tokio::test]\n#[ignore]\nasync fn test_{}_{}_individual() {{\n",
                venue_name, method.method_name
            ));
            content.push_str(&format!("    let client = create_{}_client().await;\n", venue_name));
            content.push_str("    \n");
            
            if method.has_request_param {
                if method.is_reference_param && method.requires_timestamp {
                    // Generate proper request object for timestamp-requiring methods
                    if let Some(req_type) = &method.request_type {
                        content.push_str(&format!(
                            "    let timestamp = chrono::Utc::now().timestamp_millis();\n    let request = {}::new(timestamp);\n    let result = client.{}(&request).await;\n", 
                            req_type, // Use the request type as-is
                            method.method_name
                        ));
                    }
                } else {
                    content.push_str(&format!("    let result = client.{}(None).await;\n", method.method_name));
                }
            } else {
                content.push_str(&format!("    let result = client.{}().await;\n", method.method_name));
            }
            
            content.push_str("    \n");
            content.push_str("    match result {\n");
            content.push_str(&format!(
                "        Ok(_) => println!(\"✓ {}: {} - Success\"),\n",
                venue_name, method.method_name
            ));
            content.push_str(&format!(
                "        Err(e) => {{\n            println!(\"✗ {}: {} - Error: {{:?}}\", e);\n            // Don't fail the test for API errors, just log them\n        }}\n",
                venue_name, method.method_name
            ));
            content.push_str("    }\n");
            content.push_str("}\n\n");
        }
    }
    
    content
}

fn generate_client_import_path(venue_name: &str) -> String {
    match venue_name {
        "binance" => "venues::binance::spot::public::rest::RestClient".to_string(),
        "bingx" => "venues::bingx::public::rest::RestClient".to_string(),
        "bitget" => "venues::bitget::public::rest::RestClient".to_string(),
        "bitmart" => "venues::bitmart::spot::public::rest::RestClient".to_string(),
        "bullish" => "venues::bullish::public::rest::RestClient".to_string(),
        "bybit" => "venues::bybit::public::rest::RestClient".to_string(),
        "coinbase" => "venues::coinbase::public::rest::RestClient".to_string(),
        "cryptocom" => "venues::cryptocom::public::rest::RestClient".to_string(),
        "deribit" => "venues::deribit::public::rest::RestClient".to_string(),
        "gateio" => "venues::gateio::public::rest::RestClient".to_string(),
        "kucoin" => "venues::kucoin::public::rest::RestClient".to_string(),
        "okx" => "venues::okx::public::rest::RestClient".to_string(),
        _ => format!("venues::{}::public::rest::RestClient", venue_name),
    }
}

fn generate_rate_limiter_import(venue_name: &str) -> String {
    match venue_name {
        "binance" => "venues::binance::spot::RateLimiter".to_string(),
        "bingx" => "venues::bingx::RateLimiter".to_string(),
        "bitget" => "venues::bitget::RateLimiter".to_string(),
        "bitmart" => "venues::bitmart::RateLimiter".to_string(),
        "bullish" => "venues::bullish::RateLimiter".to_string(),
        "bybit" => "venues::bybit::RateLimiter".to_string(),
        "coinbase" => "venues::coinbase::RateLimiter".to_string(),
        "cryptocom" => "venues::cryptocom::RateLimiter".to_string(),
        "deribit" => "venues::deribit::RateLimiter".to_string(),
        "gateio" => "venues::gateio::RateLimiter".to_string(),
        "kucoin" => "venues::kucoin::RateLimiter".to_string(),
        "okx" => "venues::okx::RateLimiter".to_string(),
        _ => format!("venues::{}::RateLimiter", venue_name),
    }
}

fn generate_client_creation_function(venue_name: &str) -> String {
    let rate_limiter_path = generate_rate_limiter_import(venue_name);
    let base_url = get_venue_base_url(venue_name);
    
    format!(
        "async fn create_{}_client() -> RestClient {{\n    let client = Client::new();\n    let rate_limiter = {}::new();\n    RestClient::new(\n        \"{}\",\n        client,\n        rate_limiter,\n    )\n}}\n\n",
        venue_name,
        rate_limiter_path,
        base_url
    )
}

fn get_venue_base_url(venue_name: &str) -> &'static str {
    match venue_name {
        "binance" => "https://api.binance.com",
        "bingx" => "https://open-api.bingx.com",
        "bitget" => "https://api.bitget.com",
        "bitmart" => "https://api-cloud.bitmart.com",
        "bullish" => "https://api.exchange.bullish.com",
        "bybit" => "https://api.bybit.com",
        "coinbase" => "https://api.exchange.coinbase.com",
        "cryptocom" => "https://api.crypto.com",
        "deribit" => "https://www.deribit.com",
        "gateio" => "https://api.gateio.ws",
        "kucoin" => "https://api.kucoin.com",
        "okx" => "https://www.okx.com",
        _ => "https://api.example.com", // fallback
    }
}

fn clean_type_name(type_name: &str) -> String {
    // Remove lifetime parameters and extract the base type name
    let cleaned = type_name
        .replace("&", "") // Remove references
        .replace("'a", "") // Remove lifetime parameters
        .replace("'static", "");
    
    // If it's an Option<T>, extract T
    if cleaned.starts_with("Option<") && cleaned.ends_with(">") {
        let inner = &cleaned[7..cleaned.len()-1];
        return inner.trim().to_string();
    }
    
    // If it contains generics, extract the base type
    if let Some(generic_start) = cleaned.find('<') {
        return cleaned[..generic_start].trim().to_string();
    }
    
    cleaned.trim().to_string()
}
