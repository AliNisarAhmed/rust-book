use dotenv::dotenv;
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    timestamp: usize,
    base: String,
    rates: HashMap<String, f64>,
}

static BASE_URL: &str = "https://openexchangerates.org/api";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv()?;
    let app_id: String = dotenv::var("APP_ID").unwrap();

    let stdin = std::io::stdin();

    let mut base_currency = String::new();
    let mut to_currency = String::new();
    let mut amount = String::new();

    println!("Enter the 3-digit code of the base currency (the currency you have)");

    stdin.read_line(&mut base_currency)?;
    let base_currency = base_currency.trim();

    println!("Enter the 3-digit code of the currency to convert to");

    stdin.read_line(&mut to_currency)?;
    let to_currency = to_currency.trim();

    println!("Enter the amount to convert");

    stdin.read_line(&mut amount)?;

    let amount: f64 = amount.trim().parse().unwrap();

    let resp = reqwest::blocking::get(format!(
        "{base_url}/latest.json?base={base_curr}&symbols={to_curr}&app_id={app_id}",
        base_url = BASE_URL,
        base_curr = base_currency,
        to_curr = to_currency,
        app_id = app_id
    ))?
    .json::<APIResponse>()?;

    println!(
        "{} {} equals {}",
        amount,
        base_currency,
        amount * resp.rates.get(&to_currency.to_uppercase()).unwrap()
    );

    Ok(())
}
