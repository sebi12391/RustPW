# Rust Password Manager

A command-line password manager implemented in Rust. This program allows you to generate, store, and manage passwords securely. The password file is encrypted and can only be viewed when the program decrypts it with the master password.

## Features

- Generate a password
- Add password to file
- Remove password from file
- Display password list
- Close the program

## Getting Started

### Prerequisites

- Rust programming language (installation instructions: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install))

### Installation

1. Clone the repository:

   ```bash
   git clone <replace_with_repo_link>
```
2. Change to the Project Directory:
```bash
cd rust-password-manager
```
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
Make sure to include these dependencies in your `Cargo.toml` file.

### To-Do List
🔲️ Implement encryption and decryption of the password file.
🔲️ Improve error handling and user feedback.
☑️ Add unit tests for different functions.
☑️ Implement command-line arguments for customizing the program behavior.
☑️ Add support for exporting and importing password data.
