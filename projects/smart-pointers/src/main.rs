fn main() {
    println!("Hello, world!");
}

enum List {
    Cons(i32, Box<List>),
    Nil
}
