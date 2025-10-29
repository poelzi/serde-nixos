use serde::{Deserialize, Serialize};
use serde_nixos::NixosType;

#[test]
fn test_recursive_tree_structure() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct TreeNode {
        value: String,

        #[nixos(description = "Child nodes")]
        children: Vec<TreeNode>,
    }

    let options = TreeNode::nixos_options();
    assert!(options.contains("value = lib.mkOption"));
    assert!(options.contains("children = lib.mkOption"));
    assert!(options.contains("types.listOf"));

    println!("Recursive tree structure: {}", options);
}

#[test]
fn test_optional_recursive_reference() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct LinkedNode {
        data: String,

        #[nixos(description = "Next node in chain")]
        next: Option<Box<LinkedNode>>,
    }

    let options = LinkedNode::nixos_options();
    assert!(options.contains("data = lib.mkOption"));
    assert!(options.contains("next = lib.mkOption"));

    println!("Optional recursive reference: {}", options);
}

#[test]
fn test_mutually_recursive_types() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct NodeA {
        name: String,
        b_ref: Option<Box<NodeB>>,
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct NodeB {
        value: u32,
        a_ref: Option<Box<NodeA>>,
    }

    let options_a = NodeA::nixos_options();
    assert!(options_a.contains("name = lib.mkOption"));
    assert!(options_a.contains("b_ref = lib.mkOption"));

    let options_b = NodeB::nixos_options();
    assert!(options_b.contains("value = lib.mkOption"));
    assert!(options_b.contains("a_ref = lib.mkOption"));

    println!("Mutually recursive A: {}", options_a);
    println!("Mutually recursive B: {}", options_b);
}

#[test]
fn test_recursive_with_multiple_self_refs() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct BinaryTree {
        value: i32,

        #[nixos(description = "Left subtree")]
        left: Option<Box<BinaryTree>>,

        #[nixos(description = "Right subtree")]
        right: Option<Box<BinaryTree>>,
    }

    let options = BinaryTree::nixos_options();
    assert!(options.contains("value = lib.mkOption"));
    assert!(options.contains("left = lib.mkOption"));
    assert!(options.contains("right = lib.mkOption"));

    println!("Binary tree: {}", options);
}

#[test]
fn test_deeply_nested_recursive() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct Folder {
        name: String,

        #[nixos(description = "Subfolder")]
        subfolders: Vec<Folder>,

        #[nixos(description = "File count")]
        file_count: u32,
    }

    let options = Folder::nixos_options();
    assert!(options.contains("name = lib.mkOption"));
    assert!(options.contains("subfolders = lib.mkOption"));
    assert!(options.contains("file_count = lib.mkOption"));

    let definition = Folder::nixos_type_definition();
    assert!(!definition.is_empty());

    println!("Folder structure options: {}", options);
    println!("Folder structure definition: {}", definition);
}

#[test]
fn test_recursive_with_enum() {
    #[derive(Serialize, Deserialize, NixosType)]
    enum Expression {
        Literal(i32),
        Variable(String),
        Add(Box<Expression>, Box<Expression>),
        Multiply(Box<Expression>, Box<Expression>),
    }

    let nixos_type = Expression::nixos_type();
    assert!(!nixos_type.is_empty());

    println!("Recursive enum expression: {}", nixos_type);
}

#[test]
fn test_indirect_recursion() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct Container {
        items: Vec<Item>,
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct Item {
        name: String,
        nested_container: Option<Box<Container>>,
    }

    let container_opts = Container::nixos_options();
    assert!(container_opts.contains("items = lib.mkOption"));

    let item_opts = Item::nixos_options();
    assert!(item_opts.contains("name = lib.mkOption"));
    assert!(item_opts.contains("nested_container = lib.mkOption"));

    println!("Indirect recursion - Container: {}", container_opts);
    println!("Indirect recursion - Item: {}", item_opts);
}
