use syn::{token, Attribute, DeriveInput, Token};

/// Serde rename strategies for fields and enum variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenameRule {
    Lower,
    Upper,
    Pascal,
    Camel,
    Snake,
    ScreamingSnake,
    Kebab,
    ScreamingKebab,
}

impl RenameRule {
    fn from_str(value: &str) -> Option<Self> {
        match value {
            "lowercase" => Some(Self::Lower),
            "UPPERCASE" => Some(Self::Upper),
            "PascalCase" => Some(Self::Pascal),
            "camelCase" => Some(Self::Camel),
            "snake_case" => Some(Self::Snake),
            "SCREAMING_SNAKE_CASE" => Some(Self::ScreamingSnake),
            "kebab-case" => Some(Self::Kebab),
            "SCREAMING-KEBAB-CASE" => Some(Self::ScreamingKebab),
            _ => None,
        }
    }

    fn from_str_or_error(value: &str, span: proc_macro2::Span) -> syn::Result<Self> {
        Self::from_str(value).ok_or_else(|| {
            syn::Error::new(
                span,
                format!(
                    "invalid rename_all value: '{}'. Expected one of: \
                     lowercase, UPPERCASE, PascalCase, camelCase, snake_case, \
                     SCREAMING_SNAKE_CASE, kebab-case, SCREAMING-KEBAB-CASE",
                    value
                ),
            )
        })
    }
}

/// Serde attributes parsed at the struct / enum level.
#[derive(Debug, Default, Clone)]
pub struct SerdeContainerAttributes {
    pub rename_all: Option<RenameRule>,
}

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

/// Parse serde attributes from a container (struct/enum).
pub fn parse_serde_container_attributes(
    attrs: &[Attribute],
) -> syn::Result<SerdeContainerAttributes> {
    let mut serde_attrs = SerdeContainerAttributes::default();

    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename_all") {
                if meta.input.peek(Token![=]) {
                    let value = meta.value()?;
                    let s: syn::LitStr = value.parse()?;
                    serde_attrs.rename_all =
                        Some(RenameRule::from_str_or_error(&s.value(), s.span())?);
                } else if meta.input.peek(token::Paren) {
                    let mut serialize_rule: Option<RenameRule> = None;
                    let mut deserialize_rule: Option<RenameRule> = None;

                    meta.parse_nested_meta(|nested| {
                        if nested.path.is_ident("serialize") {
                            let value = nested.value()?;
                            let s: syn::LitStr = value.parse()?;
                            serialize_rule =
                                Some(RenameRule::from_str_or_error(&s.value(), s.span())?);
                        } else if nested.path.is_ident("deserialize") {
                            let value = nested.value()?;
                            let s: syn::LitStr = value.parse()?;
                            deserialize_rule =
                                Some(RenameRule::from_str_or_error(&s.value(), s.span())?);
                        } else {
                            consume_meta_input(&nested)?;
                        }
                        Ok(())
                    })?;

                    serde_attrs.rename_all = deserialize_rule.or(serialize_rule);
                }
            } else {
                // Consume unknown serde container meta to avoid parser errors on value-bearing attributes.
                consume_meta_input(&meta)?;
            }

            Ok(())
        })?;
    }

    Ok(serde_attrs)
}

fn consume_meta_input(meta: &syn::meta::ParseNestedMeta<'_>) -> syn::Result<()> {
    if meta.input.peek(Token![=]) {
        let value = meta.value()?;
        let _: syn::Expr = value.parse()?;
    } else if meta.input.peek(token::Paren) {
        meta.parse_nested_meta(|nested| {
            consume_meta_input(&nested)?;
            Ok(())
        })?;
    }

    Ok(())
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
                if meta.input.peek(Token![=]) {
                    let value = meta.value()?;
                    let s: syn::LitStr = value.parse()?;
                    serde_attrs.rename = Some(s.value());
                } else if meta.input.peek(token::Paren) {
                    let mut serialize_name: Option<String> = None;
                    let mut deserialize_name: Option<String> = None;

                    meta.parse_nested_meta(|nested| {
                        if nested.path.is_ident("serialize") {
                            let value = nested.value()?;
                            let s: syn::LitStr = value.parse()?;
                            serialize_name = Some(s.value());
                        } else if nested.path.is_ident("deserialize") {
                            let value = nested.value()?;
                            let s: syn::LitStr = value.parse()?;
                            deserialize_name = Some(s.value());
                        }
                        Ok(())
                    })?;

                    serde_attrs.rename = deserialize_name.or(serialize_name);
                }
            } else if meta.path.is_ident("skip") {
                serde_attrs.skip = true;
            } else if meta.path.is_ident("skip_serializing") {
                serde_attrs.skip_serializing = true;
            } else if meta.path.is_ident("skip_deserializing") {
                serde_attrs.skip_deserializing = true;
            } else if meta.path.is_ident("default") {
                // Handle both #[serde(default)] and #[serde(default = "function")]
                serde_attrs.has_default = true;
                // If there's a value (like default = "function"), consume it to prevent parse errors
                // We don't need to store the function name for NixOS generation
                if let Ok(value) = meta.value() {
                    let _: syn::Result<syn::LitStr> = value.parse();
                }
            } else if meta.path.is_ident("flatten") {
                serde_attrs.flatten = true;
            } else if meta.path.is_ident("skip_serializing_if") {
                // Ignore this serde-only behavior, but consume its value to avoid parse errors.
                let value = meta.value()?;
                let _: syn::LitStr = value.parse()?;
            } else if meta.path.is_ident("alias")
                || meta.path.is_ident("serialize_with")
                || meta.path.is_ident("deserialize_with")
                || meta.path.is_ident("with")
                || meta.path.is_ident("getter")
            {
                let value = meta.value()?;
                let _: syn::LitStr = value.parse()?;
            } else if meta.path.is_ident("borrow") {
                if meta.input.peek(Token![=]) {
                    let value = meta.value()?;
                    let _: syn::LitStr = value.parse()?;
                }
            } else if meta.path.is_ident("bound") {
                if meta.input.peek(Token![=]) {
                    let value = meta.value()?;
                    let _: syn::LitStr = value.parse()?;
                } else if meta.input.peek(token::Paren) {
                    meta.parse_nested_meta(|nested| {
                        if nested.path.is_ident("serialize") || nested.path.is_ident("deserialize")
                        {
                            let value = nested.value()?;
                            let _: syn::LitStr = value.parse()?;
                        }
                        Ok(())
                    })?;
                }
            } else if meta.path.is_ident("rename_all") {
                // rename_all is only valid at container level (struct/enum), not on fields/variants.
                return Err(meta.error(
                    "`rename_all` is only valid at container level (struct/enum). \
                     Use `rename` for field-level renaming.",
                ));
            } else {
                // Best-effort consumption for future serde attributes with values/lists.
                consume_meta_input(&meta)?;
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

/// Apply serde rename rule to a struct field name (assumed snake_case input).
///
/// Matches serde's `RenameRule::apply_to_field` behavior:
/// - `lowercase` / `snake_case` / `SCREAMING_SNAKE_CASE` / `UPPERCASE` operate on the
///   raw field string (preserving underscores), because fields are already snake_case.
/// - `camelCase` / `PascalCase` / `kebab-case` / `SCREAMING-KEBAB-CASE` split on
///   underscores and rejoin with the target casing.
pub fn apply_rename_rule_to_field(input: &str, rule: RenameRule) -> String {
    match rule {
        // For fields (already snake_case), serde just transforms the whole string
        RenameRule::Lower => input.to_ascii_lowercase(),
        RenameRule::Upper => input.to_ascii_uppercase(),
        RenameRule::Snake => input.to_string(),
        RenameRule::ScreamingSnake => input.to_ascii_uppercase(),
        // These need word splitting
        RenameRule::Pascal => {
            let words = split_words(input);
            words
                .iter()
                .map(|w| capitalize(w))
                .collect::<Vec<_>>()
                .join("")
        }
        RenameRule::Camel => {
            let words = split_words(input);
            if words.is_empty() {
                String::new()
            } else {
                let mut result = words[0].to_string();
                for word in words.iter().skip(1) {
                    result.push_str(&capitalize(word));
                }
                result
            }
        }
        RenameRule::Kebab => input.replace('_', "-"),
        RenameRule::ScreamingKebab => input.replace('_', "-").to_ascii_uppercase(),
    }
}

/// Apply serde rename rule to an enum variant name (assumed PascalCase input).
///
/// Matches serde's `RenameRule::apply_to_variant` behavior:
/// - `PascalCase` is a no-op (variants are already PascalCase).
/// - `lowercase` / `UPPERCASE` operate on the raw string (no separators inserted).
/// - `snake_case` / `camelCase` / `kebab-case` etc. split on PascalCase word boundaries.
pub fn apply_rename_rule_to_variant(input: &str, rule: RenameRule) -> String {
    match rule {
        RenameRule::Pascal => input.to_string(),
        RenameRule::Lower => input.to_ascii_lowercase(),
        RenameRule::Upper => input.to_ascii_uppercase(),
        // These need word splitting from PascalCase
        RenameRule::Camel => {
            let words = split_words(input);
            if words.is_empty() {
                String::new()
            } else {
                let mut result = words[0].to_string();
                for word in words.iter().skip(1) {
                    result.push_str(&capitalize(word));
                }
                result
            }
        }
        RenameRule::Snake => {
            let words = split_words(input);
            words.join("_")
        }
        RenameRule::ScreamingSnake => {
            let words = split_words(input);
            words.join("_").to_uppercase()
        }
        RenameRule::Kebab => {
            let words = split_words(input);
            words.join("-")
        }
        RenameRule::ScreamingKebab => {
            let words = split_words(input);
            words.join("-").to_uppercase()
        }
    }
}

fn split_words(input: &str) -> Vec<String> {
    if input.is_empty() {
        return Vec::new();
    }

    let chars: Vec<char> = input.chars().collect();
    let mut words: Vec<String> = Vec::new();
    let mut current = String::new();

    for (i, ch) in chars.iter().enumerate() {
        if *ch == '_' || *ch == '-' || *ch == ' ' {
            if !current.is_empty() {
                words.push(current.to_lowercase());
                current.clear();
            }
            continue;
        }

        let prev = if i > 0 { Some(chars[i - 1]) } else { None };
        let next = chars.get(i + 1).copied();

        let is_boundary = match (prev, next) {
            (Some(p), Some(n)) => {
                (p.is_ascii_lowercase() && ch.is_ascii_uppercase())
                    || (p.is_ascii_uppercase() && ch.is_ascii_uppercase() && n.is_ascii_lowercase())
            }
            (Some(p), None) => p.is_ascii_lowercase() && ch.is_ascii_uppercase(),
            _ => false,
        };

        if is_boundary && !current.is_empty() {
            words.push(current.to_lowercase());
            current.clear();
        }

        current.push(*ch);
    }

    if !current.is_empty() {
        words.push(current.to_lowercase());
    }

    words
}

fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            let mut out = String::new();
            out.extend(first.to_uppercase());
            out.push_str(chars.as_str());
            out
        }
    }
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
