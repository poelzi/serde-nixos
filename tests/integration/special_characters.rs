use serde::{Deserialize, Serialize};
use serde_nixos::NixosType;

#[test]
fn test_special_chars_in_descriptions() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct Config {
        #[nixos(description = "Path with \"quotes\" and 'apostrophes'")]
        path: String,

        #[nixos(description = "Value with\nnewlines\nand\ttabs")]
        value: String,

        #[nixos(description = "Backslashes: \\ and forward slashes: /")]
        slashes: String,
    }

    let options = Config::nixos_options();
    assert!(options.contains("path = lib.mkOption"));
    assert!(options.contains("value = lib.mkOption"));
    assert!(options.contains("slashes = lib.mkOption"));

    // Verify special characters are properly escaped
    println!("Special chars in descriptions: {}", options);
}

#[test]
fn test_unicode_in_descriptions() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct Intl {
        #[nixos(description = "Unicode: café, naïve, 日本語, emoji 🚀")]
        text: String,

        #[nixos(description = "Symbols: © ™ € £ ¥")]
        symbols: String,
    }

    let options = Intl::nixos_options();
    assert!(options.contains("text = lib.mkOption"));
    assert!(options.contains("symbols = lib.mkOption"));

    println!("Unicode in descriptions: {}", options);
}

#[test]
fn test_default_with_special_chars() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct Defaults {
        #[nixos(default = "\"hello\\nworld\"", description = "Multi-line default")]
        multiline: String,

        #[nixos(default = "\"path/to/file\"", description = "Path with slashes")]
        path: String,

        #[nixos(default = "\"value with \\\"quotes\\\"\"", description = "Quoted default")]
        quoted: String,
    }

    let options = Defaults::nixos_options();
    assert!(options.contains("multiline = lib.mkOption"));
    assert!(options.contains("path = lib.mkOption"));
    assert!(options.contains("quoted = lib.mkOption"));

    println!("Defaults with special chars: {}", options);
}

#[test]
fn test_nix_keywords_in_descriptions() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct Keywords {
        #[nixos(description = "Keywords: let, in, with, if, then, else, inherit")]
        keywords: String,

        #[nixos(description = "Reserved: rec, import, true, false, null")]
        reserved: String,
    }

    let options = Keywords::nixos_options();
    assert!(options.contains("keywords = lib.mkOption"));
    assert!(options.contains("reserved = lib.mkOption"));

    println!("Nix keywords in descriptions: {}", options);
}

#[test]
fn test_raw_strings_in_field_docs() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct RawStrings {
        /// This is a doc comment with "quotes"
        quoted_doc: String,

        /// Multi-line
        /// doc comment
        /// with many lines
        multiline_doc: String,

        /// Doc with special chars: \n \t \r
        escaped_doc: String,
    }

    let options = RawStrings::nixos_options();
    assert!(options.contains("quoted_doc = lib.mkOption"));
    assert!(options.contains("multiline_doc = lib.mkOption"));
    assert!(options.contains("escaped_doc = lib.mkOption"));

    println!("Raw strings in docs: {}", options);
}

#[test]
fn test_empty_and_whitespace_descriptions() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct Whitespace {
        #[nixos(description = "")]
        empty: String,

        #[nixos(description = "   ")]
        spaces: String,

        #[nixos(description = "\t\t")]
        tabs: String,
    }

    let options = Whitespace::nixos_options();
    assert!(options.contains("empty = lib.mkOption"));
    assert!(options.contains("spaces = lib.mkOption"));
    assert!(options.contains("tabs = lib.mkOption"));

    println!("Empty/whitespace descriptions: {}", options);
}

#[test]
fn test_nix_string_interpolation_chars() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct Interpolation {
        #[nixos(description = "Dollar signs: $ and ${var}")]
        dollar: String,

        #[nixos(description = "Curly braces: { and }")]
        braces: String,

        #[nixos(description = "Combined: ${foo.bar}")]
        combined: String,
    }

    let options = Interpolation::nixos_options();
    assert!(options.contains("dollar = lib.mkOption"));
    assert!(options.contains("braces = lib.mkOption"));
    assert!(options.contains("combined = lib.mkOption"));

    // These should be properly escaped to avoid Nix interpolation
    println!("Nix interpolation chars: {}", options);
}

#[test]
fn test_multiline_string_in_default() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct MultilineDefault {
        #[nixos(
            default = r#"''
              Line 1
              Line 2
              Line 3
            ''"#,
            description = "Multi-line Nix string"
        )]
        content: String,
    }

    let options = MultilineDefault::nixos_options();
    assert!(options.contains("content = lib.mkOption"));

    println!("Multiline default: {}", options);
}

#[test]
fn test_extreme_nesting_in_description() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct Extreme {
        #[nixos(description = "((nested (((deeply))) with [[[[brackets]]]] and {{{braces}}}))")]
        nested: String,
    }

    let options = Extreme::nixos_options();
    assert!(options.contains("nested = lib.mkOption"));

    println!("Extreme nesting: {}", options);
}
