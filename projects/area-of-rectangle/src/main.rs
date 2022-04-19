const FACTOR: f64 = 0.09290304;

fn main() {
    let stdin = std::io::stdin();

    let mut length = String::new();
    let mut width = String::new();

    println!("What is the length of the room in feet?");

    stdin.read_line(&mut length).unwrap();

    println!("What is the width of the room in feet?");

    stdin.read_line(&mut width).unwrap();

    let length = length
        .trim()
        .parse::<usize>()
        .expect("Please enter a positive number");
    let width = width
        .trim()
        .parse::<usize>()
        .expect("Please enter a positive number");

    let area = length * width;
    let area_sq_meter = (area as f64) * FACTOR;

    println!(
        "You entered dimensions of {} feet by {} feet.",
        length, width
    );

    println!("The area is");
    println!("{} square feet", area);
    println!("{:.3} square meters", area_sq_meter);
}
