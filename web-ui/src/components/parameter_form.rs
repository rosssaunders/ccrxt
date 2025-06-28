use leptos::*;
use crate::endpoints::{ApiEndpoint, Parameter, ParameterType};
use std::collections::HashMap;

#[component]
pub fn ParameterForm(
    endpoint: ApiEndpoint,
    on_execute: impl Fn(HashMap<String, String>) + 'static,
) -> impl IntoView {
    let (parameters, set_parameters) = create_signal(HashMap::<String, String>::new());
    let (validation_errors, set_validation_errors) = create_signal(Vec::<String>::new());

    // Clone endpoint data for use in closures
    let endpoint_clone = endpoint.clone();
    let endpoint_clone2 = endpoint.clone();

    // Initialize default values
    create_effect(move |_| {
        let mut defaults = HashMap::new();
        
        // Set default values for regular parameters
        for param in &endpoint_clone.parameters {
            if let Some(default) = &param.default {
                defaults.insert(param.name.clone(), default.clone());
            }
        }
        
        // Set default values for path parameters
        for param in &endpoint_clone.path_parameters {
            if let Some(default) = &param.default {
                defaults.insert(param.name.clone(), default.clone());
            }
        }
        
        set_parameters.set(defaults);
        set_validation_errors.set(Vec::new());
    });

    let update_parameter = move |name: String, value: String| {
        set_parameters.update(|params| {
            if value.is_empty() {
                params.remove(&name);
            } else {
                params.insert(name, value);
            }
        });
    };

    let execute_handler = move |_| {
        // Basic validation
        let mut errors = Vec::new();
        let current_params = parameters.get();
        
        // Check required parameters
        for param in &endpoint_clone2.parameters {
            if param.required && !current_params.contains_key(&param.name) {
                errors.push(format!("Required parameter '{}' is missing", param.name));
            }
        }
        
        // Check required path parameters
        for param in &endpoint_clone2.path_parameters {
            if param.required && !current_params.contains_key(&param.name) {
                errors.push(format!("Required path parameter '{}' is missing", param.name));
            }
        }
        
        if !errors.is_empty() {
            set_validation_errors.set(errors);
            return;
        }
        
        set_validation_errors.set(Vec::new());
        on_execute(current_params);
    };

    // All parameters combined for rendering
    let all_parameters = {
        let mut all_params = endpoint.path_parameters.clone();
        all_params.extend(endpoint.parameters.clone());
        all_params
    };

    view! {
        <div>
            <h3 style="margin-bottom: 12px; color: #374151;">
                {format!("{} Parameters", endpoint.name)}
            </h3>
            
            <div style="margin-bottom: 16px; padding: 8px; background: #f3f4f6; border-radius: 6px; font-size: 12px;">
                <strong>"Method:"</strong> {format!("{:?}", endpoint.method)} <br/>
                <strong>"Path:"</strong> {endpoint.path.clone()} <br/>
                <strong>"Auth:"</strong> {format!("{:?}", endpoint.auth_type)}
            </div>

            {if all_parameters.is_empty() {
                view! {
                    <div style="padding: 12px; background: #f9fafb; border: 1px solid #e5e7eb; border-radius: 6px; color: #6b7280; margin-bottom: 16px;">
                        "This endpoint has no parameters"
                    </div>
                }.into_view()
            } else {
                view! {
                    <div class="parameter-form">
                        <For
                            each=move || all_parameters.clone()
                            key=|param| param.name.clone()
                            children=move |param| {
                                view! {
                                    <ParameterInput 
                                        parameter=param
                                        on_change=update_parameter
                                        parameters=parameters
                                    />
                                }
                            }
                        />
                    </div>
                }.into_view()
            }}

            // Validation Errors
            {move || {
                let errors = validation_errors.get();
                if !errors.is_empty() {
                    view! {
                        <div style="background: #fef2f2; border: 1px solid #fecaca; padding: 8px; border-radius: 6px; margin-bottom: 16px;">
                            <strong style="color: #dc2626;">"Validation Errors:"</strong>
                            <ul style="margin: 4px 0 0 16px;">
                                {errors.into_iter().map(|error| {
                                    view! { <li style="color: #dc2626;">{error}</li> }
                                }).collect::<Vec<_>>()}
                            </ul>
                        </div>
                    }.into_view()
                } else {
                    view! { <div></div> }.into_view()
                }
            }}

            <button 
                on:click=execute_handler
                style="width: 100%; margin-top: 8px;"
            >
                {format!("Execute {} API Call", endpoint.name)}
            </button>
        </div>
    }
}

#[component]
fn ParameterInput(
    parameter: Parameter,
    on_change: impl Fn(String, String) + Copy + 'static,
    parameters: ReadSignal<HashMap<String, String>>,
) -> impl IntoView {
    let param_name = parameter.name.clone();
    let is_path_param = false; // We could enhance this to distinguish path params

    let current_value = move || {
        parameters.with(|params| {
            params.get(&param_name).cloned().unwrap_or_default()
        })
    };

    let param_name_for_callback = parameter.name.clone();
    let on_change_wrapper = move |value: String| {
        on_change(param_name_for_callback.clone(), value);
    };

    view! {
        <div class="form-group">
            <label>
                {parameter.name.clone()}
                {if parameter.required { 
                    " *" 
                } else { 
                    "" 
                }}
                {if is_path_param {
                    " (path)"
                } else {
                    ""
                }}
            </label>
            
            <div style="font-size: 12px; color: #6b7280; margin-bottom: 4px;">
                {parameter.description.clone()}
                {if let Some(example) = &parameter.example {
                    format!(" • Example: {}", example)
                } else {
                    "".to_string()
                }}
            </div>

            {match &parameter.param_type {
                ParameterType::Enum(options) => {
                    view! {
                        <select 
                            on:change=move |ev| {
                                on_change_wrapper(event_target_value(&ev));
                            }
                            prop:value=current_value
                        >
                            <option value="">"Select..."</option>
                            {options.iter().map(|option| {
                                view! {
                                    <option value=option.clone()>{option.clone()}</option>
                                }
                            }).collect::<Vec<_>>()}
                        </select>
                    }.into_view()
                }
                ParameterType::Boolean => {
                    view! {
                        <select 
                            on:change=move |ev| {
                                on_change_wrapper(event_target_value(&ev));
                            }
                            prop:value=current_value
                        >
                            <option value="">"Select..."</option>
                            <option value="true">"true"</option>
                            <option value="false">"false"</option>
                        </select>
                    }.into_view()
                }
                ParameterType::Number | ParameterType::Decimal => {
                    let input_type = if matches!(parameter.param_type, ParameterType::Number) {
                        "number"
                    } else {
                        "text"
                    };
                    
                    view! {
                        <input 
                            type=input_type
                            placeholder={if let Some(example) = &parameter.example {
                                example.clone()
                            } else {
                                format!("Enter {}", parameter.param_type.display_name())
                            }}
                            prop:value=current_value
                            on:input=move |ev| {
                                on_change_wrapper(event_target_value(&ev));
                            }
                            step={if matches!(parameter.param_type, ParameterType::Decimal) {
                                Some("0.00000001")
                            } else {
                                None
                            }}
                            min={parameter.min.map(|m| m.to_string())}
                            max={parameter.max.map(|m| m.to_string())}
                        />
                    }.into_view()
                }
                _ => {
                    view! {
                        <input 
                            type="text"
                            placeholder={if let Some(example) = &parameter.example {
                                example.clone()
                            } else {
                                "Enter value".to_string()
                            }}
                            prop:value=current_value
                            on:input=move |ev| {
                                on_change_wrapper(event_target_value(&ev));
                            }
                        />
                    }.into_view()
                }
            }}
        </div>
    }
}

impl ParameterType {
    fn display_name(&self) -> &'static str {
        match self {
            ParameterType::String => "string",
            ParameterType::Number => "number",
            ParameterType::Boolean => "boolean",
            ParameterType::Decimal => "decimal",
            ParameterType::Enum(_) => "enum",
            ParameterType::Array(_) => "array",
            ParameterType::Object(_) => "object",
        }
    }
}