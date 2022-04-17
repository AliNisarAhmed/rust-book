use std::io;

fn main() {
    println!("What is the quote? ");

    let stdin = io::stdin();

    let mut quote = String::new();

    stdin.read_line(&mut quote).unwrap();

    let quote = quote.trim();

    println!("Who said it? ");

    let mut author = String::new();

    stdin.read_line(&mut author).unwrap();

    let author = author.trim();

    println!("{} says, \"{}\".", author, quote);


}
