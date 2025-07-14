use std::fs;
use std::path::Path;
use std::collections::HashMap;
use syn::{ItemStruct, ItemEnum, ItemImpl, Type, TypePath, Visibility, ImplItem};

fn main() {
    println!("cargo:rerun-if-changed=venues/src");
    
    let src_dir = Path::new("venues/src");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let output_dir = Path::new(&out_dir).join("generated");
    
    // Create output directory
    fs::create_dir_all(&output_dir).unwrap();
    
    // Process all Rust files and collect bindings by venue
    let mut venue_bindings: HashMap<String, Vec<String>> = HashMap::new();
    collect_bindings(src_dir, &mut venue_bindings);
    
    // Generate module files for each venue
    for (venue_name, bindings) in venue_bindings.iter() {
        if !bindings.is_empty() {
            let output_file = output_dir.join(format!("{}.rs", venue_name));
            let generated_code = generate_venue_module(venue_name, bindings);
            fs::write(output_file, generated_code).unwrap();
            println!("Generated bindings for venue: {}", venue_name);
        }
    }
    
    // Generate main module file
    let venue_names: Vec<&String> = venue_bindings.keys().collect();
    generate_main_module(&output_dir, &venue_names);
}

fn collect_bindings(src_dir: &Path, venue_bindings: &mut HashMap<String, Vec<String>>) {
    for entry in fs::read_dir(src_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        
        if path.is_dir() {
            collect_bindings(&path, venue_bindings);
        } else if path.extension().map_or(false, |ext| ext == "rs") {
            process_file(&path, venue_bindings);
        }
    }
}

fn process_file(file_path: &Path, venue_bindings: &mut HashMap<String, Vec<String>>) {
    let content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(_) => return, // Skip files that can't be read
    };
    
    // Parse the Rust file
    let ast = match syn::parse_file(&content) {
        Ok(ast) => ast,
        Err(_) => return, // Skip files that can't be parsed
    };
    
    // Extract venue name from file path
    let venue_name = extract_venue_name(file_path);
    let module_path = extract_module_path(file_path);
    let bindings = venue_bindings.entry(venue_name.clone()).or_insert_with(Vec::new);
    
    // Process each item in the file
    for item in ast.items {
        match item {
            syn::Item::Struct(s) => {
                if should_expose_struct(&s) {
                    bindings.push(generate_struct_binding(s, &venue_name, &module_path));
                }
            }
            syn::Item::Enum(e) => {
                if should_expose_enum(&e) {
                    bindings.push(generate_enum_binding(e, &venue_name, &module_path));
                }
            }
            syn::Item::Impl(i) => {
                if should_expose_impl(&i) {
                    let impl_binding = generate_impl_binding(i, &venue_name, &module_path);
                    if !impl_binding.is_empty() {
                        bindings.push(impl_binding);
                    }
                }
            }
            _ => {}
        }
    }
}

fn extract_venue_name(file_path: &Path) -> String {
    let path_str = file_path.to_string_lossy();
    
    // Extract venue name from path like "venues/src/binance/..."
    if let Some(venues_idx) = path_str.find("venues/src/") {
        let after_venues = &path_str[venues_idx + "venues/src/".len()..];
        if let Some(slash_idx) = after_venues.find('/') {
            return after_venues[..slash_idx].to_string();
        }
    }
    
    "unknown".to_string()
}

fn extract_module_path(file_path: &Path) -> String {
    let path_str = file_path.to_string_lossy();
    
    // Extract module path from path like "venues/src/binance/spot/public/rest/client.rs"
    if let Some(venues_idx) = path_str.find("venues/src/") {
        let after_venues = &path_str[venues_idx + "venues/src/".len()..];
        // Remove the file extension and replace slashes with underscores
        let module_path = after_venues
            .strip_suffix(".rs")
            .unwrap_or(after_venues)
            .replace('/', "_");
        return module_path;
    }
    
    "unknown".to_string()
}

fn should_expose_struct(s: &ItemStruct) -> bool {
    // More comprehensive naming convention detection
    let name = s.ident.to_string();
    let is_public = matches!(s.vis, Visibility::Public(_));
    
    if !is_public {
        return false;
    }
    
    // Data structures that should be exposed based on naming conventions
    let expose_patterns = [
        "Request", "Response", "Client", "Error", "Info", "Data",
        "Order", "Trade", "Account", "Balance", "Position", "Ticker",
        "Kline", "Depth", "Symbol", "Filter", "RateLimit", "Status",
        "Config", "Params", "Result", "Entry", "History", "Stats",
        "RestClient", "PublicRestClient", "PrivateRestClient"
    ];
    
    expose_patterns.iter().any(|pattern| name.contains(pattern))
}

fn should_expose_enum(e: &ItemEnum) -> bool {
    let name = e.ident.to_string();
    let is_public = matches!(e.vis, Visibility::Public(_));
    
    if !is_public {
        return false;
    }
    
    // Expose most public enums, but exclude error enums (they need special handling)
    !name.contains("Error")
}

fn should_expose_impl(i: &ItemImpl) -> bool {
    // Expose implementations for client structs and request builders
    if let Type::Path(TypePath { path, .. }) = &*i.self_ty {
        if let Some(segment) = path.segments.last() {
            let type_name = segment.ident.to_string();
            return type_name.contains("Client") || 
                   type_name.contains("Request") ||
                   type_name.contains("Builder");
        }
    }
    false
}

fn generate_struct_binding(s: ItemStruct, venue_name: &str, module_path: &str) -> String {
    let original_name = s.ident.to_string();
    let namespaced_name = create_namespaced_name(venue_name, module_path, &original_name);
    
    // Generate Python field getters/setters for public fields
    let field_bindings: Vec<String> = match &s.fields {
        syn::Fields::Named(fields_named) => {
            fields_named.named.iter()
                .filter(|f| matches!(f.vis, Visibility::Public(_)))
                .map(|f| {
                    let field_name = f.ident.as_ref().unwrap().to_string();
                    let field_type = convert_rust_type_to_python_type(&f.ty);
                    
                    format!(
                        "    #[getter]\n    fn {}(&self) -> PyResult<{}> {{\n        Ok(self.inner.{}.clone().into())\n    }}\n\n    #[setter]\n    fn set_{}(&mut self, value: {}) -> PyResult<()> {{\n        self.inner.{} = value.try_into().map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(\"{{:?}}\", e)))?;\n        Ok(())\n    }}",
                        field_name, field_type, field_name, field_name, field_type, field_name
                    )
                })
                .collect()
        }
        _ => vec![]
    };
    
    // Generate constructor based on struct type
    let constructor = if original_name.contains("RestClient") {
        // Generate specialized constructors for REST clients
        generate_rest_client_constructor(venue_name, module_path, &original_name)
    } else if original_name.contains("Request") {
        "    #[new]\n    fn new() -> Self {\n        Self { inner: Default::default() }\n    }".to_string()
    } else if original_name.contains("Response") {
        format!(
            "    #[staticmethod]\n    fn from_inner(inner: venues::{}) -> Self {{\n        Self {{ inner }}\n    }}",
            original_name
        )
    } else {
        format!(
            "    #[new]\n    fn new() -> Self {{\n        Self {{ inner: venues::{}::default() }}\n    }}\n\n    #[staticmethod]\n    fn from_inner(inner: venues::{}) -> Self {{\n        Self {{ inner }}\n    }}",
            original_name, original_name
        )
    };
    
    // Combine constructor and field bindings
    let methods = if field_bindings.is_empty() {
        constructor
    } else {
        format!("{}\n\n{}", constructor, field_bindings.join("\n\n"))
    };
    
    format!(
        "#[pyclass]\n#[derive(Clone)]\npub struct {} {{\n    inner: venues::{},\n}}\n\n#[pymethods]\nimpl {} {{\n{}\n}}",
        namespaced_name, original_name, namespaced_name, methods
    )
}

fn generate_enum_binding(e: ItemEnum, venue_name: &str, module_path: &str) -> String {
    let original_name = e.ident.to_string();
    let namespaced_name = create_namespaced_name(venue_name, module_path, &original_name);
    let variants: Vec<String> = e.variants.iter().map(|v| v.ident.to_string()).collect();
    
    format!(
        "#[pyclass]\n#[derive(Clone)]\npub enum {} {{\n    {},\n}}",
        namespaced_name,
        variants.join(",\n    ")
    )
}

fn generate_impl_binding(i: ItemImpl, venue_name: &str, module_path: &str) -> String {
    if let Type::Path(TypePath { path, .. }) = &*i.self_ty {
        if let Some(segment) = path.segments.last() {
            let original_type_name = segment.ident.to_string();
            let namespaced_type_name = create_namespaced_name(venue_name, module_path, &original_type_name);
            
            let methods: Vec<String> = i.items.iter()
                .filter_map(|item| {
                    if let ImplItem::Fn(method) = item {
                        if matches!(method.vis, Visibility::Public(_)) {
                            let method_name = method.sig.ident.to_string();
                            
                            // Skip constructor methods
                            if method_name == "new" {
                                return None;
                            }
                            
                            // Extract method parameters (excluding self)
                            let params: Vec<String> = method.sig.inputs.iter()
                                .skip(1) // Skip self parameter
                                .filter_map(|arg| {
                                    if let syn::FnArg::Typed(pat_type) = arg {
                                        if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                                            let param_name = pat_ident.ident.to_string();
                                            let param_type = convert_rust_type_to_python_type(&pat_type.ty);
                                            return Some(format!("{}: {}", param_name, param_type));
                                        }
                                    }
                                    None
                                })
                                .collect();
                            
                            let param_list = params.join(", ");
                            let param_names: Vec<String> = method.sig.inputs.iter()
                                .skip(1) // Skip self parameter
                                .filter_map(|arg| {
                                    if let syn::FnArg::Typed(pat_type) = arg {
                                        if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                                            return Some(pat_ident.ident.to_string());
                                        }
                                    }
                                    None
                                })
                                .collect();
                            let param_call = param_names.join(", ");
                            
                            // Generate async wrapper for async methods
                            if method.sig.asyncness.is_some() {
                                let param_str = if param_list.is_empty() { 
                                    String::new() 
                                } else { 
                                    format!(", {}", param_list) 
                                };
                                
                                Some(format!(
                                    "    fn {}<'py>(&self, py: Python<'py>{}) -> PyResult<&'py PyAny> {{\n        let inner = self.inner.clone();\n        pyo3_asyncio::tokio::future_into_py(py, async move {{\n            let result = inner.{}({}).await;\n            match result {{\n                Ok(response) => Ok(response),\n                Err(e) => Err(format!(\"API Error: {{:?}}\", e)),\n            }}\n        }})\n    }}",
                                    method_name, 
                                    param_str,
                                    method_name, 
                                    param_call
                                ))
                            } else {
                                let param_str = if param_list.is_empty() { 
                                    String::new() 
                                } else { 
                                    format!(", {}", param_list) 
                                };
                                
                                Some(format!(
                                    "    fn {}(&self{}) -> PyResult<PyObject> {{\n        let result = self.inner.{}({});\n        match result {{\n            Ok(response) => Ok(Python::with_gil(|py| response.into_py(py))),\n            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(\"API Error: {{:?}}\", e))),\n        }}\n    }}",
                                    method_name,
                                    param_str,
                                    method_name,
                                    param_call
                                ))
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();
            
            if !methods.is_empty() {
                format!(
                    "#[pymethods]\nimpl {} {{\n{}\n}}",
                    namespaced_type_name,
                    methods.join("\n\n")
                )
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}

fn convert_rust_type_to_python_type(rust_type: &Type) -> String {
    match rust_type {
        Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                let type_name = segment.ident.to_string();
                match type_name.as_str() {
                    "String" => "String".to_string(),
                    "u64" | "u32" | "u16" | "u8" => "u64".to_string(),
                    "i64" | "i32" | "i16" | "i8" => "i64".to_string(),
                    "f64" | "f32" => "f64".to_string(),
                    "bool" => "bool".to_string(),
                    "Decimal" => "String".to_string(),
                    "Option" => "Option<PyObject>".to_string(),
                    "Vec" => "Vec<PyObject>".to_string(),
                    _ => "PyObject".to_string(),
                }
            } else {
                "PyObject".to_string()
            }
        }
        _ => "PyObject".to_string(),
    }
}

fn generate_venue_module(venue_name: &str, bindings: &[String]) -> String {
    // Extract class names from bindings to generate m.add_class calls
    let mut class_names = Vec::new();
    for binding in bindings {
        if let Some(start) = binding.find("pub struct ") {
            let after_struct = &binding[start + 11..];
            if let Some(end) = after_struct.find(" {") {
                let class_name = &after_struct[..end];
                class_names.push(class_name.to_string());
            }
        } else if let Some(start) = binding.find("pub enum ") {
            let after_enum = &binding[start + 9..];
            if let Some(end) = after_enum.find(" {") {
                let class_name = &after_enum[..end];
                class_names.push(class_name.to_string());
            }
        }
    }
    
    let class_registrations: Vec<String> = class_names.iter()
        .map(|name| format!("    m.add_class::<{}>()?;", name))
        .collect();
    
    format!(
        "//! Python bindings for {} venue\n//!\n//! This module is automatically generated from the Rust source code.\n//! Do not edit this file directly.\n\nuse pyo3::prelude::*;\nuse pyo3::types::PyAny;\nuse pyo3_asyncio;\nuse venues;\nuse rest::secrets::{{SecretString, ExposableSecret}};\nuse std::sync::Arc;\n\n// Type conversion helpers\nuse pyo3::{{IntoPy, ToPyObject}};\n\n{}\n\n/// Initialize the Python module for {}\npub fn init_module(py: Python, m: &PyModule) -> PyResult<()> {{\n    // Add all generated classes to the module\n{}\n    Ok(())\n}}",
        venue_name,
        bindings.join("\n\n"),
        venue_name,
        class_registrations.join("\n")
    )
}

fn generate_main_module(output_dir: &Path, venue_names: &[&String]) {
    let module_imports: Vec<String> = venue_names.iter()
        .map(|name| format!("pub mod {};", name))
        .collect();
    
    let main_module = format!(
        "//! Automatically generated Python bindings for CCRXT venues\n//!\n//! This module provides Python bindings for all cryptocurrency exchange venues.\n//! Each venue module contains client implementations and data structures.\n\nuse pyo3::prelude::*;\n\n{}\n\n/// Initialize all venue modules\npub fn init_all_modules(py: Python, parent_module: &PyModule) -> PyResult<()> {{\n    // Initialize each venue module\n    {}\n    Ok(())\n}}",
        module_imports.join("\n"),
        venue_names.iter()
            .map(|name| format!("    {}::init_module(py, parent_module)?;", name))
            .collect::<Vec<_>>()
            .join("\n")
    );
    
    let main_file = output_dir.join("mod.rs");
    fs::write(main_file, main_module).unwrap();
}

fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn create_namespaced_name(venue_name: &str, module_path: &str, original_name: &str) -> String {
    // Create a compact but unique name by combining venue and key module parts
    let venue_part = capitalize_first_letter(venue_name);
    
    // Extract key parts from module path (e.g., "spot_public_rest" -> "SpotPublicRest")
    let module_parts: Vec<&str> = module_path.split('_').collect();
    let module_part = if module_parts.len() > 1 {
        // Skip the venue name (first part) and filename (last part if it's a duplicate)
        let relevant_parts: Vec<&str> = module_parts.iter()
            .skip(1) // Skip venue name
            .filter(|&&part| part != "client" && part != "mod") // Skip common filename parts
            .take(2) // Take at most 2 parts to keep names manageable
            .cloned()
            .collect();
        
        if relevant_parts.is_empty() {
            "".to_string()
        } else {
            relevant_parts.iter()
                .map(|part| capitalize_first_letter(part))
                .collect::<Vec<_>>()
                .join("")
        }
    } else {
        "".to_string()
    };
    
    format!("{}{}{}", venue_part, module_part, original_name)
}

fn generate_rest_client_constructor(venue_name: &str, module_path: &str, _original_name: &str) -> String {
    let is_private = module_path.contains("private");
    let is_public = module_path.contains("public");
    
    if is_private {
        // Private clients need API credentials
        format!(
            "    #[new]\n    fn new(api_key: String, api_secret: String, base_url: String) -> PyResult<Self> {{\n        let api_key = rest::secrets::SecretString::new(api_key);\n        let api_secret = rest::secrets::SecretString::new(api_secret);\n        let client = reqwest::Client::new();\n        let rate_limiter = venues::{}::RateLimiter::new();\n        \n        let inner = venues::{}::RestClient::new(\n            Box::new(api_key),\n            Box::new(api_secret),\n            base_url,\n            rate_limiter,\n            client,\n        );\n        \n        Ok(Self {{ inner }})\n    }}",
            venue_name, venue_name
        )
    } else if is_public {
        // Public clients only need base URL
        format!(
            "    #[new]\n    fn new(base_url: String) -> PyResult<Self> {{\n        let client = reqwest::Client::new();\n        let rate_limiter = venues::{}::RateLimiter::new();\n        \n        let inner = venues::{}::RestClient::new(\n            base_url,\n            client,\n            rate_limiter,\n        );\n        \n        Ok(Self {{ inner }})\n    }}",
            venue_name, venue_name
        )
    } else {
        // Generic client constructor
        format!(
            "    #[new]\n    fn new(base_url: String) -> PyResult<Self> {{\n        let client = reqwest::Client::new();\n        let rate_limiter = venues::{}::RateLimiter::new();\n        \n        let inner = venues::{}::RestClient::new(\n            base_url,\n            client,\n            rate_limiter,\n        );\n        \n        Ok(Self {{ inner }})\n    }}",
            venue_name, venue_name
        )
    }
}
