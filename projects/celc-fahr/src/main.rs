use std::fmt::Display;
use std::io;

fn main() {
    println!("Convert Celcius to Fahrenheit or vice versa");

    println!("First select the input temperature unit");
    println!("type c for Celsius, f for Fahrenheit, defaults to Celsius");

    let mut mode = String::new();

    let stdin = io::stdin();
    stdin
        .read_line(&mut mode)
        .expect("Could not read Temperature unit");

    let mode = mode.trim();

    let mode: Temperature = if mode == "f" || mode == "F" {
        Temperature::Fahrenheit
    } else {
        Temperature::Celsius
    };

    // print_unit(&mode);

    println!("Enter a temperature in {}: ", mode);

    let mut temp = String::new();

    stdin
        .read_line(&mut temp)
        .expect("Could not read temp");

    loop {
        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        let res = cels_to_fahr(temp);

        println!("Result = {} Deg Celsius in Fahrenheit is: {}", temp, res);

        break;
    }
}

enum Temperature {
    Celsius,
    Fahrenheit,
}

impl Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self)
    }
}

fn cels_to_fahr(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn fahr_to_cels(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn print_unit(t: &Temperature) {
    match t {
        Temperature::Celsius => println!("Current unit is Celsius"),
        _ => println!("Current unit is Fahrenheit"),
    }
}
