use std::collections::HashMap;

const TAX_RATE: f64 = 0.055;

fn main() {
    let stdin = std::io::stdin();

    let mut store: HashMap<String, (usize, f64)> = HashMap::new();
    let mut item_count = 1;

    loop {
        let mut name = String::new();
        let mut quantity = String::new();
        let mut price = String::new();

        println!("Enter the name of item #{}: ", item_count);

        stdin.read_line(&mut name).unwrap();

        println!("Enter the quantity of item #{}", item_count);

        stdin.read_line(&mut quantity).unwrap();

        println!("Enter the price of item #{}", item_count);

        stdin.read_line(&mut price).unwrap();

        let name = name.trim().to_owned();

        if name.is_empty() {
            panic!("item name cannot be empty")
        }
        let quantity = quantity
            .trim()
            .parse::<usize>()
            .expect("quantity should be a number");
        let price = price
            .trim()
            .parse::<f64>()
            .expect("price should be a number");

        store.insert(name, (quantity, price));

        let mut resp = String::new();

        println!("Do you want to add another item? (y/n)");

        stdin.read_line(&mut resp).unwrap();

        let resp = resp.trim();

        if resp.is_empty() || resp == "n" || resp == "N" {
            break;
        }

        item_count += 1;
    }

    let sub_total: f64 = calculate_subtotal(store);

    let tax: f64 = calculate_tax(sub_total);

    let total: f64 = tax + sub_total;

    println!("Subtotal: ${:.2}", sub_total);
    println!("Tax: ${:.2}", tax);
    println!("Total: ${:.2}", total);
}

pub fn calculate_subtotal(store: HashMap<String, (usize, f64)>) -> f64 {
    store
        .iter()
        .fold(0.0, |acc, (_, (q, p))| acc + (*q as f64) * (*p as f64))
}

pub fn calculate_tax(sub_total: f64) -> f64 {
    sub_total * TAX_RATE
}
