#[cfg(test)]

mod tests {
    use assert_cmd::prelude::*;
    use dotenvy::dotenv;
    use std::process::Command;
    use std::env;

    #[test]
    fn test_run_with_project_test_mode() {
        dotenv().expect(".env file not found");
        let test_mode = env::var("TEST_MODE").unwrap().to_string();
        let output = Command::cargo_bin("rust_cargo_starter")
            .unwrap()
            .env(test_mode, "1")
            .output()
            .expect("Failed to execute command");

            println!("{:?}", output);
        // Assert that the command execution was not successful
        assert!(!output.status.success());

        // Assert that the error message was printed to stderr
        let stderr = String::from_utf8(output.stderr).unwrap();
        println!("{:?}", stderr);
        assert!(stderr.contains("Error running Rust Cargo Starter (RCS): Simulated error"));
    }
}
