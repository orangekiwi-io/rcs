# Rust and Cargo Starter (RCS) project

## Description

A starter Rust project to get you up and running. It will be added to, refined and expanded over time. This little project is just to get up-and-running a little quicker and form, hopefully, good habits.

## Getting Started

Make sure that you have **Rust** (version used at time of writing was **1.76.0**) and associated software installed.
* [Rust installation](https://doc.rust-lang.org/book/ch01-01-installation.html)

### Dependencies

* Rust programming language
* Cargo, Rust's package manager

### Installation

Provide step-by-step instructions on how to get a development environment running.

1. Clone the repository to your local machine:
```bash
git clone https://github.com/orangekiwi-io/rcs.git
```
2. In a terminal window, in the root of the local repository directory, run the following to compile the code:
```bash
cargo build
```

3. To see the Welcome message, and to check everything is good run the following:
```bash
cargo run
```
Welcome message:
```
Welcome to Rust and Cargo starter (RCS)
A starter Rust project to get you up and running.
It will be added to, refined and expanded over time.
```
## Modifying the project
Usually you would not add a ```.env``` file to a repository (bad practise), but it is added here to help with setting up this starter.

### Cargo.toml
Change the ```name``` value to something more useful. Remember that Rust uses snake_case, so follow that naming convention.

### src/lib.rs
Change the ```crate_name``` to match the value you changed in ```Cargo.toml```.

### src/main.rs
Change ```rcs::run()``` to match the value you changed in ```Cargo.toml```. For example, if you changed the crate name to ```bob``` the code would become ```bob::run()```.

### .env
Change the ```PROJECT_NAME``` value to the name of your project.

To see testing working, change ```TEST_MODE``` to 1.

### tests/test_main.rs
Change ```cargo_bin("rcs")``` to match the value you changed in ```Cargo.toml```. For example, if you changed the crate name to ```bob``` the code would become ```cargo_bin("bob")```.

Change ```assert!(stderr.contains("Error running Rust Cargo Starter (RCS): Simulated error"));``` to
```assert!(stderr.contains("Error running Project Bob: Simulated error"));``` (the Project name you changed in the ```.env``` file).

### Run the test function
Once cargo says everything is good (via ```cargo build```), you can run the test.

To test a specific test function, use the entire test function name. Rust/Cargo tests run tests on a pattern matching basis. This means if you have lost of tests starting with ```test_``` and you just run that it will run all tests that match that pattern.

To test your changes run this:
```cargo test test_run_with_project_test_mode```

Don't forget to change the ```TEST_MODE``` value back to 0 in your ```.env``` file otherwise you will be met with the message in the ```src/main.rs``` file.

## .gitignore
Should you use this starter as a, well starter project, then you **MUST** add ```.env``` to your ```.gitignore``` file because you do not want to commit all your private data (such as API keys) to a repository.

## Shout out
Shout out the [@sebastienrousseau/](https://github.com/sebastienrousseau/) for pointing me towards Rust. Seb, I'm getting there slowly and this little repository was born from me starting to refactor another project that was/is not very well structured. 😄

## License

The project is licensed under the terms of both the MIT license and the Apache License (Version 2.0).

* [Apache License, Version 2.0](https://opensource.org/license/apache-2-0/b)
* [MIT license](https://opensource.org/licenses/MIT)