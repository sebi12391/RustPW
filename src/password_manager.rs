use std::fs;
use std::io;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use crate::encryption::{encrypt_file, decrypt_file, verify_master_password};

const YAFO_FILE_EXTENSION: &str = ".SK";

pub struct PasswordManager {
    password_file: String,
    encrypted_file: String,
}

impl PasswordManager {
    pub fn new(password_file: &str, encrypted_file: &str) -> Self {
        PasswordManager {
            password_file: password_file.to_owned(),
            encrypted_file: encrypted_file.to_owned(),
        }
    }

    pub fn get_master_password(&self) -> String {
        println!("Enter the master password:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input.");
        input.trim().to_owned()
    }

    pub fn generate_password(&self) {
        println!("Enter the password size:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input.");
        let size: usize = input.trim().parse().expect("Invalid size input.");

        println!("Allow symbols? (y/n)");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input.");
        let allow_symbols = input.trim().eq_ignore_ascii_case("y");

        let password: String = (0..size)
            .map(|_| self.generate_random_char(allow_symbols))
            .collect();

        println!("Generated password: {}", password);
    }

    fn generate_random_char(&self, allow_symbols: bool) -> char {
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

    pub fn add_password(&self, master_password: &str) {
        decrypt_file(&self.encrypted_file, master_password, true);

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
                .open(&self.password_file)
                .expect("Failed to open password file.");

            writeln!(file, "{},{},{}", password_entry.website, password_entry.username, password_entry.password)
                .expect("Failed to write to password file.");

            println!("Password added successfully.");
            encrypt_file(&self.password_file, master_password, true);
        } else {
            println!("Incorrect master password. Access denied.");
        }
    }

    pub fn remove_password(&self, master_password: &str) {
        println!("Enter the website:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input.");
        let website = input.trim().to_owned();

        if verify_master_password(master_password) {
            decrypt_file(&self.encrypted_file, master_password, true);
            let file = OpenOptions::new()
                .read(true)
                .open(&self.password_file)
                .expect("Failed to open password file.");

            let passwords: Vec<String> = io::BufReader::new(file)
                .lines()
                .map(|line| line.unwrap())
                .collect();

            let updated_passwords: Vec<String> = passwords
                .into_iter()
                .filter(|line| {
                    let entry = self.parse_password_entry(line);
                    entry.is_some() && entry.unwrap().website != website
                })
                .collect();

            let mut file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&self.password_file)
                .expect("Failed to open password file.");

            for password in updated_passwords {
                writeln!(file, "{}", password).expect("Failed to write to password file.");
            }

            println!("Passwords for the website '{}' removed successfully.", website);
            encrypt_file(&self.password_file, master_password, true);
        } else {
            println!("Incorrect master password. Access denied.");
        }
    }

    pub fn display_passwords(&self, master_password: &str) {
        if verify_master_password(master_password) {
            decrypt_file(&self.encrypted_file, master_password, true);
            let file_exists = Path::new(&self.password_file).exists();

            if file_exists {
                let file = OpenOptions::new()
                    .read(true)
                    .open(&self.password_file)
                    .expect("Failed to open password file.");

                println!("Passwords:");
                for line in io::BufReader::new(file).lines() {
                    println!("{}", line.unwrap());
                }
            } else {
                println!("No passwords found.");
            }

            encrypt_file(&self.password_file, master_password, true);
        } else {
            println!("Incorrect master password. Access denied.");
        }
    }

    fn parse_password_entry(&self, line: String) -> Option<PasswordEntry> {
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
}

pub struct PasswordEntry {
    pub website: String,
    pub username: String,
    pub password: String,
}
