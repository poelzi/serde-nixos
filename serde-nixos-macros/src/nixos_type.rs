use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;
use syn::{Data, DeriveInput, Fields, FieldsNamed, Ident, Result, Type};

use crate::attributes::{
    combine_attributes, extract_doc_comments, parse_nixos_attributes,
    parse_nixos_struct_attributes, parse_serde_attributes,
};
use crate::type_mapping::{
    get_custom_type_name, is_optional_type, rust_type_to_nixos, unwrap_option_type,
};

pub fn expand_nixos_type(input: &DeriveInput) -> Result<TokenStream> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Parse struct-level attributes
    let struct_attrs = parse_nixos_struct_attributes(input)?;

    let type_name = generate_type_name(name);

    let body = match &input.data {
        Data::Struct(data_struct) => generate_struct_impl(&data_struct.fields, name, &type_name)?,
        Data::Enum(data_enum) => {
            // For enums, generate a type.enum with all variants
            let variants: Vec<String> = data_enum
                .variants
                .iter()
                .map(|v| format!("\"{}\"", v.ident))
                .collect();

            let variants_str = variants.join(" ");
            quote! {
                format!("types.enum [ {} ]", #variants_str)
            }
        }
        Data::Union(_) => {
            return Err(syn::Error::new_spanned(
                input,
                "Union types are not supported by serde-nixos.\n\
                 \n\
                 Unions cannot be safely serialized with serde and don't have a clear\n\
                 NixOS type mapping. Consider using an enum instead:\n\
                 \n\
                 Instead of:\n\
                   union MyUnion { ... }\n\
                 \n\
                 Use:\n\
                   enum MyEnum {\n\
                       Variant1(Type1),\n\
                       Variant2(Type2),\n\
                   }",
            ));
        }
    };

    let nixos_type_def =
        generate_nixos_type_definition(&input.data, name, &type_name, struct_attrs.auto_doc)?;
    let nixos_options = generate_nixos_options(&input.data, name, struct_attrs.auto_doc)?;
    let nixos_type_name_literal = type_name.clone();

    // Generate the full definition with all dependent types
    let nixos_full_def =
        generate_nixos_full_definition(&input.data, name, &type_name, struct_attrs.auto_doc)?;

    Ok(quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            /// Generate a complete NixOS module definition for this type
            pub fn nixos_type_definition() -> String {
                #nixos_type_def
            }

            /// Generate just the options portion of the NixOS module
            pub fn nixos_options() -> String {
                #nixos_options
            }

            /// Get the NixOS type expression for this type
            pub fn nixos_type() -> String {
                #body
            }

            /// Get the NixOS type name for this struct
            pub fn nixos_type_name() -> &'static str {
                #nixos_type_name_literal
            }

            /// Generate the full NixOS type definition with all dependencies
            /// Creates a `let` chain with all submodules defined first
            pub fn nixos_type_full_definition() -> String {
                #nixos_full_def
            }
        }
    })
}

/// Generate a camelCase type name from a struct name
fn generate_type_name(ident: &Ident) -> String {
    let name = ident.to_string();
    // Convert to camelCase: ServerConfig -> serverConfigType
    let mut chars = name.chars();
    match chars.next() {
        None => "type".to_string(),
        Some(f) => format!("{}{}Type", f.to_lowercase(), chars.as_str()),
    }
}

fn generate_struct_impl(
    fields: &Fields,
    _struct_name: &Ident,
    type_name: &str,
) -> Result<TokenStream> {
    match fields {
        Fields::Named(_fields) => Ok(quote! {
            format!("{}", #type_name)
        }),
        Fields::Unnamed(_) => Ok(quote! {
            "types.attrs".to_string()
        }),
        Fields::Unit => Ok(quote! {
            "types.null".to_string()
        }),
    }
}

fn generate_nixos_type_definition(
    data: &Data,
    name: &Ident,
    type_name: &str,
    auto_doc: bool,
) -> Result<TokenStream> {
    let struct_name_str = name.to_string();

    match data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => {
                let options_body = generate_options_for_fields(fields, false, auto_doc)?;
                Ok(quote! {
                    {
                        let mut result = String::new();
                        result.push_str("# NixOS type definition for ");
                        result.push_str(#struct_name_str);
                        result.push_str("\n");
                        result.push_str(#type_name);
                        result.push_str(" = types.submodule {\n  options = {\n");
                        #options_body
                        result.push_str("  };\n};\n");
                        result
                    }
                })
            }
            _ => Ok(quote! {
                format!("# NixOS type definition for {}\n{} = types.attrs;", #struct_name_str, #type_name)
            }),
        },
        Data::Enum(data_enum) => {
            let variants: Vec<String> = data_enum
                .variants
                .iter()
                .map(|v| format!("\"{}\"", v.ident))
                .collect();
            let variants_str = variants.join(" ");

            Ok(quote! {
                format!(
                    "# NixOS type definition for {}\n{} = types.enum [ {} ];",
                    #struct_name_str,
                    #type_name,
                    #variants_str
                )
            })
        }
        Data::Union(_) => Err(syn::Error::new_spanned(
            name,
            "Union types are not supported. Use enums instead.",
        )),
    }
}

fn generate_nixos_options(data: &Data, _name: &Ident, auto_doc: bool) -> Result<TokenStream> {
    match data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => {
                let options_body = generate_options_for_fields(fields, false, auto_doc)?;
                Ok(quote! {
                    {
                        let mut result = String::new();
                        #options_body
                        result
                    }
                })
            }
            _ => Ok(quote! { String::new() }),
        },
        _ => Ok(quote! { String::new() }),
    }
}

/// Generate the full definition with let bindings for all dependent types
///
/// This function generates a complete let-in expression with all nested custom types.
///
/// ## Features
/// - Recursively collects all custom types from fields
/// - Handles nested types within Option, Vec, HashMap, Box, Rc, Arc, etc.
/// - Generates proper let-in structure with all dependent types
/// - Each custom type calls its own `nixos_type()` method
///
/// ## Example Output
/// For a struct with nested types:
/// ```rust
/// struct DatabaseConfig { host: String, port: u16 }
/// struct AppConfig { database: DatabaseConfig }
/// ```
///
/// This generates:
/// ```nix
/// let
///   databaseConfigType = types.submodule { options = { ... }; };
///   appConfigType = types.submodule {
///     options = {
///       database = mkOption { type = databaseConfigType; };
///     };
///   };
/// in appConfigType
/// ```
///
/// ## Implementation Note
/// The function recursively traverses type structures to find all custom types,
/// then generates let bindings by calling the `NixosType::nixos_type()` trait method
/// on each discovered type.
fn generate_nixos_full_definition(
    data: &Data,
    name: &Ident,
    type_name: &str,
    auto_doc: bool,
) -> Result<TokenStream> {
    match data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields) => {
                    // Collect all custom types used in fields (recursively)
                    let mut custom_types = HashSet::new();
                    collect_custom_types(fields, &mut custom_types);

                    let options_body = generate_options_for_fields(fields, true, auto_doc)?;

                    // Generate let bindings for nested custom types
                    let nested_type_bindings = if custom_types.is_empty() {
                        quote! {}
                    } else {
                        let mut bindings = Vec::new();
                        for custom_type in custom_types {
                            let type_ident =
                                syn::Ident::new(&custom_type, proc_macro2::Span::call_site());
                            let generated_name = generate_type_name(&type_ident);
                            bindings.push(quote! {
                                result.push_str("  ");
                                result.push_str(#generated_name);
                                result.push_str(" = types.submodule {\n    options = {\n");
                                // Get options and indent each line by 4 spaces (matching main type indentation)
                                let options = #type_ident::nixos_options();
                                for line in options.lines() {
                                    if !line.is_empty() {
                                        result.push_str("    ");
                                        result.push_str(line);
                                    }
                                    result.push_str("\n");
                                }
                                result.push_str("    };\n  };\n");
                            });
                        }
                        quote! { #(#bindings)* }
                    };

                    Ok(quote! {
                        {
                            let mut result = String::new();

                            result.push_str("let\n");

                            // Generate let bindings for nested custom types
                            #nested_type_bindings

                            // Generate the main type definition
                            result.push_str("  ");
                            result.push_str(#type_name);
                            result.push_str(" = types.submodule {\n    options = {\n");
                            #options_body
                            result.push_str("    };\n  };\n");
                            result.push_str("in ");
                            result.push_str(#type_name);
                            result.push_str("\n");

                            result
                        }
                    })
                }
                _ => Ok(quote! {
                    format!("let {} = types.attrs; in {}", #type_name, #type_name)
                }),
            }
        }
        Data::Enum(_) => generate_nixos_type_definition(data, name, type_name, auto_doc),
        Data::Union(_) => Err(syn::Error::new_spanned(
            name,
            "Union types are not supported. Use enums instead.",
        )),
    }
}

/// Collect custom type names from fields (recursively handles nested types)
fn collect_custom_types(fields: &FieldsNamed, types: &mut HashSet<String>) {
    for field in &fields.named {
        collect_custom_types_from_type(&field.ty, types);
    }
}

/// Recursively collect custom type names from a type, including nested types
fn collect_custom_types_from_type(ty: &Type, types: &mut HashSet<String>) {
    if let Type::Path(type_path) = ty {
        let segment = type_path.path.segments.last();
        if let Some(seg) = segment {
            let type_name = seg.ident.to_string();

            // Check if this is a wrapper type (Option, Vec, HashMap, etc.)
            match type_name.as_str() {
                "Option" | "Vec" | "Box" | "Rc" | "Arc" => {
                    // Extract inner type from generic arguments
                    if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                        for arg in &args.args {
                            if let syn::GenericArgument::Type(inner_ty) = arg {
                                collect_custom_types_from_type(inner_ty, types);
                            }
                        }
                    }
                }
                "HashMap" | "BTreeMap" => {
                    // HashMap<K, V> - collect both key and value types
                    if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                        for arg in &args.args {
                            if let syn::GenericArgument::Type(inner_ty) = arg {
                                collect_custom_types_from_type(inner_ty, types);
                            }
                        }
                    }
                }
                _ => {
                    // Check if this is a custom (non-built-in) type
                    if let Some(custom_name) = get_custom_type_name(ty) {
                        types.insert(custom_name);
                    }
                }
            }
        }
    }
}

fn generate_options_for_fields(
    fields: &FieldsNamed,
    use_named_types: bool,
    auto_doc: bool,
) -> Result<TokenStream> {
    let mut field_options = Vec::new();

    for field in &fields.named {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        // Parse attributes
        let nixos_attrs = parse_nixos_attributes(&field.attrs)?;
        let serde_attrs = parse_serde_attributes(&field.attrs)?;
        let doc_comment = extract_doc_comments(&field.attrs);
        let effective_attrs = combine_attributes(nixos_attrs, serde_attrs, doc_comment, auto_doc);

        // Skip if marked to skip
        if effective_attrs.skip {
            continue;
        }

        // Determine field name (considering rename)
        let nix_field_name = effective_attrs
            .name
            .as_ref()
            .unwrap_or(&field_name.to_string())
            .clone();

        // Generate the type expression
        let type_expr = if is_optional_type(field_type) {
            let inner_type = unwrap_option_type(field_type);
            let inner_nixos = if use_named_types {
                rust_type_to_nixos_named(inner_type)
            } else {
                rust_type_to_nixos(inner_type)
            };
            quote! {
                {
                    let inner = #inner_nixos;
                    format!("types.nullOr {}", inner)
                }
            }
        } else if use_named_types {
            rust_type_to_nixos_named(field_type)
        } else {
            rust_type_to_nixos(field_type)
        };

        // Build the option definition with proper indentation
        let indent = if use_named_types { "      " } else { "    " };
        let field_indent = if use_named_types { "    " } else { "  " };

        field_options.push(quote! {
            result.push_str(#field_indent);
            result.push_str(#nix_field_name);
            result.push_str(" = lib.mkOption {\n");
        });

        // Add type
        field_options.push(quote! {
            result.push_str(#indent);
            result.push_str("type = ");
            result.push_str(&#type_expr);
            result.push_str(";\n");
        });

        // Add description if present
        if let Some(desc) = &effective_attrs.description {
            let escaped_desc = desc.replace('"', "\\\"").replace('\n', "\\n");
            field_options.push(quote! {
                result.push_str(#indent);
                result.push_str("description = \"");
                result.push_str(#escaped_desc);
                result.push_str("\";\n");
            });
        }

        // Add default if present
        if let Some(default) = &effective_attrs.default {
            field_options.push(quote! {
                result.push_str(#indent);
                result.push_str("default = ");
                result.push_str(#default);
                result.push_str(";\n");
            });
        }

        // Add defaultText if present
        if let Some(default_text) = &effective_attrs.default_text {
            field_options.push(quote! {
                result.push_str(#indent);
                result.push_str("defaultText = ");
                result.push_str(#default_text);
                result.push_str(";\n");
            });
        }

        // Add example if present
        if let Some(example) = &effective_attrs.example {
            field_options.push(quote! {
                result.push_str(#indent);
                result.push_str("example = ");
                result.push_str(#example);
                result.push_str(";\n");
            });
        }

        // Add apply if present
        if let Some(apply) = &effective_attrs.apply {
            field_options.push(quote! {
                result.push_str(#indent);
                result.push_str("apply = ");
                result.push_str(#apply);
                result.push_str(";\n");
            });
        }

        // Add internal if set
        if effective_attrs.internal {
            field_options.push(quote! {
                result.push_str(#indent);
                result.push_str("internal = true;\n");
            });
        }

        // Add visible if present
        if let Some(visible) = &effective_attrs.visible {
            field_options.push(quote! {
                result.push_str(#indent);
                result.push_str("visible = ");
                result.push_str(#visible);
                result.push_str(";\n");
            });
        }

        // Add readOnly if set
        if effective_attrs.read_only {
            field_options.push(quote! {
                result.push_str(#indent);
                result.push_str("readOnly = true;\n");
            });
        }

        // Add relatedPackages if present
        if let Some(related) = &effective_attrs.related_packages {
            field_options.push(quote! {
                result.push_str(#indent);
                result.push_str("relatedPackages = ");
                result.push_str(#related);
                result.push_str(";\n");
            });
        }

        field_options.push(quote! {
            result.push_str(#field_indent);
            result.push_str("};\n\n");
        });
    }

    Ok(quote! {
        #(#field_options)*
    })
}

/// Generate nixos type expression using named types for custom structs
fn rust_type_to_nixos_named(ty: &Type) -> TokenStream {
    if let Some(type_name) = get_custom_type_name(ty) {
        // For custom types, use the type name directly
        let camel_case_name = {
            let mut chars = type_name.chars();
            match chars.next() {
                None => "type".to_string(),
                Some(f) => format!("{}{}Type", f.to_lowercase(), chars.as_str()),
            }
        };
        quote! { #camel_case_name.to_string() }
    } else {
        // Fall back to regular type mapping
        rust_type_to_nixos(ty)
    }
}
