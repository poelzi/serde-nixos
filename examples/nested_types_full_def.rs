use serde::{Deserialize, Serialize};
use serde_nixos::NixosType;

#[derive(Serialize, Deserialize, NixosType)]
struct DatabaseConfig {
    #[nixos(description = "Database host")]
    host: String,

    #[nixos(description = "Database port", default = "5432")]
    port: u16,
}

#[derive(Serialize, Deserialize, NixosType)]
struct ServerConfig {
    #[nixos(description = "Server port", default = "8080")]
    port: u16,

    #[nixos(description = "Enable SSL")]
    ssl: bool,
}

#[derive(Serialize, Deserialize, NixosType)]
struct AppConfig {
    #[nixos(description = "Application name")]
    name: String,

    #[nixos(description = "Database configuration")]
    database: DatabaseConfig,

    #[nixos(description = "Server configuration")]
    server: ServerConfig,

    #[nixos(description = "Optional backup database")]
    backup_db: Option<DatabaseConfig>,
}

fn main() {
    println!("=== Full Definition with Nested Types ===\n");
    println!("{}", AppConfig::nixos_type_full_definition());

    println!("\n=== Just the Type Name ===\n");
    println!("{}", AppConfig::nixos_type_name());

    println!("\n=== Type Expression ===\n");
    println!("{}", AppConfig::nixos_type());
}
