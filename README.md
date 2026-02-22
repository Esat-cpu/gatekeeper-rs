# gatekeeper

A simple CLI authentication system written in Rust.
Stores usernames and Argon2 password hashes in a CSV file.

---

## Installation
```bash
git clone https://github.com/Esat-cpu/gatekeeper-rs.git
cd gatekeeper-rs
cargo build --release
cargo run
```

## Usage

Run the program and choose to login or sign up.
Credentials are stored in `vault.csv`.

