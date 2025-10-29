use proc_macro2::TokenStream;
use quote::quote;
use syn::{GenericArgument, Path, PathArguments, Type};

/// Maps Rust types to their corresponding NixOS type expressions
pub fn rust_type_to_nixos(ty: &Type) -> TokenStream {
    match ty {
        Type::Path(type_path) => path_to_nixos_type(&type_path.path),
        Type::Reference(reference) => rust_type_to_nixos(&reference.elem),
        _ => quote! { "types.attrs" }, // Fallback to attrs for unknown types
    }
}

/// Convert a type path to a NixOS type expression
fn path_to_nixos_type(path: &Path) -> TokenStream {
    let type_name = path
        .segments
        .last()
        .map(|s| s.ident.to_string())
        .unwrap_or_default();

    match type_name.as_str() {
        // Primitive types
        "bool" => quote! { "types.bool" },
        "String" | "str" => quote! { "types.str" },
        "u8" | "u16" | "u32" | "u64" | "u128" | "i8" | "i16" | "i32" | "i64" | "i128" | "usize"
        | "isize" => quote! { "types.int" },
        "f32" | "f64" => quote! { "types.float" },

        // Container types
        "Vec" => {
            if let Some(inner_type) = get_generic_inner_type(path) {
                let inner_nixos = rust_type_to_nixos(inner_type);
                quote! { format!("types.listOf {}", #inner_nixos) }
            } else {
                quote! { "types.listOf types.attrs" }
            }
        }
        "Option" => {
            if let Some(inner_type) = get_generic_inner_type(path) {
                let inner_nixos = rust_type_to_nixos(inner_type);
                quote! { format!("types.nullOr {}", #inner_nixos) }
            } else {
                quote! { "types.nullOr types.attrs" }
            }
        }
        "HashMap" | "BTreeMap" => {
            if let Some(value_type) = get_map_value_type(path) {
                let value_nixos = rust_type_to_nixos(value_type);
                quote! { format!("types.attrsOf {}", #value_nixos) }
            } else {
                quote! { "types.attrsOf types.attrs" }
            }
        }
        "HashSet" | "BTreeSet" => {
            if let Some(inner_type) = get_generic_inner_type(path) {
                let inner_nixos = rust_type_to_nixos(inner_type);
                quote! { format!("types.listOf {}", #inner_nixos) }
            } else {
                quote! { "types.listOf types.attrs" }
            }
        }

        // Path types
        "PathBuf" | "Path" => quote! { "types.path" },

        // Default to submodule for custom types
        _ => quote! { format!("types.submodule {{ /* {} options */ }}", #type_name) },
    }
}

/// Extract the inner type from a generic type like Vec<T> or Option<T>
fn get_generic_inner_type(path: &Path) -> Option<&Type> {
    let last_segment = path.segments.last()?;

    if let PathArguments::AngleBracketed(args) = &last_segment.arguments {
        if let Some(GenericArgument::Type(ty)) = args.args.first() {
            return Some(ty);
        }
    }

    None
}

/// Extract the value type from a map type like HashMap<K, V>
fn get_map_value_type(path: &Path) -> Option<&Type> {
    let last_segment = path.segments.last()?;

    if let PathArguments::AngleBracketed(args) = &last_segment.arguments {
        // Get the second generic argument (the value type)
        let mut iter = args.args.iter();
        iter.next(); // Skip key type

        if let Some(GenericArgument::Type(ty)) = iter.next() {
            return Some(ty);
        }
    }

    None
}

/// Generate a NixOS type expression for an enum
#[allow(dead_code)]
pub fn enum_to_nixos_type(variants: &[String]) -> TokenStream {
    let variants_str = variants.join(" ");
    quote! { format!("types.enum [ {} ]", #variants_str) }
}

/// Check if a type is optional (Option<T>)
pub fn is_optional_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

/// Extract the inner type from Option<T>
pub fn unwrap_option_type(ty: &Type) -> &Type {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(GenericArgument::Type(inner)) = args.args.first() {
                        return inner;
                    }
                }
            }
        }
    }
    ty
}

/// Get the custom type name if this is a custom struct/enum (not a built-in type)
pub fn get_custom_type_name(ty: &Type) -> Option<String> {
    if let Type::Path(type_path) = ty {
        let type_name = type_path.path.segments.last()?.ident.to_string();

        // Check if it's a built-in type
        match type_name.as_str() {
            "bool" | "String" | "str" | "u8" | "u16" | "u32" | "u64" | "u128" | "i8" | "i16"
            | "i32" | "i64" | "i128" | "usize" | "isize" | "f32" | "f64" | "Vec" | "Option"
            | "HashMap" | "BTreeMap" | "HashSet" | "BTreeSet" | "PathBuf" | "Path" | "Box"
            | "Rc" | "Arc" | "Value" => None,
            _ => Some(type_name),
        }
    } else {
        None
    }
}
