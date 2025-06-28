use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use syn::{parse_file, Item, ImplItem, FnArg, Pat, Type, ReturnType, ItemStruct};
use regex::Regex;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EndpointInfo {
    venue: String,
    client_type: ClientType,
    method_name: String,
    doc_comment: String,
    parameters: Vec<ParameterInfo>,
    #[allow(dead_code)]
    return_type: String,
    http_method: HttpMethod,
    path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ClientType {
    PublicRest,
    PrivateRest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ParameterInfo {
    name: String,
    type_name: String,
    is_request_struct: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct RequestStructInfo {
    #[allow(dead_code)]
    name: String,
    fields: Vec<FieldInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FieldInfo {
    name: String,
    type_name: String,
    is_optional: bool,
    doc_comment: String,
}

fn main() {
    println!("cargo:rerun-if-changed=../venues/src");
    
    // Get current directory to debug path issues
    let current_dir = std::env::current_dir().unwrap();
    let venues_dir = Path::new("../venues/src");
    
    // Check if venues directory exists
    if !venues_dir.exists() {
        println!("cargo:warning=Venues directory not found at: {:?}", venues_dir);
        
        // Try absolute path as fallback
        let abs_venues_dir = current_dir.parent().unwrap().join("venues/src");
        eprintln!("cargo:warning=Trying absolute path: {:?}", abs_venues_dir);
        if !abs_venues_dir.exists() {
            eprintln!("cargo:warning=Absolute path also doesn't exist");
            // Generate empty file
            let out_path = Path::new("src/generated_endpoints.rs");
            fs::write(&out_path, r#"// AUTO-GENERATED FILE - DO NOT EDIT
// Generated from CCRXT source code at build time

impl ApiEndpoints {
    /// Create endpoints from parsed CCRXT source code
    pub fn create_from_source() -> Self {
        let endpoints = HashMap::new();
        Self { endpoints }
    }
}"#).unwrap();
            return;
        }
    }
    
    let mut all_endpoints = Vec::new();
    let mut request_structs = HashMap::new();
    
    // First pass: collect request structs
    for entry in WalkDir::new(venues_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        let path = entry.path();
        if let Ok(content) = fs::read_to_string(path) {
            let structs = scan_request_structs(&content);
            request_structs.extend(structs);
        }
    }
    
    // Second pass: collect endpoints from individual endpoint files
    let mut endpoint_files_found = 0;
    
    for entry in WalkDir::new(venues_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        let path = entry.path();
        let path_str = path.to_str().unwrap();
        
        // Look for individual endpoint files in rest directories
        // Skip client.rs, mod.rs, and other non-endpoint files
        if path_str.contains("/rest/") && 
           !path_str.ends_with("/client.rs") && 
           !path_str.ends_with("/mod.rs") &&
           !path_str.contains("/shared/") {
            
            if let Some(venue) = extract_venue_name(path) {
                if let Some(client_type) = extract_client_type(path) {
                    if let Ok(content) = fs::read_to_string(path) {
                        endpoint_files_found += 1;
                        let endpoints = parse_endpoint_file(&content, &venue, client_type, path);
                        all_endpoints.extend(endpoints);
                    }
                }
            }
        }
    }
    
    // Generate the endpoint definitions
    let generated_code = generate_endpoint_definitions(&all_endpoints, &request_structs);
    
    // Write to file in src directory (not OUT_DIR for easier access)
    let out_path = Path::new("src/generated_endpoints.rs");
    fs::write(&out_path, generated_code).unwrap();
    
    // Generate JSON file for API documentation
    let json_output = generate_json_documentation(&all_endpoints, &request_structs);
    let json_path = Path::new("endpoints.json");
    fs::write(&json_path, json_output).unwrap();
    
    println!("cargo:warning=Generated {} endpoints", all_endpoints.len());
}

fn extract_venue_name(path: &Path) -> Option<String> {
    let path_str = path.to_str()?;
    
    // Extract venue from path like: venues/src/coinbase/public/rest/client.rs
    if let Some(start) = path_str.find("venues/src/") {
        let after_src = &path_str[start + 11..];
        if let Some(slash) = after_src.find('/') {
            return Some(after_src[..slash].to_string());
        }
    }
    None
}

fn extract_client_type(path: &Path) -> Option<ClientType> {
    let path_str = path.to_str()?;
    
    if path_str.contains("/public/") {
        Some(ClientType::PublicRest)
    } else if path_str.contains("/private/") {
        Some(ClientType::PrivateRest)
    } else {
        None
    }
}

fn parse_endpoint_file(content: &str, venue: &str, client_type: ClientType, file_path: &std::path::Path) -> Vec<EndpointInfo> {
    let mut endpoints = Vec::new();
    
    // Extract endpoint name from file path (e.g., get_products.rs -> get_products)
    let file_name = file_path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");
    
    if let Ok(syntax_tree) = parse_file(content) {
        for item in syntax_tree.items {
            if let Item::Impl(impl_block) = item {
                // Look for impl blocks for RestClient
                if let Type::Path(type_path) = &*impl_block.self_ty {
                    let type_name = type_path.path.segments.last()
                        .map(|s| s.ident.to_string())
                        .unwrap_or_default();
                    
                    let is_rest_client = type_name == "RestClient";
                    
                    if is_rest_client {
                        // Process each method - look for the main endpoint function
                        for impl_item in impl_block.items {
                            if let ImplItem::Fn(method) = impl_item {
                                if matches!(method.vis, syn::Visibility::Public(_)) {
                                    let method_name = method.sig.ident.to_string();
                                    
                                    // Skip constructor and utility methods
                                    if method_name != "new" && method_name != "from" && 
                                       method_name != "with_rate_limiter" {
                                        if let Some(endpoint) = extract_endpoint_info_from_file(
                                            &method, venue, &client_type, content, file_name
                                        ) {
                                            endpoints.push(endpoint);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    endpoints
}

fn extract_endpoint_info_from_file(
    method: &syn::ImplItemFn,
    venue: &str,
    client_type: &ClientType,
    file_content: &str,
    file_name: &str,
) -> Option<EndpointInfo> {
    let method_name = method.sig.ident.to_string();
    
    // Extract doc comment
    let doc_comment = extract_doc_comment(&method.attrs);
    
    // Extract parameters
    let parameters = extract_parameters(&method.sig);
    
    // Extract return type
    let return_type = extract_return_type(&method.sig.output);
    
    // Try to extract HTTP method and path from method body
    let (http_method, path) = extract_http_info_from_file(&method.block, &method_name, venue, file_content);
    
    Some(EndpointInfo {
        venue: venue.to_string(),
        client_type: client_type.clone(),
        method_name,
        doc_comment,
        parameters,
        return_type,
        http_method,
        path,
    })
}

fn extract_doc_comment(attrs: &[syn::Attribute]) -> String {
    let mut comments = Vec::new();
    
    for attr in attrs {
        if attr.path().is_ident("doc") {
            if let syn::Meta::NameValue(meta) = &attr.meta {
                if let syn::Expr::Lit(expr_lit) = &meta.value {
                    if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                        let comment = lit_str.value();
                        let trimmed = comment.trim();
                        if !trimmed.is_empty() {
                            comments.push(trimmed.to_string());
                        }
                    }
                }
            }
        }
    }
    
    comments.join(" ")
}

fn extract_parameters(sig: &syn::Signature) -> Vec<ParameterInfo> {
    let mut parameters = Vec::new();
    
    for arg in &sig.inputs {
        if let FnArg::Typed(pat_type) = arg {
            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                let name = pat_ident.ident.to_string();
                
                // Skip self parameter
                if name == "self" {
                    continue;
                }
                
                let type_name = extract_type_name(&pat_type.ty);
                let is_request_struct = type_name.ends_with("Request") || 
                                       type_name.ends_with("Params") ||
                                       type_name.ends_with("Query");
                
                parameters.push(ParameterInfo {
                    name,
                    type_name,
                    is_request_struct,
                });
            }
        }
    }
    
    parameters
}

fn extract_type_name(ty: &Type) -> String {
    match ty {
        Type::Reference(type_ref) => extract_type_name(&type_ref.elem),
        Type::Path(type_path) => {
            // Handle the full type path to preserve generics like Option<T>
            let segments: Vec<String> = type_path.path.segments.iter().map(|segment| {
                let ident = segment.ident.to_string();
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    let type_args: Vec<String> = args.args.iter().filter_map(|arg| {
                        if let syn::GenericArgument::Type(ty) = arg {
                            Some(extract_type_name(ty))
                        } else {
                            None
                        }
                    }).collect();
                    if !type_args.is_empty() {
                        format!("{}<{}>", ident, type_args.join(", "))
                    } else {
                        ident
                    }
                } else {
                    ident
                }
            }).collect();
            segments.join("::")
        }
        _ => "Unknown".to_string(),
    }
}

fn extract_return_type(output: &ReturnType) -> String {
    match output {
        ReturnType::Default => "()".to_string(),
        ReturnType::Type(_, ty) => extract_type_name(ty),
    }
}

fn extract_http_info_from_file(block: &syn::Block, method_name: &str, venue: &str, file_content: &str) -> (HttpMethod, String) {
    // Convert block to string to search for HTTP method calls
    let block_str = quote::quote! { #block }.to_string();
    
    // Look for direct API paths in send_request calls
    // Pattern: self.send_request("path", Method::GET, ...)
    let send_request_pattern = Regex::new(r#"send_request\s*\(\s*"([^"]*)".*?Method::\s*(\w+)"#).unwrap();
    let send_request_pattern2 = Regex::new(r#"send_request\s*\(\s*"([^"]*)".*?reqwest::Method::\s*(\w+)"#).unwrap();
    
    if let Some(cap) = send_request_pattern.captures(&block_str) {
        let path = cap[1].to_string();
        let method_str = cap[2].to_string();
        let http_method = match method_str.as_str() {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            _ => HttpMethod::GET,
        };
        return (http_method, format!("/{}", path.trim_start_matches('/')));
    }
    
    if let Some(cap) = send_request_pattern2.captures(&block_str) {
        let path = cap[1].to_string();
        let method_str = cap[2].to_string();
        let http_method = match method_str.as_str() {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            _ => HttpMethod::GET,
        };
        return (http_method, format!("/{}", path.trim_start_matches('/')));
    }
    
    // Also search the entire file content for API path constants or literals
    let api_path_pattern = Regex::new(r#""/api/[^"]*""#).unwrap();
    if let Some(cap) = api_path_pattern.find(file_content) {
        let path = cap.as_str().trim_matches('"').to_string();
        let http_method = guess_http_method(method_name);
        return (http_method, path);
    }
    
    // Fallback: create realistic paths based on method name and venue
    let http_method = guess_http_method(method_name);
    let path = guess_realistic_path_from_method_name(method_name, venue);
    
    (http_method, path)
}

fn guess_http_method(method_name: &str) -> HttpMethod {
    if method_name.starts_with("get_") || method_name.starts_with("list_") || method_name.starts_with("query_") {
        HttpMethod::GET
    } else if method_name.starts_with("create_") || method_name.starts_with("new_") || method_name.starts_with("place_") {
        HttpMethod::POST
    } else if method_name.starts_with("update_") || method_name.starts_with("modify_") {
        HttpMethod::PUT
    } else if method_name.starts_with("delete_") || method_name.starts_with("cancel_") {
        HttpMethod::DELETE
    } else {
        HttpMethod::POST
    }
}

fn guess_realistic_path_from_method_name(method_name: &str, venue: &str) -> String {
    // More comprehensive and realistic API path mapping based on actual exchange APIs
    match venue {
        "coinbase" => {
            match method_name {
                "get_products" => "/products".to_string(),
                "get_product" => "/products/{product_id}".to_string(),
                "get_product_book" => "/products/{product_id}/book".to_string(),
                "get_product_ticker" => "/products/{product_id}/ticker".to_string(),
                "get_product_trades" => "/products/{product_id}/trades".to_string(),
                "get_product_candles" => "/products/{product_id}/candles".to_string(),
                "get_product_stats" => "/products/{product_id}/stats".to_string(),
                "create_order" => "/orders".to_string(),
                "cancel_order" => "/orders/{order_id}".to_string(),
                "cancel_orders" => "/orders".to_string(),
                "get_orders" => "/orders".to_string(),
                "get_order" => "/orders/{order_id}".to_string(),
                "get_fills" => "/fills".to_string(),
                "get_accounts" => "/accounts".to_string(),
                "get_account" => "/accounts/{account_id}".to_string(),
                "get_account_holds" => "/accounts/{account_id}/holds".to_string(),
                "get_account_ledger" => "/accounts/{account_id}/ledger".to_string(),
                "get_account_transfers" => "/accounts/{account_id}/transfers".to_string(),
                "get_currencies" => "/currencies".to_string(),
                "get_time" => "/time".to_string(),
                _ => format!("/{}", method_name.replace("_", "-")),
            }
        }
        "binance" => {
            match method_name {
                "get_exchange_info" => "/api/v3/exchangeInfo".to_string(),
                "get_order_book" | "get_depth" => "/api/v3/depth".to_string(),
                "get_recent_trades" => "/api/v3/trades".to_string(),
                "get_historical_trades" => "/api/v3/historicalTrades".to_string(),
                "get_agg_trades" => "/api/v3/aggTrades".to_string(),
                "get_klines" => "/api/v3/klines".to_string(),
                "get_avg_price" => "/api/v3/avgPrice".to_string(),
                "get_ticker_24hr" => "/api/v3/ticker/24hr".to_string(),
                "get_ticker_price" => "/api/v3/ticker/price".to_string(),
                "get_ticker_book_ticker" => "/api/v3/ticker/bookTicker".to_string(),
                "new_order" | "create_order" => "/api/v3/order".to_string(),
                "cancel_order" => "/api/v3/order".to_string(),
                "cancel_all_orders" => "/api/v3/openOrders".to_string(),
                "get_order" => "/api/v3/order".to_string(),
                "get_open_orders" => "/api/v3/openOrders".to_string(),
                "get_all_orders" => "/api/v3/allOrders".to_string(),
                "get_account" => "/api/v3/account".to_string(),
                "get_my_trades" => "/api/v3/myTrades".to_string(),
                _ => format!("/api/v3/{}", method_name.replace("get_", "").replace("_", "")),
            }
        }
        "deribit" => {
            match method_name {
                "get_instruments" => "public/get_instruments".to_string(),
                "get_currencies" => "public/get_currencies".to_string(),
                "get_order_book" => "public/get_order_book".to_string(),
                "get_ticker" => "public/ticker".to_string(),
                "get_trades" => "public/get_last_trades_by_instrument".to_string(),
                "get_index" => "public/get_index".to_string(),
                "get_time" => "public/get_time".to_string(),
                "buy" | "sell" => "private/buy".to_string(),
                "cancel" => "private/cancel".to_string(),
                "cancel_all" => "private/cancel_all".to_string(),
                "get_open_orders" => "private/get_open_orders_by_currency".to_string(),
                "get_order_history" => "private/get_order_history_by_currency".to_string(),
                "get_positions" => "private/get_positions".to_string(),
                "get_account_summary" => "private/get_account_summary".to_string(),
                _ => format!("public/{}", method_name),
            }
        }
        "okx" => {
            match method_name {
                "get_instruments" => "/api/v5/public/instruments".to_string(),
                "get_tickers" => "/api/v5/market/tickers".to_string(),
                "get_ticker" => "/api/v5/market/ticker".to_string(),
                "get_index_tickers" => "/api/v5/market/index-tickers".to_string(),
                "get_order_book" => "/api/v5/market/books".to_string(),
                "get_candlesticks" => "/api/v5/market/candles".to_string(),
                "get_trades" => "/api/v5/market/trades".to_string(),
                "place_order" => "/api/v5/trade/order".to_string(),
                "cancel_order" => "/api/v5/trade/cancel-order".to_string(),
                "get_order_details" => "/api/v5/trade/order".to_string(),
                "get_order_list" => "/api/v5/trade/orders-pending".to_string(),
                "get_order_history" => "/api/v5/trade/orders-history".to_string(),
                "get_account_balance" => "/api/v5/account/balance".to_string(),
                "get_positions" => "/api/v5/account/positions".to_string(),
                _ => format!("/api/v5/{}", method_name.replace("_", "-")),
            }
        }
        "cryptocom" => {
            match method_name {
                "get_instruments" => "public/get-instruments".to_string(),
                "get_book" => "public/get-book".to_string(),
                "get_ticker" => "public/get-ticker".to_string(),
                "get_trades" => "public/get-trades".to_string(),
                "get_candlestick" => "public/get-candlestick".to_string(),
                "create_order" => "private/create-order".to_string(),
                "cancel_order" => "private/cancel-order".to_string(),
                "cancel_all_orders" => "private/cancel-all-orders".to_string(),
                "get_order_history" => "private/get-order-history".to_string(),
                "get_open_orders" => "private/get-open-orders".to_string(),
                "get_order_detail" => "private/get-order-detail".to_string(),
                "get_user_trades" => "private/get-trades".to_string(),
                "get_account_summary" => "private/get-account-summary".to_string(),
                _ => format!("public/{}", method_name.replace("_", "-")),
            }
        }
        _ => format!("/{}", method_name.replace("_", "-")),
    }
}

fn scan_request_structs(content: &str) -> HashMap<String, RequestStructInfo> {
    let mut structs = HashMap::new();
    
    if let Ok(syntax_tree) = parse_file(content) {
        for item in syntax_tree.items {
            if let Item::Struct(struct_item) = item {
                let struct_name = struct_item.ident.to_string();
                
                // Check if this is a request struct
                if struct_name.ends_with("Request") || 
                   struct_name.ends_with("Params") || 
                   struct_name.ends_with("Query") {
                    let fields = extract_struct_fields(&struct_item);
                    
                    structs.insert(struct_name.clone(), RequestStructInfo {
                        name: struct_name,
                        fields,
                    });
                }
            }
        }
    }
    
    structs
}

fn extract_struct_fields(struct_item: &ItemStruct) -> Vec<FieldInfo> {
    let mut fields = Vec::new();
    
    if let syn::Fields::Named(named_fields) = &struct_item.fields {
        for field in &named_fields.named {
            if let Some(ident) = &field.ident {
                let field_name = ident.to_string();
                let type_name = extract_type_name(&field.ty);
                let is_optional = type_name.starts_with("Option<");
                let doc_comment = extract_doc_comment(&field.attrs);
                
                fields.push(FieldInfo {
                    name: field_name,
                    type_name,
                    is_optional,
                    doc_comment,
                });
            }
        }
    }
    
    fields
}

fn generate_endpoint_definitions(
    endpoints: &[EndpointInfo],
    request_structs: &HashMap<String, RequestStructInfo>,
) -> String {
    let mut code = String::from(r#"// AUTO-GENERATED FILE - DO NOT EDIT
// Generated from CCRXT source code at build time

impl ApiEndpoints {
    /// Create endpoints from parsed CCRXT source code
    pub fn create_from_source() -> Self {
        let mut endpoints = HashMap::new();
        
"#);
    
    // Group endpoints by venue
    let mut by_venue: HashMap<String, Vec<&EndpointInfo>> = HashMap::new();
    for endpoint in endpoints {
        by_venue.entry(endpoint.venue.clone())
            .or_default()
            .push(endpoint);
    }
    
    // Generate code for each venue
    for (venue, venue_endpoints) in by_venue {
        code.push_str(&format!("        // {} endpoints\n", venue));
        code.push_str(&format!("        endpoints.insert(Venue::{}, vec![\n", 
            venue_to_enum(&venue)));
        
        for endpoint in venue_endpoints {
            code.push_str(&generate_endpoint_code(endpoint, request_structs));
        }
        
        code.push_str("        ]);\n\n");
    }
    
    code.push_str("        Self { endpoints }\n");
    code.push_str("    }\n");
    code.push_str("}\n");
    
    code
}

fn venue_to_enum(venue: &str) -> String {
    match venue {
        "binance" => "BinanceSpot",
        "coinbase" => "Coinbase",
        "deribit" => "Deribit",
        "okx" => "OKX",
        "cryptocom" => "CryptoCom",
        "bitmart" => "Bitmart",
        "bingx" => "BingX",
        "bitget" => "Bitget",
        "bullish" => "Bullish",
        "bybit" => "Bybit",
        _ => venue,
    }.to_string()
}

fn generate_endpoint_code(
    endpoint: &EndpointInfo,
    request_structs: &HashMap<String, RequestStructInfo>,
) -> String {
    let auth_type = match endpoint.client_type {
        ClientType::PublicRest => "AuthType::None",
        ClientType::PrivateRest => "AuthType::Private",
    };
    
    let category = guess_category(&endpoint.method_name);
    
    let mut code = format!(r#"            ApiEndpoint::new(
                "{}_{}_{}".to_string(),
                Venue::{},
                "{}".to_string(),
                HttpMethod::{:?},
                "{}".to_string(),
                EndpointCategory::{},
                {},
                "{}".to_string(),
            )"#,
        endpoint.venue.to_lowercase(),
        match endpoint.client_type {
            ClientType::PublicRest => "public",
            ClientType::PrivateRest => "private",
        },
        endpoint.method_name,
        venue_to_enum(&endpoint.venue),
        endpoint.method_name,
        endpoint.http_method,
        endpoint.path,
        category,
        auth_type,
        endpoint.doc_comment.replace('"', r#"\""#),
    );
    
    // Add parameters
    for param in &endpoint.parameters {
        if param.is_request_struct {
            // Look up the request struct and add its fields as parameters
            if let Some(struct_info) = request_structs.get(&param.type_name) {
                for field in &struct_info.fields {
                    code.push_str(&format!(r#"
            .with_parameter(Parameter {{
                name: "{}".to_string(),
                param_type: {},
                required: {},
                default: None,
                description: "{}".to_string(),
                example: None,
                min: None,
                max: None,
                pattern: None,
            }})"#,
                        field.name,
                        rust_type_to_parameter_type(&field.type_name),
                        !field.is_optional,
                        field.doc_comment.replace('"', r#"\""#),
                    ));
                }
            }
        } else {
            // Path parameter (like product_id)
            code.push_str(&format!(r#"
            .with_path_parameter(Parameter {{
                name: "{}".to_string(),
                param_type: ParameterType::String,
                required: true,
                default: None,
                description: "{}".to_string(),
                example: None,
                min: None,
                max: None,
                pattern: None,
            }})"#,
                param.name,
                param.name.replace('_', " "),
            ));
        }
    }
    
    code.push_str(",\n");
    code
}

fn guess_category(method_name: &str) -> &'static str {
    if method_name.contains("order") || method_name.contains("trade") || 
       method_name.contains("cancel") || method_name.contains("place") {
        "Trading"
    } else if method_name.contains("account") || method_name.contains("balance") ||
              method_name.contains("position") || method_name.contains("wallet") {
        "Account"
    } else if method_name.contains("withdraw") || method_name.contains("deposit") {
        "Wallet"
    } else {
        "MarketData"
    }
}

fn rust_type_to_parameter_type(rust_type: &str) -> String {
    // Handle Option types
    if rust_type.starts_with("Option<") {
        let inner = rust_type.trim_start_matches("Option<").trim_end_matches('>');
        return rust_type_to_parameter_type(inner);
    }
    
    match rust_type {
        "String" | "&str" | "str" => "ParameterType::String",
        "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" | "usize" => "ParameterType::Number",
        "f32" | "f64" | "Decimal" => "ParameterType::Decimal",
        "bool" => "ParameterType::Boolean",
        t if t.starts_with("Vec<") => "ParameterType::Array(Box::new(ParameterType::String))",
        _ => "ParameterType::String",
    }.to_string()
}

#[derive(Serialize, Deserialize)]
struct JsonApiDocumentation {
    version: String,
    generated_at: String,
    venues: HashMap<String, Vec<JsonEndpoint>>,
    total_endpoints: usize,
}

#[derive(Serialize, Deserialize)]
struct JsonEndpoint {
    id: String,
    venue: String,
    name: String,
    method: String,
    path: String,
    category: String,
    auth_type: String,
    description: String,
    parameters: Vec<JsonParameter>,
    path_parameters: Vec<JsonParameter>,
}

#[derive(Serialize, Deserialize)]
struct JsonParameter {
    name: String,
    param_type: String,
    required: bool,
    description: String,
}

fn generate_json_documentation(
    endpoints: &[EndpointInfo],
    request_structs: &HashMap<String, RequestStructInfo>,
) -> String {
    let mut venues = HashMap::new();
    
    // Group endpoints by venue
    for endpoint in endpoints {
        let json_endpoint = convert_endpoint_to_json(endpoint, request_structs);
        venues.entry(endpoint.venue.clone())
            .or_insert_with(Vec::new)
            .push(json_endpoint);
    }
    
    let documentation = JsonApiDocumentation {
        version: "1.0.0".to_string(),
        generated_at: chrono::Utc::now().to_rfc3339(),
        venues,
        total_endpoints: endpoints.len(),
    };
    
    serde_json::to_string_pretty(&documentation).unwrap()
}

fn convert_endpoint_to_json(
    endpoint: &EndpointInfo,
    request_structs: &HashMap<String, RequestStructInfo>,
) -> JsonEndpoint {
    let mut parameters = Vec::new();
    let mut path_parameters = Vec::new();
    
    // Process parameters
    for param in &endpoint.parameters {
        if param.is_request_struct {
            // Look up the request struct and add its fields as parameters
            if let Some(struct_info) = request_structs.get(&param.type_name) {
                for field in &struct_info.fields {
                    parameters.push(JsonParameter {
                        name: field.name.clone(),
                        param_type: convert_rust_type_to_json(&field.type_name),
                        required: !field.is_optional,
                        description: field.doc_comment.clone(),
                    });
                }
            }
        } else {
            // Path parameter
            path_parameters.push(JsonParameter {
                name: param.name.clone(),
                param_type: "string".to_string(),
                required: true,
                description: param.name.replace('_', " "),
            });
        }
    }
    
    JsonEndpoint {
        id: format!("{}_{}_{}",
            endpoint.venue.to_lowercase(),
            match endpoint.client_type {
                ClientType::PublicRest => "public",
                ClientType::PrivateRest => "private",
            },
            endpoint.method_name
        ),
        venue: endpoint.venue.clone(),
        name: endpoint.method_name.clone(),
        method: format!("{:?}", endpoint.http_method),
        path: endpoint.path.clone(),
        category: guess_category(&endpoint.method_name).to_string(),
        auth_type: match endpoint.client_type {
            ClientType::PublicRest => "none".to_string(),
            ClientType::PrivateRest => "private".to_string(),
        },
        description: endpoint.doc_comment.clone(),
        parameters,
        path_parameters,
    }
}

fn convert_rust_type_to_json(rust_type: &str) -> String {
    // Handle Option types
    if rust_type.starts_with("Option<") {
        let inner = rust_type.trim_start_matches("Option<").trim_end_matches('>');
        return convert_rust_type_to_json(inner);
    }
    
    match rust_type {
        "String" | "&str" | "str" => "string",
        "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" | "usize" => "integer",
        "f32" | "f64" | "Decimal" => "number",
        "bool" => "boolean",
        t if t.starts_with("Vec<") => "array",
        _ => "string",
    }.to_string()
}