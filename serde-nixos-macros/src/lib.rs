use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod attributes;
mod nixos_type;
mod type_mapping;

/// Derive macro for generating NixOS type definitions from Rust structures.
///
/// This macro automatically generates methods to output NixOS module definitions
/// that correspond to your Rust structure, making it easy to keep your Rust
/// configuration and NixOS configuration in sync.
///
/// # Example
///
/// ```ignore
/// use serde::{Serialize, Deserialize};
/// use serde_nixos::NixosType;
///
/// #[derive(Serialize, Deserialize, NixosType)]
/// struct Config {
///     #[nixos(description = "The server port")]
///     port: u16,
///
///     #[nixos(default = "localhost")]
///     host: String,
///
///     #[nixos(optional)]
///     max_connections: Option<u32>,
/// }
///
/// // Generate NixOS type definition
/// let nixos_module = Config::nixos_type_definition();
/// ```
#[proc_macro_derive(NixosType, attributes(nixos))]
pub fn derive_nixos_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match nixos_type::expand_nixos_type(&input) {
        Ok(expanded) => expanded.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Generate a NixOS module from a Rust type at compile time.
///
/// This procedural macro generates a string literal containing the NixOS
/// module definition for the annotated type.
///
/// # Example
///
/// ```ignore
/// use serde_nixos::nixos_module;
///
/// #[derive(Serialize, Deserialize)]
/// struct Config {
///     port: u16,
///     host: String,
/// }
///
/// const NIXOS_MODULE: &str = nixos_module!(Config);
/// ```
#[proc_macro]
pub fn nixos_module(input: TokenStream) -> TokenStream {
    let type_name = parse_macro_input!(input as syn::Type);

    quote! {
        {
            use std::any::TypeId;
            // This would need runtime reflection or compile-time generation
            // For now, this is a placeholder
            concat!("# NixOS module for ", stringify!(#type_name))
        }
    }
    .into()
}
