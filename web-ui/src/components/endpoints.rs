use leptos::*;
use crate::endpoints::{ApiEndpoints, Venue, EndpointCategory, ApiEndpoint, AuthType};
use crate::components::ParameterForm;
use crate::extension_api;
use std::collections::HashMap;

#[component]
pub fn EndpointPanel() -> impl IntoView {
    let endpoints = ApiEndpoints::new();
    let endpoints_clone = endpoints.clone();
    
    let (selected_venue, set_selected_venue) = create_signal(Venue::Coinbase);
    let (selected_category, set_selected_category) = create_signal(EndpointCategory::MarketData);
    let (selected_endpoint, set_selected_endpoint) = create_signal::<Option<ApiEndpoint>>(None);
    let (response_text, set_response_text) = create_signal(String::new());
    let (is_loading, set_is_loading) = create_signal(false);

    // Get available endpoints based on selected venue and category
    let available_endpoints = create_memo(move |_| {
        endpoints_clone.get_endpoints_by_category(&selected_venue.get(), &selected_category.get())
            .into_iter()
            .cloned()
            .collect::<Vec<_>>()
    });

    // Reset selected endpoint when venue or category changes
    create_effect(move |_| {
        let _venue = selected_venue.get();
        let _category = selected_category.get();
        set_selected_endpoint.set(None);
        set_response_text.set(String::new());
    });

    let execute_endpoint = move |params: HashMap<String, String>| {
        if let Some(endpoint) = selected_endpoint.get() {
            set_is_loading.set(true);
            set_response_text.set("Executing API call...".to_string());
            
            // Extract path parameters from the regular parameters
            let mut path_parameters = HashMap::new();
            let mut query_parameters = HashMap::new();
            
            for (key, value) in params {
                if endpoint.path_parameters.iter().any(|p| p.name == key) {
                    path_parameters.insert(key, value);
                } else {
                    query_parameters.insert(key, value);
                }
            }
            
            // Use the extension API for real calls
            let endpoint_clone = endpoint.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match extension_api::call_exchange_api(&endpoint_clone, query_parameters, path_parameters).await {
                    Ok(response) => {
                        let formatted_response = serde_json::to_string_pretty(&response)
                            .unwrap_or_else(|_| "Error formatting response".to_string());
                        set_response_text.set(formatted_response);
                    }
                    Err(error) => {
                        let error_response = serde_json::json!({
                            "error": error,
                            "endpoint": endpoint_clone.name,
                            "venue": format!("{:?}", endpoint_clone.venue)
                        });
                        let formatted_error = serde_json::to_string_pretty(&error_response)
                            .unwrap_or_else(|_| format!("Error: {}", error));
                        set_response_text.set(formatted_error);
                    }
                }
                set_is_loading.set(false);
            });
        }
    };

    view! {
        <div class="panel">
            <h2>"API Endpoints"</h2>
            <p style="margin-bottom: 16px; color: #6b7280;">
                "Select an exchange, category, and endpoint to test API calls"
            </p>

            // Venue Selection
            <div class="form-group">
                <label>"Exchange"</label>
                <select on:change=move |ev| {
                    let value = event_target_value(&ev);
                    let venue = match value.as_str() {
                        "Coinbase" => Venue::Coinbase,
                        "BinanceSpot" => Venue::BinanceSpot,
                        "BinanceUsdm" => Venue::BinanceUsdm,
                        "BinanceCoinm" => Venue::BinanceCoinm,
                        "Deribit" => Venue::Deribit,
                        "OKX" => Venue::OKX,
                        "CryptoCom" => Venue::CryptoCom,
                        "Bitmart" => Venue::Bitmart,
                        "BingX" => Venue::BingX,
                        _ => Venue::Coinbase,
                    };
                    set_selected_venue.set(venue);
                }>
                    {Venue::all().into_iter().map(|venue| {
                        let venue_key = format!("{:?}", venue);
                        let display_name = venue.display_name();
                        view! {
                            <option value=venue_key.clone()>{display_name}</option>
                        }
                    }).collect::<Vec<_>>()}
                </select>
            </div>

            // Category Selection
            <div class="form-group">
                <label>"Category"</label>
                <select on:change=move |ev| {
                    let value = event_target_value(&ev);
                    let category = match value.as_str() {
                        "MarketData" => EndpointCategory::MarketData,
                        "Trading" => EndpointCategory::Trading,
                        "Account" => EndpointCategory::Account,
                        "Wallet" => EndpointCategory::Wallet,
                        "System" => EndpointCategory::System,
                        _ => EndpointCategory::MarketData,
                    };
                    set_selected_category.set(category);
                }>
                    <option value="MarketData">"Market Data"</option>
                    <option value="Trading">"Trading"</option>
                    <option value="Account">"Account"</option>
                    <option value="Wallet">"Wallet"</option>
                    <option value="System">"System"</option>
                </select>
            </div>

            // Endpoint Selection
            <div class="form-group">
                <label>"Endpoint"</label>
                <select on:change=move |ev| {
                    let value = event_target_value(&ev);
                    if value.is_empty() {
                        set_selected_endpoint.set(None);
                    } else {
                        let endpoint = endpoints.get_endpoint(&selected_venue.get(), &value);
                        set_selected_endpoint.set(endpoint.cloned());
                    }
                }>
                    <option value="">"Select an endpoint..."</option>
                    {move || {
                        available_endpoints.get().into_iter().map(|endpoint| {
                            let auth_badge = match endpoint.auth_type {
                                AuthType::None => "🟢",
                                AuthType::Public => "🟡", 
                                AuthType::Private | AuthType::View => "🔒",
                                AuthType::Trade => "⚠️",
                            };
                            view! {
                                <option value=endpoint.id.clone()>
                                    {format!("{} {}", auth_badge, endpoint.name)}
                                </option>
                            }
                        }).collect::<Vec<_>>()
                    }}
                </select>
            </div>

            // Authentication warning
            {move || {
                if let Some(endpoint) = selected_endpoint.get() {
                    match endpoint.auth_type {
                        AuthType::Private | AuthType::Trade | AuthType::View => {
                            view! {
                                <div style="background: #fef3c7; border: 1px solid #f59e0b; padding: 8px; border-radius: 6px; margin-bottom: 16px;">
                                    <strong>"⚠️ Authentication Required"</strong>
                                    <br/>
                                    "This endpoint requires API credentials. Make sure you've configured them in the credentials panel."
                                </div>
                            }.into_view()
                        },
                        _ => view! { <div></div> }.into_view()
                    }
                } else {
                    view! { <div></div> }.into_view()
                }
            }}

            // Endpoint Documentation
            {move || {
                if let Some(endpoint) = selected_endpoint.get() {
                    view! {
                        <EndpointDocumentation endpoint=endpoint.clone() />
                    }.into_view()
                } else {
                    view! { <div></div> }.into_view()
                }
            }}

            // Parameter Form
            {move || {
                if let Some(endpoint) = selected_endpoint.get() {
                    view! {
                        <ParameterForm 
                            endpoint=endpoint
                            on_execute=execute_endpoint
                        />
                    }.into_view()
                } else {
                    view! {
                        <div style="padding: 20px; text-align: center; color: #6b7280;">
                            "Select an endpoint to see its parameters"
                        </div>
                    }.into_view()
                }
            }}

            // Response Area
            <div class="form-group">
                <label>"Response"</label>
                <div class=move || {
                    if is_loading.get() {
                        "response-area loading"
                    } else {
                        "response-area"
                    }
                }>
                    {move || {
                        let response = response_text.get();
                        if response.is_empty() {
                            "API response will appear here...".to_string()
                        } else {
                            response
                        }
                    }}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn EndpointDocumentation(endpoint: ApiEndpoint) -> impl IntoView {
    // Parse the description to extract different sections
    let (summary, details) = parse_endpoint_description(&endpoint.description);
    
    view! {
        <div class="documentation-panel">
            <div class="endpoint-header">
                <h3 class="endpoint-title">{endpoint.name.clone()}</h3>
                <div class="endpoint-meta">
                    <span class="method-badge" class:get=matches!(endpoint.method, crate::endpoints::HttpMethod::GET)
                          class:post=matches!(endpoint.method, crate::endpoints::HttpMethod::POST)
                          class:put=matches!(endpoint.method, crate::endpoints::HttpMethod::PUT)
                          class:delete=matches!(endpoint.method, crate::endpoints::HttpMethod::DELETE)>
                        {format!("{:?}", endpoint.method)}
                    </span>
                    <code class="endpoint-path">{endpoint.path.clone()}</code>
                    <span class="auth-badge" class:public=matches!(endpoint.auth_type, AuthType::None)
                          class:private=!matches!(endpoint.auth_type, AuthType::None)>
                        {match endpoint.auth_type {
                            AuthType::None => "Public",
                            AuthType::Public => "Public",
                            AuthType::Private => "Private",
                            AuthType::View => "View Only",
                            AuthType::Trade => "Trading",
                        }}
                    </span>
                </div>
            </div>
            
            <div class="documentation-content">
                <div class="description-section">
                    <h4>"Description"</h4>
                    <p class="summary">{summary}</p>
                    {if !details.is_empty() {
                        view! {
                            <div class="details">
                                {details.into_iter().map(|detail| {
                                    view! { <p>{detail}</p> }
                                }).collect::<Vec<_>>()}
                            </div>
                        }.into_view()
                    } else {
                        view! { <div></div> }.into_view()
                    }}
                </div>
                
                {if !endpoint.path_parameters.is_empty() {
                    view! {
                        <div class="parameters-section">
                            <h4>"Path Parameters"</h4>
                            <div class="parameter-list">
                                {endpoint.path_parameters.iter().map(|param| {
                                    view! {
                                        <div class="parameter-item">
                                            <code class="param-name">{param.name.clone()}</code>
                                            <span class="param-type">{format!("{:?}", param.param_type)}</span>
                                            {if param.required {
                                                view! { <span class="required">"required"</span> }.into_view()
                                            } else {
                                                view! { <span class="optional">"optional"</span> }.into_view()
                                            }}
                                            {if !param.description.is_empty() {
                                                view! {
                                                    <p class="param-description">{param.description.clone()}</p>
                                                }.into_view()
                                            } else {
                                                view! { <div></div> }.into_view()
                                            }}
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
                    }.into_view()
                } else {
                    view! { <div></div> }.into_view()
                }}
                
                {if !endpoint.parameters.is_empty() {
                    view! {
                        <div class="parameters-section">
                            <h4>"Query Parameters"</h4>
                            <div class="parameter-list">
                                {endpoint.parameters.iter().map(|param| {
                                    view! {
                                        <div class="parameter-item">
                                            <code class="param-name">{param.name.clone()}</code>
                                            <span class="param-type">{format!("{:?}", param.param_type)}</span>
                                            {if param.required {
                                                view! { <span class="required">"required"</span> }.into_view()
                                            } else {
                                                view! { <span class="optional">"optional"</span> }.into_view()
                                            }}
                                            {if !param.description.is_empty() {
                                                view! {
                                                    <p class="param-description">{param.description.clone()}</p>
                                                }.into_view()
                                            } else {
                                                view! { <div></div> }.into_view()
                                            }}
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
                    }.into_view()
                } else {
                    view! { <div></div> }.into_view()
                }}
            </div>
        </div>
    }
}

fn parse_endpoint_description(description: &str) -> (String, Vec<String>) {
    if description.is_empty() {
        return ("No description available".to_string(), vec![]);
    }
    
    // Split by common patterns and clean up the description
    let parts: Vec<&str> = description.split(&['.', '#', '\n', '\\'])
        .map(|s| s.trim())
        .filter(|s| !s.is_empty() && !s.starts_with("Arguments") && !s.starts_with("Returns") && !s.starts_with("Example"))
        .collect();
    
    if parts.is_empty() {
        return (description.to_string(), vec![]);
    }
    
    // First sentence as summary
    let summary = parts[0].to_string();
    
    // Rest as details (up to 3 additional sentences)
    let details: Vec<String> = parts.iter()
        .skip(1)
        .take(3)
        .map(|s| s.to_string())
        .filter(|s| s.len() > 10) // Filter out very short fragments
        .collect();
    
    (summary, details)
}