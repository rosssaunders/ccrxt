use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type, parse_macro_input};

#[proc_macro_derive(PrivateRequest)]
pub fn derive_private_request(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    // This will later check for a timestamp field and generate the impl
    let expanded = quote! {
        impl crate::binance::coinm::private::rest::PrivateRequest for #name {
            fn timestamp(&self) -> u64 {
                self.timestamp
            }
        }
    };
    TokenStream::from(expanded)
}

/// Automatically generates PyO3 bindings for request/response structs
#[proc_macro_derive(PyO3Bindings, attributes(pyclass, pymethods))]
pub fn derive_pyo3_bindings(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => {
                let field_getters: Vec<_> = fields
                    .named
                    .iter()
                    .filter(|f| is_public_field(f))
                    .map(|f| {
                        let field_name = &f.ident;
                        let field_type = &f.ty;
                        let python_type = convert_rust_type_to_python(field_type);
                        let setter_name = syn::Ident::new(
                            &format!("set_{}", field_name.as_ref().unwrap()),
                            proc_macro2::Span::call_site(),
                        );

                        quote! {
                            #[getter]
                            fn #field_name(&self) -> PyResult<#python_type> {
                                Ok(self.inner.#field_name.clone().into())
                            }

                            #[setter]
                            fn #setter_name(&mut self, value: #python_type) -> PyResult<()> {
                                self.inner.#field_name = value.into();
                                Ok(())
                            }
                        }
                    })
                    .collect();

                quote! {
                    #[pyclass]
                    #[derive(Clone)]
                    pub struct #name {
                        inner: venues::#name,
                    }

                    #[pymethods]
                    impl #name {
                        #[new]
                        fn new() -> Self {
                            Self {
                                inner: venues::#name::default(),
                            }
                        }

                        #(#field_getters)*
                    }

                    impl From<venues::#name> for #name {
                        fn from(inner: venues::#name) -> Self {
                            Self { inner }
                        }
                    }

                    impl From<#name> for venues::#name {
                        fn from(wrapper: #name) -> Self {
                            wrapper.inner
                        }
                    }
                }
            }
            _ => quote! {
                #[pyclass]
                #[derive(Clone)]
                pub struct #name {
                    inner: venues::#name,
                }

                #[pymethods]
                impl #name {
                    #[new]
                    fn new() -> Self {
                        Self {
                            inner: venues::#name::default(),
                        }
                    }
                }
            },
        },
        Data::Enum(data_enum) => {
            let variants: Vec<_> = data_enum.variants.iter().map(|v| &v.ident).collect();

            quote! {
                #[pyclass]
                #[derive(Clone)]
                pub enum #name {
                    #(#variants,)*
                }

                impl From<venues::#name> for #name {
                    fn from(inner: venues::#name) -> Self {
                        match inner {
                            #(venues::#name::#variants => #name::#variants,)*
                        }
                    }
                }
            }
        }
        _ => quote! {},
    };

    TokenStream::from(expanded)
}

/// Generates PyO3 bindings for client implementations
#[proc_macro_derive(PyO3Client)]
pub fn derive_pyo3_client(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        #[pyclass]
        #[derive(Clone)]
        pub struct #name {
            inner: std::sync::Arc<venues::#name>,
            runtime: std::sync::Arc<tokio::runtime::Runtime>,
        }

        #[pymethods]
        impl #name {
            #[new]
            fn new(base_url: String) -> PyResult<Self> {
                let runtime = tokio::runtime::Runtime::new()
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

                let client = venues::#name::new(
                    base_url,
                    reqwest::Client::new(),
                    venues::RateLimiter::new(),
                );

                Ok(Self {
                    inner: std::sync::Arc::new(client),
                    runtime: std::sync::Arc::new(runtime),
                })
            }

            // Methods will be added by a separate macro or code generation
        }
    };

    TokenStream::from(expanded)
}

fn is_public_field(field: &syn::Field) -> bool {
    matches!(field.vis, syn::Visibility::Public(_))
}

fn convert_rust_type_to_python(rust_type: &Type) -> TokenStream2 {
    // Convert common Rust types to Python-compatible types
    match rust_type {
        Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                match segment.ident.to_string().as_str() {
                    "String" => quote! { String },
                    "u64" => quote! { u64 },
                    "i64" => quote! { i64 },
                    "f64" => quote! { f64 },
                    "bool" => quote! { bool },
                    "Decimal" => quote! { String }, // Convert Decimal to String for Python
                    _ => quote! { PyObject },
                }
            } else {
                quote! { PyObject }
            }
        }
        _ => quote! { PyObject },
    }
}
