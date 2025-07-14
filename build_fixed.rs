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
    let bindings = venue_bindings.entry(venue_name).or_insert_with(Vec::new);
    
    // Process each item in the file
    for item in ast.items {
        match item {
            syn::Item::Struct(s) => {
                if should_expose_struct(&s) {
                    bindings.push(generate_struct_binding(s));
                }
            }
            syn::Item::Enum(e) => {
                if should_expose_enum(&e) {
                    bindings.push(generate_enum_binding(e));
                }
            }
            syn::Item::Impl(i) => {
                if should_expose_impl(&i) {
                    let impl_binding = generate_impl_binding(i);
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
        "Config", "Params", "Result", "Entry", "History", "Stats"
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

fn generate_struct_binding(s: ItemStruct) -> String {
    let name = s.ident.to_string();
    
    match &s.fields {
        syn::Fields::Named(fields_named) => {
            let field_bindings: Vec<String> = fields_named.named.iter()
                .filter(|f| matches!(f.vis, Visibility::Public(_)))
                .map(|f| {
                    let field_name = f.ident.as_ref().unwrap().to_string();
                    let field_type = convert_rust_type_to_python_type(&f.ty);
                    
                    format!(
                        "    #[getter]\n    fn {}(&self) -> PyResult<{}> {{\n        Ok(self.inner.{}.clone().into())\n    }}",
                        field_name, field_type, field_name
                    )
                })
                .collect();
            
            let constructor = if name.contains("Request") {
                "    #[new]\n    fn new() -> Self {\n        Self { inner: Default::default() }\n    }".to_string()
            } else {
                format!(
                    "    #[staticmethod]\n    fn from_inner(inner: {}) -> Self {{\n        Self {{ inner }}\n    }}",
                    name
                )
            };
            
            let methods = if field_bindings.is_empty() {
                constructor
            } else {
                format!("{}\n\n{}", constructor, field_bindings.join("\n\n"))
            };
            
            format!(
                "#[pyclass]\n#[derive(Clone)]\npub struct {} {{\n    inner: venues::{},\n}}\n\n#[pymethods]\nimpl {} {{\n{}\n}}",
                name, name, name, methods
            )
        }
        _ => {
            format!(
                "#[pyclass]\n#[derive(Clone)]\npub struct {} {{\n    inner: venues::{},\n}}\n\n#[pymethods]\nimpl {} {{\n    #[staticmethod]\n    fn from_inner(inner: {}) -> Self {{\n        Self {{ inner }}\n    }}\n}}",
                name, name, name, name
            )
        }
    }
}

fn generate_enum_binding(e: ItemEnum) -> String {
    let name = e.ident.to_string();
    let variants: Vec<String> = e.variants.iter().map(|v| v.ident.to_string()).collect();
    
    format!(
        "#[pyclass]\n#[derive(Clone)]\npub enum {} {{\n    {},\n}}",
        name,
        variants.join(",\n    ")
    )
}

fn generate_impl_binding(i: ItemImpl) -> String {
    if let Type::Path(TypePath { path, .. }) = &*i.self_ty {
        if let Some(segment) = path.segments.last() {
            let type_name = segment.ident.to_string();
            
            let methods: Vec<String> = i.items.iter()
                .filter_map(|item| {
                    if let ImplItem::Fn(method) = item {
                        if matches!(method.vis, Visibility::Public(_)) {
                            let method_name = method.sig.ident.to_string();
                            
                            // Skip constructor methods
                            if method_name == "new" {
                                return None;
                            }
                            
                            // Generate async wrapper for async methods
                            if method.sig.asyncness.is_some() {
                                Some(format!(
                                    "    fn {}<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {{\n        let client = self.inner.clone();\n        pyo3_asyncio::tokio::future_into_py(py, async move {{\n            client.{}().await\n        }})\n    }}",
                                    method_name, method_name
                                ))
                            } else {
                                Some(format!(
                                    "    fn {}(&self) -> PyResult<()> {{\n        self.inner.{}();\n        Ok(())\n    }}",
                                    method_name, method_name
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
                    type_name,
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
    format!(
        "//! Python bindings for {} venue\n//!\n//! This module is automatically generated from the Rust source code.\n//! Do not edit this file directly.\n\nuse pyo3::prelude::*;\nuse pyo3_asyncio;\nuse venues;\n\n{}\n\n/// Create the Python module for {}\npub fn create_module(py: Python) -> PyResult<&PyModule> {{\n    let m = PyModule::new(py, \"{}\")?;\n    Ok(m)\n}}",
        venue_name,
        bindings.join("\n\n"),
        venue_name,
        venue_name
    )
}

fn generate_main_module(output_dir: &Path, venue_names: &[&String]) {
    let module_imports: Vec<String> = venue_names.iter()
        .map(|name| format!("pub mod {};", name))
        .collect();
    
    let main_module = format!(
        "//! Automatically generated Python bindings for CCRXT venues\n\nuse pyo3::prelude::*;\n\n{}",
        module_imports.join("\n")
    );
    
    let main_file = output_dir.join("mod.rs");
    fs::write(main_file, main_module).unwrap();
}
