use std::io::{self, Write};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;
use rand::thread_rng;
use std::fs::{self, OpenOptions};


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
        // Check if vault.csv does not exist
        if !std::path::Path::new("vault.csv").exists() {
            eprintln!("No users found.");
            return;
        }

        // Get username
        let mut username = String::new();

        print!("Enter your username: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut username)
            .unwrap();

        let username = username.trim();


        // Get password
        let password = rpassword::prompt_password("Enter your password: ")
            .unwrap();


        // Read the file
        let content = fs::read_to_string("vault.csv").unwrap();

        // Initialize hash as None
        let mut hash: Option<&str> = None;

        for line in content.lines() {
            // split line with '::'
            let mut parts = line.split("::");
            let f_username = parts.next().unwrap();

            // Get the hash part if username matches
            if f_username == username {
                hash = Some(parts.next().unwrap());
                break;
            }
        }

        match hash {
            Some(h) => {
                let parsed_hash = PasswordHash::new(h)
                    .unwrap();

                let result = Argon2::default()
                    .verify_password(password.as_bytes(), &parsed_hash);


                if result.is_ok() {
                    println!("Correct!");
                }
                else {
                    println!("Wrong!");
                }
            },
            None => {
                eprintln!("No user with that name was found.");
            },
        }
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
            let pass = rpassword::prompt_password("Enter your password: ")
                .unwrap();

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

