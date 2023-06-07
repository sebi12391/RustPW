use std::env;
use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};

use rand::Rng;

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
    println!("Enter the password size:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    let size: usize = input.trim().parse().expect("Invalid size input.");

    println!("Allow symbols? (y/n)");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    let allow_symbols = input.trim().eq_ignore_ascii_case("y");

    let password: String = (0..size)
        .map(|_| generate_random_char(allow_symbols))
        .collect();

    println!("Generated password: {}", password);
}

fn generate_random_char(allow_symbols: bool) -> char {
    let mut rng = rand::thread_rng();

    let chars_lower = "abcdefghijklmnopqrstuvwxyz";
    let chars_upper = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let chars_digits = "0123456789";
    let chars_symbols = "!#$%^&";

    let mut chars: String = format!("{}{}", chars_lower, chars_upper);
    if allow_symbols {
        chars.push_str(chars_digits);
        chars.push_str(chars_symbols);
    } else {
        chars.push_str(chars_digits);
    }

    let chars_len = chars.len();
    let idx = rng.gen_range(0..chars_len);
    chars.chars().nth(idx).unwrap()
}


struct PasswordEntry {
    website: String,
    username: String,
    password: String,
}

fn add_password(master_password: &str) {
    println!("Enter the website:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    let website = input.trim().to_owned();

    println!("Enter the username:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    let username = input.trim().to_owned();

    println!("Enter the password:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    let password = input.trim().to_owned();

    if verify_master_password(master_password) {
        let password_entry = PasswordEntry {
            website,
            username,
            password,
        };

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(PASSWORD_FILE)
            .expect("Failed to open password file.");

        writeln!(file, "{},{},{}", password_entry.website, password_entry.username, password_entry.password)
            .expect("Failed to write to password file.");

        println!("Password added successfully.");
    } else {
        println!("Incorrect master password. Access denied.");
    }
}

fn parse_password_entry(line: &str) -> Option<PasswordEntry> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() == 3 {
        let website = parts[0].to_owned();
        let username = parts[1].to_owned();
        let password = parts[2].to_owned();
        Some(PasswordEntry {
            website,
            username,
            password,
        })
    } else {
        None
    }
}

fn remove_password(master_password: &str) {
    println!("Enter the website:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    let website = input.trim().to_owned();

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
            .filter(|line| {
                let entry = parse_password_entry(line);
                entry.is_some() && entry.unwrap().website != website
            })
            .collect();

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(PASSWORD_FILE)
            .expect("Failed to open password file.");

        for password in updated_passwords {
            writeln!(file, "{}", password).expect("Failed to write to password file.");
        }

        println!("Passwords for the website '{}' removed successfully.", website);
    } else {
        println!("Incorrect master password. Access denied.");
    }
}


fn display_passwords(master_password: &str) {
    if verify_master_password(master_password) {
        let file_exists = OpenOptions::new()
            .read(true)
            .open(PASSWORD_FILE)
            .is_ok();

        if file_exists {
            let file = OpenOptions::new()
                .read(true)
                .open(PASSWORD_FILE)
                .expect("Failed to open password file.");

            let passwords: Vec<String> = io::BufReader::new(file)
                .lines()
                .map(|line| line.unwrap())
                .collect();

            if passwords.is_empty() {
                println!("There are no passwords to display.");
            } else {
                println!("Password List:");
                for password in passwords {
                    println!("{}", password);
                }
            }
        } else {
            println!("There are no passwords to display.");
        }
    }else { println!("Incorrect master password. Access denied."); }
}

fn verify_master_password(master_password: &str) -> bool {
    // Your master password verification logic goes here
    // You can compare the given master_password with the expected value
    // For simplicity, we're assuming the master password is "password"
    master_password == "password"
}
