use chrono::Datelike;

fn main() {
    println!("What is your current age? ");
    let stdin = std::io::stdin();
    let mut age = String::new();

    stdin.read_line(&mut age).unwrap();

    let age = age
        .trim()
        .parse::<u32>()
        .expect("Please enter a valid number as age");

    let mut retirement_age = String::new();
    println!("At what age would you like to retire ");

    stdin.read_line(&mut retirement_age).unwrap();

    let retirement_age = retirement_age
        .trim()
        .parse::<u32>()
        .expect("Please enter a valid number as retiremement age");
    let current_year = chrono::Utc::now().year();

    if retirement_age <= age {
        println!("You can already retire!");
    } else {
        println!(
            "You have {} years left till you can retire",
            retirement_age - age
        );
        println!(
            "It's {}, so you can retire in {}.",
            current_year,
            current_year as u32 + retirement_age - age
        );
    }
}
