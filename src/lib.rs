#![crate_name = "rust_cargo_starter"]
#![crate_type = "lib"]

use colored::*;
use dotenvy::dotenv;
use std::env;

/// This is the main entry point for `Rust and Cargo starter (RCS)`
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect(".env file not found");

    // TEST_MODE value can be changed in the .env file in the root of this project
    if env::var("TEST_MODE").unwrap_or_default() == "1" {
        println!("\n{} value: {}", "TEST_MODE".bright_cyan(), env::var("TEST_MODE").unwrap().to_string().bright_green());
        return Err("Simulated error\n".yellow().into());
    }

    let name = env::var("PROJECT_NAME").unwrap().to_string();
    println!("\nWelcome to {}", name.yellow());
    println!(
        "A starter Rust project to get you up and running.\nIt will be added to, refined and expanded over time."
    );

    Ok(())
}
