use num::complex::Complex;
use std::time::{Duration, Instant};

fn main() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im);

    let mut count = 0;
    let time_limit = Duration::new(1, 0);
    let start = Instant::now();

    while (Instant::now() - start) < time_limit {
        count += 1;
    }

    println!("{}", count);

    // ----

    let needle = 0o204;
    let haystack = [1, 1, 2, 3, 4, 5, 6, 42, 132, 429, 1430, 4862];

    for item in &haystack {
        let result = match item {
            42 | 32 => "hit",
            _ => "miss",
        };

        if result == "hit" {
            println!("{}: {}", item, result);
        }
    }

    for item in &haystack {
        if *item == needle {
            println!("found a match with needle: {}", item);
        }
    }
}
