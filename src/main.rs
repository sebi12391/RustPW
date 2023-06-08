use std::io;

mod password_manager;

fn main() {
    // Get master password
    let master_password = password_manager::get_master_password();

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
            1 => password_manager::generate_password(),
            2 => password_manager::add_password(&master_password),
            3 => password_manager::remove_password(&master_password),
            4 => password_manager::display_passwords(&master_password),
            5 => {
                println!("Closing the program...");
                break;
            }
            _ => println!("Invalid command. Please enter a number between 1 and 5."),
        }
    }
}
