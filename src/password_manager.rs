use std::fs;
use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};

use rand::Rng;

use super::encryption;

const PASSWORD_FILE: &str = "passwords.txt";
const ENCRYPTED_PASSWORD_FILE: &str = "passwords.txt.SK";
const SILENT: bool = true;

struct PasswordEntry {
    website: String,
    username: String,
    password: String,
}

pub fn get_master_password() -> String {
    println!("Enter the master password:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    input.trim().to_owned()
}

//TEMPORARY PLEASE PROPERLY IMPLEMENT SOON
pub fn verify_master_password(master_password: &str) -> bool {
    // Decrypt
    let _ = encryption::decrypt_file(ENCRYPTED_PASSWORD_FILE, master_password, SILENT);

    // Check if the passwords.txt file exists
    let file_exists = OpenOptions::new()
        .read(true)
        .open(PASSWORD_FILE)
        .is_ok();

    if !file_exists {
        // Create the passwords.txt file
        let _file = match OpenOptions::new().create(true).write(true).open(PASSWORD_FILE) {
            Ok(file) => file,
            Err(error) => {
                println!("Error creating passwords file: {}", error);
                return false;
            }
        };
    }

    // Read the decrypted text from the file
    let decrypted_text = match fs::read_to_string(PASSWORD_FILE) {
        Ok(content) => content,
        Err(error) => {
            println!("Error reading passwords file: {}", error);
            let _ = encryption::encrypt_file(PASSWORD_FILE, master_password, SILENT);
            return false;
        }
    };

    // Check for garbage characters (NOT NEEDED SINCE ITS NOT VALID UTF-8 I think...)
    let garbage_char_list = vec!["Ô","Ó","¾","‰","™","®","¿","Ž","þ","Ç"];

    // Check if the decrypted text contains any garbage characters (NOT NEEDED SINCE ITS NOT VALID UTF-8 I think...)
    let contains_garbage = garbage_char_list.iter().any(|garbage| decrypted_text.contains(garbage));

    if contains_garbage {
        // Encrypt and return false
        let _ = encryption::encrypt_file(PASSWORD_FILE, master_password, SILENT);

        false
    } else {
        // Re-encrypt and return true
        let _ = encryption::encrypt_file(PASSWORD_FILE, master_password, SILENT);

        true
    }
}

pub fn generate_password() {
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

pub fn add_password(master_password: &str) {
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


    let _ = encryption::decrypt_file(ENCRYPTED_PASSWORD_FILE, master_password, SILENT);
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
    let _ = encryption::encrypt_file(PASSWORD_FILE, master_password, SILENT);
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


pub fn remove_password(master_password: &str) {
    println!("Enter the website:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    let website = input.trim().to_owned();

    let _ = encryption::decrypt_file(ENCRYPTED_PASSWORD_FILE, master_password, SILENT);
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
    let _ = encryption::encrypt_file(PASSWORD_FILE, master_password, SILENT);
}

pub fn display_passwords(master_password: &str) {
    let _ = encryption::decrypt_file(ENCRYPTED_PASSWORD_FILE, master_password, SILENT);
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
        let _ = encryption::encrypt_file(PASSWORD_FILE, master_password, SILENT);
    } else {
        println!("There are no passwords to display.");
    }
}
