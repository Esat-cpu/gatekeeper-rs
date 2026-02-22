use std::io::{self, Write};

fn main() {
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
        println!("2");
    }
}

