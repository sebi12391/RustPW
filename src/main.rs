use std::env;
use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};

const PASSWORD_FILE: &str = "passwords.txt";

fn main() {
    // Get master password
    let master_password = get_master_password();

    // Process user commands
    loop {
        println!("Enter a command (1-5):");
        println!("1. Generate a password");
        println!("2. Add password to file");
        println!("3. Remove password from file");
        println!("4. Display password list");
        println!("5. Close the program");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input.");
        let command = input.trim().parse::<u32>().unwrap_or(0);

        match command {
            1 => generate_password(),
            2 => add_password(&master_password),
            3 => remove_password(&master_password),
            4 => display_passwords(&master_password),
            5 => {
                println!("Closing the program...");
                break;
            }
            _ => println!("Invalid command. Please enter a number between 1 and 5."),
        }
    }
}

fn get_master_password() -> String {
    println!("Enter the master password:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    input.trim().to_owned()
}

fn generate_password() {
    println!("Generating a password...");

    // Your password generation logic goes here
    let generated_password = "MyGeneratedPassword123";

    println!("Generated password: {}", generated_password);
}

fn add_password(master_password: &str) {
    println!("Enter the password to add:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    let password = input.trim();

    if verify_master_password(master_password) {
        let mut file = OpenOptions::new()
            .create(true) // Create the file if it doesn't exist
            .append(true)
            .open(PASSWORD_FILE)
            .expect("Failed to open password file.");
        writeln!(file, "{}", password).expect("Failed to write to password file.");
        println!("Password added successfully.");
    } else {
        println!("Incorrect master password. Access denied.");
    }
}


fn remove_password(master_password: &str) {
    println!("Enter the password to remove:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    let password = input.trim();

    if verify_master_password(master_password) {
        let file = OpenOptions::new()
            .read(true)
            .open(PASSWORD_FILE)
            .expect("Failed to open password file.");
        let passwords: Vec<String> = io::BufReader::new(file)
            .lines()
            .map(|line| line.unwrap())
            .collect();

        let updated_passwords: Vec<String> = passwords
            .into_iter()
            .filter(|p| p != password)
            .collect();

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true) // Clear the file before rewriting
            .open(PASSWORD_FILE)
            .expect("Failed to open password file.");

        for password in updated_passwords {
            writeln!(file, "{}", password).expect("Failed to write to password file.");
        }

        println!("Password removed successfully.");
    } else {
        println!("Incorrect master password. Access denied.");
    }
}


fn display_passwords(master_password: &str) {
    if verify_master_password(master_password) {
        let file = OpenOptions::new()
            .read(true)
            .open(PASSWORD_FILE)
            .expect("Failed to open password file.");
        let passwords: Vec<String> = io::BufReader::new(file)
            .lines()
            .map(|line| line.unwrap())
            .collect();
        println!("Password list:");
        for password in passwords {
            println!("{}", password);
        }
    } else {
        println!("Incorrect master password. Access denied.");
    }
}

fn verify_master_password(master_password: &str) -> bool {
    // Your master password verification logic goes here
    // You can compare the given master_password with the expected value
    // For simplicity, we're assuming the master password is "password"
    master_password == "password"
}
