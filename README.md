

# RustPW

A command-line password manager implemented in Rust. This program allows you to generate, store, and manage passwords securely. The password file is encrypted and can only be viewed when the program decrypts it with the master password.
**Note: This was written for a school final project, this shouldnt be used as real password manager as there's flaws in the security of this program.**

## Features

- Generate a password
- Add password to file
- Remove password from file
- Display password list
- Encryption based off [YAFO](https://lib.rs/crates/yafo)

## Getting Started

### Usage
1. Compile and run the program:
```bash
cargo build
cargo run
```
2. Follow the instructions in the program to perform various operations:

Generate a password
Add password to file
Remove password from file
Display password list
Close the program

## Dependencies
This program relies on the following external crates:
anyhow = "1.0.71"
rand = "0.8.5"
yafo = "0.1.1"
