// Copyright © 2024 Rust and Cargo starter (RCS). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! This is the main entry point for the RCS application.
use dotenvy::dotenv;
use std::env;

fn main() {
  dotenv().expect(".env file not found");
  // Call the `run()` function from the Rust and Cargo starter (RCS)` module.
  let project_name = env::var("PROJECT_NAME").unwrap().to_string();

  if let Err(err) = rcs::run() {
      eprintln!("Error running {}: {}", project_name, err);
      std::process::exit(1);
  }
}
