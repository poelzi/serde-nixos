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

/// Generate the full NixOS type definition (with `let` bindings for
/// nested types) from a Rust type at compile time.
///
/// The type must derive [`NixosType`]. The macro expands to a call to
/// `T::nixos_type_full_definition()`, which produces a `let ... in typeNameType`
/// Nix expression containing all direct-child custom types as `let` bindings.
///
/// For composing **multiple** types into a single `.nix` file with proper
/// cross-references, use the runtime [`NixosModuleGenerator`] API instead.
///
/// # Example
///
/// ```ignore
/// use serde::{Serialize, Deserialize};
/// use serde_nixos::{NixosType, nixos_module};
///
/// #[derive(Serialize, Deserialize, NixosType)]
/// struct Inner { value: String }
///
/// #[derive(Serialize, Deserialize, NixosType)]
/// struct Config {
///     name: String,
///     inner: Inner,
/// }
///
/// let nix: String = nixos_module!(Config);
/// assert!(nix.contains("innerType = types.submodule"));
/// assert!(nix.contains("configType = types.submodule"));
/// ```
#[proc_macro]
pub fn nixos_module(input: TokenStream) -> TokenStream {
    let type_path = parse_macro_input!(input as syn::Type);

    quote! {
        <#type_path>::nixos_type_full_definition()
    }
    .into()
}
