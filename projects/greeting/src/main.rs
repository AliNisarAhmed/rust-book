use std::io;

fn main() {
    println!("What is your name? ");
    let mut name = String::new();

    let stdin = io::stdin();

    stdin.read_line(&mut name).expect("Could not read name");

    let name = name.trim();

    println!("Hello, {}, nice to meet you!", name);
}
