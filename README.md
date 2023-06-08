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

2. Change to the project directory:

```bash
cd rust-password-manager
```
3. Usage

1. Compile and Run the program:

```bash

cargo build
cargo run
```
Run the program:

3.
### Follow the instructions in the program to perform various operations:

1. Generate a password
2. Add password to file
3. Remove password from file
4. Display password list
5. Close the program
## Dependencies
This program does not rely on external crates or dependencies. It uses the standard libraries provided by Rust.

### To-Do List
🔲 Implement encryption and decryption of the password file.
🔲️ Improve error handling and user feedback.
☑️Add unit tests for different functions.
☑️ Implement command-line arguments for customizing the program behavior.
☑️ Add support for exporting and importing password data.
## Contributing
Contributions are welcome! If you find any issues or have suggestions for improvement, please open an issue or submit a pull request.

## License
This project is licensed under the MIT License.
