fn main() {
    let stdin = std::io::stdin();

    let mut people = String::new();
    let mut pizzas = String::new();

    println!("How many people?");

    stdin.read_line(&mut people).unwrap();

    println!("How many pizzas do you have?");

    stdin.read_line(&mut pizzas).unwrap();

    let people = people
        .trim()
        .parse::<usize>()
        .expect("Please enter a number");
    let pizzas = pizzas
        .trim()
        .parse::<usize>()
        .expect("Please enter a number");

    let slices = pizzas * 8;
    let slices_per_person = slices / people;
    let leftovers = slices % people;

    println!("{} people with {} pizzas", people, pizzas);
    println!(
        "Each person gets {} {} of pizza.",
        slices_per_person,
        if slices_per_person <= 1 {
            "piece"
        } else {
            "pieces"
        }
    );
    println!("There are {} leftover pieces", leftovers);
}
