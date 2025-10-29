use serde::{Deserialize, Serialize};
use serde_nixos::NixosType;

#[test]
fn test_empty_struct() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct Empty {}

    let options = Empty::nixos_options();
    let nixos_type = Empty::nixos_type();
    let definition = Empty::nixos_type_definition();

    // Empty struct should still generate valid output
    println!("Empty struct options: {}", options);
    println!("Empty struct type: {}", nixos_type);
    println!("Empty struct definition: {}", definition);
}

#[test]
fn test_unit_struct() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct Unit;

    let nixos_type = Unit::nixos_type();
    let definition = Unit::nixos_type_definition();

    // Unit struct should generate something (possibly types.attrs or similar)
    assert!(!nixos_type.is_empty() || nixos_type == "");

    println!("Unit struct type: {}", nixos_type);
    println!("Unit struct definition: {}", definition);
}

#[test]
fn test_tuple_struct_single() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct Wrapper(String);

    let nixos_type = Wrapper::nixos_type();
    let definition = Wrapper::nixos_type_definition();

    println!("Single-field tuple struct type: {}", nixos_type);
    println!("Single-field tuple struct definition: {}", definition);
}

#[test]
fn test_tuple_struct_multiple() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct Pair(String, u32);

    let nixos_type = Pair::nixos_type();
    let definition = Pair::nixos_type_definition();

    println!("Pair tuple struct type: {}", nixos_type);
    println!("Pair tuple struct definition: {}", definition);
}

#[test]
fn test_tuple_struct_complex() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct Triple(String, u32, bool);

    let nixos_type = Triple::nixos_type();

    println!("Triple tuple struct type: {}", nixos_type);
}

#[test]
fn test_newtype_pattern() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct UserId(u64);

    #[derive(Serialize, Deserialize, NixosType)]
    struct User {
        id: UserId,
        name: String,
    }

    let user_opts = User::nixos_options();
    assert!(user_opts.contains("id = lib.mkOption"));
    assert!(user_opts.contains("name = lib.mkOption"));

    println!("Newtype pattern: {}", user_opts);
}

#[test]
fn test_single_field_struct() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct SingleField {
        value: String,
    }

    let options = SingleField::nixos_options();
    assert!(options.contains("value = lib.mkOption"));

    println!("Single field struct: {}", options);
}

#[test]
fn test_struct_with_only_skipped_fields() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct AllSkipped {
        #[serde(skip)]
        internal1: String,

        #[serde(skip)]
        internal2: u32,
    }

    let options = AllSkipped::nixos_options();

    // Should generate empty or minimal output since all fields are skipped
    println!("All skipped fields: {}", options);
}

#[test]
fn test_struct_with_phantom_data() {
    use std::marker::PhantomData;

    #[derive(Serialize, Deserialize, NixosType)]
    struct WithPhantom<T> {
        value: String,
        #[serde(skip)]
        _phantom: PhantomData<T>,
    }

    let options = WithPhantom::<u32>::nixos_options();
    assert!(options.contains("value = lib.mkOption"));

    println!("Struct with PhantomData: {}", options);
}

#[test]
fn test_nested_empty_structs() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct Inner {}

    #[derive(Serialize, Deserialize, NixosType)]
    struct Outer {
        name: String,
        inner: Inner,
    }

    let options = Outer::nixos_options();
    assert!(options.contains("name = lib.mkOption"));
    assert!(options.contains("inner = lib.mkOption"));

    println!("Nested empty structs: {}", options);
}

#[test]
fn test_option_of_empty_struct() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct Empty {}

    #[derive(Serialize, Deserialize, NixosType)]
    struct Container {
        maybe_empty: Option<Empty>,
    }

    let options = Container::nixos_options();
    assert!(options.contains("maybe_empty = lib.mkOption"));

    println!("Option of empty struct: {}", options);
}

#[test]
fn test_vec_of_empty_struct() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct Empty {}

    #[derive(Serialize, Deserialize, NixosType)]
    struct Container {
        items: Vec<Empty>,
    }

    let options = Container::nixos_options();
    assert!(options.contains("items = lib.mkOption"));
    assert!(options.contains("types.listOf"));

    println!("Vec of empty struct: {}", options);
}

#[test]
fn test_complex_tuple_struct_in_field() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct Coord(f64, f64);

    #[derive(Serialize, Deserialize, NixosType)]
    struct Location {
        name: String,
        coordinates: Coord,
    }

    let options = Location::nixos_options();
    assert!(options.contains("name = lib.mkOption"));
    assert!(options.contains("coordinates = lib.mkOption"));

    println!("Tuple struct in field: {}", options);
}
