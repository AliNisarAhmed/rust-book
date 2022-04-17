fn main() {
    println!("What is the first number? ");

    let stdin = std::io::stdin();

    let mut n1 = String::new();

    stdin.read_line(&mut n1).unwrap();

    let n1: isize = n1.trim().parse().expect("Please enter a number");

    println!("What is the second number");

    let mut n2 = String::new();

    stdin.read_line(&mut n2).unwrap();

    let n2: isize = n2.trim().parse().expect("Please enter a number");

    println!(
        "{} + {} = {}
{} - {} = {}
{} * {} = {}
{} / {} = {:.3}
    ",
        n1,
        n2,
        n1 + n2,
        n1,
        n2,
        n1 - n2,
        n1,
        n2,
        n1 * n2,
        n1,
        n2,
        (n1 as f64) / (n2 as f64)
    );
}
