/// Tests for correct parenthesization of compound Nix type expressions.
///
/// In Nix, function application is left-associative, so:
///   `types.nullOr types.listOf types.str`
/// parses as `(types.nullOr types.listOf) types.str` which is wrong.
///
/// Compound inner types (those containing a space) must be wrapped in parens:
///   `types.nullOr (types.listOf types.str)`
///
/// Simple inner types should NOT be wrapped:
///   `types.nullOr types.str`
use serde::{Deserialize, Serialize};
use serde_nixos::NixosType;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

// ── Vec (listOf) ────────────────────────────────────────────────────────

#[test]
fn test_vec_simple_no_parens() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct C {
        v: Vec<String>,
    }
    let opts = C::nixos_options();
    assert!(
        opts.contains("types.listOf types.str"),
        "simple inner type should not be parenthesized: {}",
        opts
    );
    assert!(
        !opts.contains("types.listOf (types.str)"),
        "simple inner type should not be parenthesized: {}",
        opts
    );
}

#[test]
fn test_vec_of_vec_parenthesized() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct C {
        v: Vec<Vec<String>>,
    }
    let opts = C::nixos_options();
    assert!(
        opts.contains("types.listOf (types.listOf types.str)"),
        "compound inner type must be parenthesized: {}",
        opts
    );
}

#[test]
fn test_vec_of_option_parenthesized() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct C {
        v: Vec<Option<String>>,
    }
    let opts = C::nixos_options();
    assert!(
        opts.contains("types.listOf (types.nullOr types.str)"),
        "compound inner type must be parenthesized: {}",
        opts
    );
}

#[test]
fn test_vec_triple_nesting() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct C {
        v: Vec<Vec<Vec<u32>>>,
    }
    let opts = C::nixos_options();
    assert!(
        opts.contains("types.listOf (types.listOf (types.listOf types.int))"),
        "triply-nested types must have correct parenthesization: {}",
        opts
    );
}

// ── Option (nullOr) ─────────────────────────────────────────────────────

#[test]
fn test_option_simple_no_parens() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct C {
        v: Option<String>,
    }
    let opts = C::nixos_options();
    assert!(
        opts.contains("types.nullOr types.str"),
        "simple inner type should not be parenthesized: {}",
        opts
    );
    assert!(
        !opts.contains("types.nullOr (types.str)"),
        "simple inner type should not be parenthesized: {}",
        opts
    );
}

#[test]
fn test_option_of_vec_parenthesized() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct C {
        v: Option<Vec<String>>,
    }
    let opts = C::nixos_options();
    assert!(
        opts.contains("types.nullOr (types.listOf types.str)"),
        "compound inner type must be parenthesized: {}",
        opts
    );
}

#[test]
fn test_option_of_hashmap_parenthesized() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct C {
        v: Option<HashMap<String, u32>>,
    }
    let opts = C::nixos_options();
    assert!(
        opts.contains("types.nullOr (types.attrsOf types.int)"),
        "compound inner type must be parenthesized: {}",
        opts
    );
}

#[test]
fn test_option_of_option_parenthesized() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct C {
        v: Option<Option<String>>,
    }
    let opts = C::nixos_options();
    assert!(
        opts.contains("types.nullOr (types.nullOr types.str)"),
        "compound inner type must be parenthesized: {}",
        opts
    );
}

// ── HashMap / BTreeMap (attrsOf) ────────────────────────────────────────

#[test]
fn test_hashmap_simple_no_parens() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct C {
        v: HashMap<String, String>,
    }
    let opts = C::nixos_options();
    assert!(
        opts.contains("types.attrsOf types.str"),
        "simple inner type should not be parenthesized: {}",
        opts
    );
    assert!(
        !opts.contains("types.attrsOf (types.str)"),
        "simple inner type should not be parenthesized: {}",
        opts
    );
}

#[test]
fn test_hashmap_of_vec_parenthesized() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct C {
        v: HashMap<String, Vec<String>>,
    }
    let opts = C::nixos_options();
    assert!(
        opts.contains("types.attrsOf (types.listOf types.str)"),
        "compound inner type must be parenthesized: {}",
        opts
    );
}

#[test]
fn test_hashmap_of_hashmap_parenthesized() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct C {
        v: HashMap<String, HashMap<String, u32>>,
    }
    let opts = C::nixos_options();
    assert!(
        opts.contains("types.attrsOf (types.attrsOf types.int)"),
        "compound inner type must be parenthesized: {}",
        opts
    );
}

#[test]
fn test_btreemap_of_vec_parenthesized() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct C {
        v: BTreeMap<String, Vec<String>>,
    }
    let opts = C::nixos_options();
    assert!(
        opts.contains("types.attrsOf (types.listOf types.str)"),
        "compound inner type must be parenthesized: {}",
        opts
    );
}

// ── HashSet / BTreeSet (listOf) ─────────────────────────────────────────

#[test]
fn test_hashset_simple_no_parens() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct C {
        v: HashSet<String>,
    }
    let opts = C::nixos_options();
    assert!(
        opts.contains("types.listOf types.str"),
        "simple inner type should not be parenthesized: {}",
        opts
    );
}

#[test]
fn test_hashset_of_vec_parenthesized() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct C {
        v: HashSet<Vec<String>>,
    }
    let opts = C::nixos_options();
    assert!(
        opts.contains("types.listOf (types.listOf types.str)"),
        "compound inner type must be parenthesized: {}",
        opts
    );
}

#[test]
fn test_btreeset_simple_no_parens() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct C {
        v: BTreeSet<String>,
    }
    let opts = C::nixos_options();
    assert!(
        opts.contains("types.listOf types.str"),
        "simple inner type should not be parenthesized: {}",
        opts
    );
}

// ── Nested struct (submodule) compound types ────────────────────────────

#[test]
fn test_option_of_struct_parenthesized() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct Inner {
        x: u32,
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct C {
        v: Option<Inner>,
    }
    let opts = C::nixos_options();
    // submodule { ... } contains a space → must be parenthesized
    assert!(
        opts.contains("types.nullOr (types.submodule"),
        "submodule compound type must be parenthesized in nullOr: {}",
        opts
    );
}

#[test]
fn test_vec_of_struct_parenthesized() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct Inner {
        x: u32,
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct C {
        v: Vec<Inner>,
    }
    let opts = C::nixos_options();
    // submodule { ... } contains a space → must be parenthesized
    assert!(
        opts.contains("types.listOf (types.submodule"),
        "submodule compound type must be parenthesized in listOf: {}",
        opts
    );
}

#[test]
fn test_hashmap_of_struct_parenthesized() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct Inner {
        x: u32,
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct C {
        v: HashMap<String, Inner>,
    }
    let opts = C::nixos_options();
    // submodule { ... } contains a space → must be parenthesized
    assert!(
        opts.contains("types.attrsOf (types.submodule"),
        "submodule compound type must be parenthesized in attrsOf: {}",
        opts
    );
}

// ── Complex mixed nesting ───────────────────────────────────────────────

#[test]
fn test_option_of_vec_of_hashmap() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct C {
        v: Option<Vec<HashMap<String, u32>>>,
    }
    let opts = C::nixos_options();
    // Should be: types.nullOr (types.listOf (types.attrsOf types.int))
    assert!(
        opts.contains("types.nullOr (types.listOf (types.attrsOf types.int))"),
        "deeply nested compound types must all be correctly parenthesized: {}",
        opts
    );
}

#[test]
fn test_vec_of_option_of_vec() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct C {
        v: Vec<Option<Vec<String>>>,
    }
    let opts = C::nixos_options();
    // Should be: types.listOf (types.nullOr (types.listOf types.str))
    assert!(
        opts.contains("types.listOf (types.nullOr (types.listOf types.str))"),
        "deeply nested compound types must all be correctly parenthesized: {}",
        opts
    );
}

#[test]
fn test_hashmap_of_option_parenthesized() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct C {
        v: HashMap<String, Option<String>>,
    }
    let opts = C::nixos_options();
    assert!(
        opts.contains("types.attrsOf (types.nullOr types.str)"),
        "compound inner type must be parenthesized: {}",
        opts
    );
}
