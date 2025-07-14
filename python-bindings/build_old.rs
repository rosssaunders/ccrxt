use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use syn::{Item, ItemEnum, ItemImpl, ItemStruct, Type, TypePath, Visibility};

fn main() {
    pyo3_build_config::add_extension_module_link_args();

    println!("cargo:rerun-if-changed=../venues/src");

    let src_dir = Path::new("../venues/src");
    let out_dir = env::var("OUT_DIR").unwrap();
    let output_dir = Path::new(&out_dir).join("generated");

    // Create output directory
    fs::create_dir_all(&output_dir).unwrap();

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

    // Extract venue name from path like "venues/src/binance/..." or "../venues/src/binance/..."
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
        "Request",
        "Response",
        "Client",
        "Error",
        "Info",
        "Data",
        "Order",
        "Trade",
        "Account",
        "Balance",
        "Position",
        "Ticker",
        "Kline",
        "Depth",
        "Symbol",
        "Filter",
        "RateLimit",
        "Status",
        "Config",
        "Params",
        "Result",
        "Entry",
        "History",
        "Stats",
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
            return type_name.contains("Client")
                || type_name.contains("Request")
                || type_name.contains("Builder");
        }
    }
    false
}

fn generate_struct_binding(s: ItemStruct) -> TokenStream {
    let name = &s.ident;

    quote! {
        #[pyclass]
        #[derive(Clone)]
        pub struct #name {
            // For now, we'll just create empty structs
            // In a real implementation, we'd wrap the actual venue struct
        }

        #[pymethods]
        impl #name {
            #[new]
            fn new() -> Self {
                Self {}
            }

            fn __str__(&self) -> String {
                format!("{}", stringify!(#name))
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

fn generate_impl_binding(_i: ItemImpl) -> TokenStream {
    // For now, just generate empty impl blocks
    // In a more sophisticated version, we'd generate method wrappers
    quote! {}
}

fn generate_venue_module(venue_name: &str, bindings: Vec<TokenStream>) -> TokenStream {
    quote! {
        //! Python bindings for #venue_name venue
        //!
        //! This module is automatically generated from the Rust source code.
        //! Do not edit this file directly.

        use pyo3::prelude::*;

        #(#bindings)*

        /// Create the Python module for #venue_name
        pub fn create_module(py: Python) -> PyResult<&PyModule> {
            let m = PyModule::new(py, #venue_name)?;
            Ok(m)
        }
    }
}

fn generate_main_module(output_dir: &Path, venue_names: &[&String]) {
    let module_imports: Vec<_> = venue_names
        .iter()
        .map(|name| {
            let module_name = syn::Ident::new(name, proc_macro2::Span::call_site());
            quote! {
                pub mod #module_name;
            }
        })
        .collect();

    let main_module = quote! {
        //! Automatically generated Python bindings for CCRXT venues

        use pyo3::prelude::*;

        #(#module_imports)*

        #[pymodule]
        fn ccrxt(py: Python, m: &PyModule) -> PyResult<()> {
            m.add("__version__", env!("CARGO_PKG_VERSION"))?;
            Ok(())
        }
    };

    let main_file = output_dir.join("mod.rs");
    fs::write(main_file, main_module.to_string()).unwrap();
}
