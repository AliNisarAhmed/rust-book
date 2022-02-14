use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let y = 12;

    let y = y + 5;

    let spaces = "   ";
    let spaces = spaces.len();

    // let overflow: i8 = 256; // overflow error

    let x = 2.0;

    // let g: f32 = 4 / 2;

    let t: (i32, f32, char) = (23, 24.5, 'a');

    t.0;
    t.1;
    t.2;
    // t.3; // error

    let a = [1, 2, 3, 4, 5];

    let mut index = String::new();

    println!("Please enter an array index");

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];

    println!(
        "The value of the element at index {} is : {}",
        index, element
    );

    let p = if true { 5 } else { 6 };

    for element in a {
        println!("The value is {}", element);
    }
}
