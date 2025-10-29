use serde::{Deserialize, Serialize};
use serde_nixos::NixosType;

#[test]
fn test_enum_with_tuple_variants() {
    #[derive(Serialize, Deserialize, NixosType)]
    enum Value {
        None,
        Single(String),
        Pair(String, u32),
        Triple(String, u32, bool),
    }

    let nixos_type = Value::nixos_type();

    // Enum with data should generate an attrs type or enum type
    // depending on the serialization format
    assert!(!nixos_type.is_empty());

    // The type should be generated (exact format depends on implementation)
    println!("Tuple variants enum type: {}", nixos_type);
}

#[test]
fn test_enum_with_struct_variants() {
    #[derive(Serialize, Deserialize, NixosType)]
    enum Config {
        Simple,

        Advanced {
            #[nixos(description = "Enable feature")]
            enabled: bool,

            #[nixos(description = "Configuration value")]
            value: u32,
        },

        Full {
            name: String,
            count: u32,
            active: bool,
        },
    }

    let nixos_type = Config::nixos_type();

    // Should generate appropriate type for struct variants
    assert!(!nixos_type.is_empty());

    println!("Struct variants enum type: {}", nixos_type);
}

#[test]
fn test_enum_with_mixed_variants() {
    #[derive(Serialize, Deserialize, NixosType)]
    enum Action {
        // Unit variant
        Noop,

        // Tuple variant
        Execute(String),

        // Struct variant
        Configure {
            name: String,
            value: String,
        },
    }

    let nixos_type = Action::nixos_type();
    assert!(!nixos_type.is_empty());

    let definition = Action::nixos_type_definition();
    assert!(!definition.is_empty());

    println!("Mixed variants enum type: {}", nixos_type);
    println!("Mixed variants enum definition: {}", definition);
}

#[test]
fn test_nested_enum_with_data() {
    #[derive(Serialize, Deserialize, NixosType)]
    enum Inner {
        A,
        B(u32),
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct Outer {
        #[nixos(description = "Inner enum field")]
        inner: Inner,

        name: String,
    }

    let options = Outer::nixos_options();
    assert!(options.contains("inner = lib.mkOption"));
    assert!(options.contains("name = lib.mkOption"));

    println!("Nested enum with data: {}", options);
}

#[test]
fn test_option_enum_with_data() {
    #[derive(Serialize, Deserialize, NixosType)]
    enum Status {
        Active,
        Paused(String), // reason
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct ServiceState {
        status: Option<Status>,
    }

    let options = ServiceState::nixos_options();
    assert!(options.contains("status = lib.mkOption"));

    println!("Optional enum with data: {}", options);
}

#[test]
fn test_vec_of_enum_with_data() {
    #[derive(Serialize, Deserialize, NixosType)]
    enum Task {
        Simple(String),
        Complex { name: String, priority: u32 },
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct TaskList {
        tasks: Vec<Task>,
    }

    let options = TaskList::nixos_options();
    assert!(options.contains("tasks = lib.mkOption"));
    assert!(options.contains("types.listOf"));

    println!("Vec of enum with data: {}", options);
}

#[test]
fn test_enum_with_complex_data() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct DatabaseConfig {
        host: String,
        port: u16,
    }

    #[derive(Serialize, Deserialize, NixosType)]
    enum Backend {
        None,
        Memory,
        Database(DatabaseConfig),
        Custom {
            name: String,
            config: DatabaseConfig,
        },
    }

    let nixos_type = Backend::nixos_type();
    assert!(!nixos_type.is_empty());

    let definition = Backend::nixos_type_definition();
    assert!(!definition.is_empty());

    println!("Enum with complex data type: {}", nixos_type);
    println!("Enum with complex data definition: {}", definition);
}
