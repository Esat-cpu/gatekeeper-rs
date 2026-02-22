use std::io::{self, Write};
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use rand::thread_rng;
use std::fs::OpenOptions;


fn main() {
    // User chooses to login or sign up
    let choice: String = loop {
        let mut choice = String::new();

        print!("Login(1) Sign up(2) Exit(3): ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut choice)
            .unwrap();

        let choice = choice.trim();

        match choice {
            "1" | "2" => break choice.to_string(),
            "3" => return,
            _ => println!("Enter '1', '2' or '3'."),
        }
    };



    if choice == "1" {
        println!("1");
    }



    else if choice == "2" {
        // Get username from user
        let username: String = loop {
            let mut input = String::new();

            print!("Enter your username: ");
            io::stdout().flush().unwrap();

            io::stdin()
                .read_line(&mut input)
                .unwrap();

            let input = input.trim();


            if input.len() < 2 || input.len() > 20 {
                eprintln!("Username must be between 2 and 20 characters.");
                continue;
            }

            if !input.chars().all(|c| c.is_alphanumeric() || c == '_') {
                eprintln!("Username can only contain letters, numbers and '_'.");
                continue;
            }

            break input.to_string();
        };


        // Get password from user
        let password: String = loop {
            let pass = rpassword::prompt_password("Enter your password: ").unwrap();

            if pass.len() < 6 || pass.len() > 50 {
                eprintln!("Password must be between 6 and 50 characters.");
                continue;
            }

            break pass;
        };


        // Hashing the password
        let salt = SaltString::generate(&mut thread_rng());

        let hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();


        // Write the username and the password hash to the file
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("vault.csv")
            .unwrap();

        writeln!(&mut file, "{}::{}", username, hash).unwrap();
        println!("Sign up successful!");
    }
}

