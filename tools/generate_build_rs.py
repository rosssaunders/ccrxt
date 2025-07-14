#!/usr/bin/env python3
"""
Clean build script generator for PyO3 bindings.
This script will generate a clean build.rs file.
"""

build_rs_content = '''use std::fs;
use std::path::Path;
use std::collections::HashMap;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{File, Item, ItemStruct, ItemEnum, ItemImpl, Type, TypePath, Visibility, ImplItem};

fn main() {
    println!("cargo:rerun-if-changed=venues/src");
    
    let src_dir = Path::new("venues/src");
    let output_dir = Path::new("python-bindings/src/generated");
    
    // Create output directory
    fs::create_dir_all(output_dir).unwrap();
    
    // Process all Rust files and collect bindings by venue
    let mut venue_bindings: HashMap<String, Vec<TokenStream>> = HashMap::new();
    collect_bindings(src_dir, &mut venue_bindings);
    
    // Generate module files for each venue
    for (venue_name, bindings) in venue_bindings.iter() {
        if !bindings.is_empty() {
            let output_file = output_dir.join(format!("{}.rs", venue_name));
            let generated_code = generate_venue_module(venue_name, bindings.clone());
            fs::write(output_file, generated_code.to_string()).unwrap();
            println!("Generated bindings for venue: {}", venue_name);
        }
    }
    
    // Generate main module file
    let venue_names: Vec<&String> = venue_bindings.keys().collect();
    generate_main_module(&output_dir, &venue_names);
}

fn collect_bindings(src_dir: &Path, venue_bindings: &mut HashMap<String, Vec<TokenStream>>) {
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

fn process_file(file_path: &Path, venue_bindings: &mut HashMap<String, Vec<TokenStream>>) {
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
            Item::Struct(s) => {
                if should_expose_struct(&s) {
                    bindings.push(generate_struct_binding(s));
                }
            }
            Item::Enum(e) => {
                if should_expose_enum(&e) {
                    bindings.push(generate_enum_binding(e));
                }
            }
            Item::Impl(i) => {
                if should_expose_impl(&i) {
                    bindings.push(generate_impl_binding(i));
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

fn generate_struct_binding(s: ItemStruct) -> TokenStream {
    let name = &s.ident;
    let struct_name_str = name.to_string();
    
    match &s.fields {
        syn::Fields::Named(fields_named) => {
            let field_bindings: Vec<_> = fields_named.named.iter()
                .filter(|f| matches!(f.vis, Visibility::Public(_)))
                .map(|f| {
                    let field_name = &f.ident;
                    let field_type = &f.ty;
                    let python_field_type = convert_rust_type_to_python_type(field_type);
                    
                    quote! {
                        #[getter]
                        fn #field_name(&self) -> PyResult<#python_field_type> {
                            Ok(self.inner.#field_name.clone().into())
                        }
                    }
                })
                .collect();
            
            let constructor = if struct_name_str.contains("Request") {
                quote! {
                    #[new]
                    fn new() -> Self {
                        Self { inner: Default::default() }
                    }
                }
            } else {
                quote! {
                    #[staticmethod]
                    fn from_inner(inner: #name) -> Self {
                        Self { inner }
                    }
                }
            };
            
            quote! {
                #[pyclass]
                #[derive(Clone)]
                pub struct #name {
                    inner: venues::#name,
                }
                
                #[pymethods]
                impl #name {
                    #constructor
                    
                    #(#field_bindings)*
                }
            }
        }
        _ => {
            quote! {
                #[pyclass]
                #[derive(Clone)]
                pub struct #name {
                    inner: venues::#name,
                }
                
                #[pymethods]
                impl #name {
                    #[staticmethod]
                    fn from_inner(inner: #name) -> Self {
                        Self { inner }
                    }
                }
            }
        }
    }
}

fn generate_enum_binding(e: ItemEnum) -> TokenStream {
    let name = &e.ident;
    let variants: Vec<_> = e.variants.iter().map(|v| &v.ident).collect();
    
    quote! {
        #[pyclass]
        #[derive(Clone)]
        pub enum #name {
            #(#variants,)*
        }
    }
}

fn generate_impl_binding(i: ItemImpl) -> TokenStream {
    if let Type::Path(TypePath { path, .. }) = &*i.self_ty {
        if let Some(segment) = path.segments.last() {
            let type_name = &segment.ident;
            
            let methods: Vec<_> = i.items.iter()
                .filter_map(|item| {
                    if let ImplItem::Fn(method) = item {
                        if matches!(method.vis, Visibility::Public(_)) {
                            let method_name = &method.sig.ident;
                            let method_name_str = method_name.to_string();
                            
                            // Skip constructor methods
                            if method_name_str == "new" {
                                return None;
                            }
                            
                            // Generate async wrapper for async methods
                            if method.sig.asyncness.is_some() {
                                Some(quote! {
                                    fn #method_name<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
                                        let client = self.inner.clone();
                                        pyo3_asyncio::tokio::future_into_py(py, async move {
                                            client.#method_name().await
                                        })
                                    }
                                })
                            } else {
                                Some(quote! {
                                    fn #method_name(&self) -> PyResult<()> {
                                        self.inner.#method_name();
                                        Ok(())
                                    }
                                })
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
                quote! {
                    #[pymethods]
                    impl #type_name {
                        #(#methods)*
                    }
                }
            } else {
                quote! {}
            }
        } else {
            quote! {}
        }
    } else {
        quote! {}
    }
}

fn convert_rust_type_to_python_type(rust_type: &Type) -> TokenStream {
    match rust_type {
        Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                let type_name = segment.ident.to_string();
                match type_name.as_str() {
                    "String" => quote! { String },
                    "u64" | "u32" | "u16" | "u8" => quote! { u64 },
                    "i64" | "i32" | "i16" | "i8" => quote! { i64 },
                    "f64" | "f32" => quote! { f64 },
                    "bool" => quote! { bool },
                    "Decimal" => quote! { String },
                    "Option" => quote! { Option<PyObject> },
                    "Vec" => quote! { Vec<PyObject> },
                    _ => quote! { PyObject },
                }
            } else {
                quote! { PyObject }
            }
        }
        _ => quote! { PyObject },
    }
}

fn generate_venue_module(venue_name: &str, bindings: Vec<TokenStream>) -> TokenStream {
    quote! {
        //! Python bindings for #venue_name venue
        //! 
        //! This module is automatically generated from the Rust source code.
        //! Do not edit this file directly.
        
        use pyo3::prelude::*;
        use pyo3_asyncio;
        use venues;
        
        #(#bindings)*
        
        /// Create the Python module for #venue_name
        pub fn create_module(py: Python) -> PyResult<&PyModule> {
            let m = PyModule::new(py, #venue_name)?;
            Ok(m)
        }
    }
}

fn generate_main_module(output_dir: &Path, venue_names: &[&String]) {
    let module_imports: Vec<_> = venue_names.iter().map(|name| {
        let module_name = syn::Ident::new(name, proc_macro2::Span::call_site());
        quote! {
            pub mod #module_name;
        }
    }).collect();
    
    let main_module = quote! {
        //! Automatically generated Python bindings for CCRXT venues
        
        use pyo3::prelude::*;
        
        #(#module_imports)*
    };
    
    let main_file = output_dir.join("mod.rs");
    fs::write(main_file, main_module.to_string()).unwrap();
}
'''

# Write the build.rs file
with open('build.rs', 'w') as f:
    f.write(build_rs_content)

print("Generated clean build.rs file")
