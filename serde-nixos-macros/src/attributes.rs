use syn::{Attribute, DeriveInput};

/// Attributes that can be applied at the struct level with #[nixos(...)]
#[derive(Debug, Default, Clone)]
pub struct NixosStructAttributes {
    /// Automatically use doc comments as descriptions for all fields
    pub auto_doc: bool,
}

/// Parse #[nixos(...)] attributes from a struct
pub fn parse_nixos_struct_attributes(input: &DeriveInput) -> syn::Result<NixosStructAttributes> {
    let mut struct_attrs = NixosStructAttributes::default();

    for attr in &input.attrs {
        if !attr.path().is_ident("nixos") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("auto_doc") {
                struct_attrs.auto_doc = true;
            } else {
                return Err(meta.error("unsupported nixos struct attribute"));
            }
            Ok(())
        })?;
    }

    Ok(struct_attrs)
}

/// Attributes that can be applied to fields with #[nixos(...)]
#[derive(Debug, Default, Clone)]
pub struct NixosFieldAttributes {
    pub description: Option<String>,
    pub default: Option<String>,
    pub default_text: Option<String>,
    pub example: Option<String>,
    pub apply: Option<String>,
    pub internal: bool,
    pub visible: Option<String>,
    pub read_only: bool,
    pub related_packages: Option<String>,
    pub optional: bool,
    pub rename: Option<String>,
    pub skip: bool,
}

/// Parse #[nixos(...)] attributes from a field
pub fn parse_nixos_attributes(attrs: &[Attribute]) -> syn::Result<NixosFieldAttributes> {
    let mut nixos_attrs = NixosFieldAttributes::default();

    for attr in attrs {
        if !attr.path().is_ident("nixos") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("description") {
                let value = meta.value()?;
                let s: syn::LitStr = value.parse()?;
                nixos_attrs.description = Some(s.value());
            } else if meta.path.is_ident("default") {
                let value = meta.value()?;
                let s: syn::LitStr = value.parse()?;
                nixos_attrs.default = Some(s.value());
            } else if meta.path.is_ident("default_text") || meta.path.is_ident("defaultText") {
                let value = meta.value()?;
                let s: syn::LitStr = value.parse()?;
                nixos_attrs.default_text = Some(s.value());
            } else if meta.path.is_ident("example") {
                let value = meta.value()?;
                let s: syn::LitStr = value.parse()?;
                nixos_attrs.example = Some(s.value());
            } else if meta.path.is_ident("apply") {
                let value = meta.value()?;
                let s: syn::LitStr = value.parse()?;
                nixos_attrs.apply = Some(s.value());
            } else if meta.path.is_ident("internal") {
                nixos_attrs.internal = true;
            } else if meta.path.is_ident("visible") {
                let value = meta.value()?;
                let s: syn::LitStr = value.parse()?;
                nixos_attrs.visible = Some(s.value());
            } else if meta.path.is_ident("read_only") || meta.path.is_ident("readOnly") {
                nixos_attrs.read_only = true;
            } else if meta.path.is_ident("related_packages")
                || meta.path.is_ident("relatedPackages")
            {
                let value = meta.value()?;
                let s: syn::LitStr = value.parse()?;
                nixos_attrs.related_packages = Some(s.value());
            } else if meta.path.is_ident("optional") {
                nixos_attrs.optional = true;
            } else if meta.path.is_ident("rename") {
                let value = meta.value()?;
                let s: syn::LitStr = value.parse()?;
                nixos_attrs.rename = Some(s.value());
            } else if meta.path.is_ident("skip") {
                nixos_attrs.skip = true;
            } else {
                return Err(meta.error("unsupported nixos attribute"));
            }
            Ok(())
        })?;
    }

    Ok(nixos_attrs)
}

/// Parse serde attributes that affect the NixOS output
pub fn parse_serde_attributes(attrs: &[Attribute]) -> syn::Result<SerdeAttributes> {
    let mut serde_attrs = SerdeAttributes::default();

    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename") {
                let value = meta.value()?;
                let s: syn::LitStr = value.parse()?;
                serde_attrs.rename = Some(s.value());
            } else if meta.path.is_ident("skip") {
                serde_attrs.skip = true;
            } else if meta.path.is_ident("skip_serializing") {
                serde_attrs.skip_serializing = true;
            } else if meta.path.is_ident("skip_deserializing") {
                serde_attrs.skip_deserializing = true;
            } else if meta.path.is_ident("default") {
                serde_attrs.has_default = true;
            } else if meta.path.is_ident("flatten") {
                serde_attrs.flatten = true;
            }
            Ok(())
        })?;
    }

    Ok(serde_attrs)
}

/// Serde attributes that we care about
#[derive(Debug, Default, Clone)]
pub struct SerdeAttributes {
    pub rename: Option<String>,
    pub skip: bool,
    pub skip_serializing: bool,
    pub skip_deserializing: bool,
    pub has_default: bool,
    pub flatten: bool,
}

/// Combine nixos and serde attributes to get the effective attributes
pub fn combine_attributes(
    nixos: NixosFieldAttributes,
    serde: SerdeAttributes,
    doc_comment: Option<String>,
    auto_doc: bool,
) -> EffectiveAttributes {
    // If auto_doc is enabled, prefer doc comments over explicit descriptions
    // Otherwise, prefer explicit descriptions over doc comments
    let description = if auto_doc {
        doc_comment.or(nixos.description)
    } else {
        nixos.description.or(doc_comment)
    };

    EffectiveAttributes {
        name: nixos.rename.or(serde.rename),
        description,
        default: nixos.default,
        default_text: nixos.default_text,
        example: nixos.example,
        apply: nixos.apply,
        internal: nixos.internal,
        visible: nixos.visible,
        read_only: nixos.read_only,
        related_packages: nixos.related_packages,
        optional: nixos.optional || serde.has_default,
        skip: nixos.skip || serde.skip,
        flatten: serde.flatten,
    }
}

/// The effective attributes after combining nixos and serde attributes
#[derive(Debug, Clone)]
pub struct EffectiveAttributes {
    pub name: Option<String>,
    pub description: Option<String>,
    pub default: Option<String>,
    pub default_text: Option<String>,
    pub example: Option<String>,
    pub apply: Option<String>,
    pub internal: bool,
    pub visible: Option<String>,
    pub read_only: bool,
    pub related_packages: Option<String>,
    #[allow(dead_code)]
    pub optional: bool,
    pub skip: bool,
    #[allow(dead_code)]
    pub flatten: bool,
}

/// Extract documentation comments from attributes
pub fn extract_doc_comments(attrs: &[Attribute]) -> Option<String> {
    let mut docs = Vec::new();

    for attr in attrs {
        if attr.path().is_ident("doc") {
            if let Ok(meta) = attr.meta.require_name_value() {
                if let syn::Expr::Lit(expr_lit) = &meta.value {
                    if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                        let comment = lit_str.value();
                        // Trim leading space that rustdoc adds
                        let trimmed = comment.strip_prefix(' ').unwrap_or(&comment);
                        docs.push(trimmed.to_string());
                    }
                }
            }
        }
    }

    if docs.is_empty() {
        None
    } else {
        Some(docs.join("\n").trim().to_string())
    }
}
