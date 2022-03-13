fn parity_bit(bytes: &[u8]) -> u8 {
    let mut number_of_ones: u32 = 0;

    for byte in bytes {
        let ones = byte.count_ones();
        number_of_ones += ones;
        println!("{} (0b{:08b}) has {} bits", byte, byte, ones);
    }

    (number_of_ones % 2 == 0) as u8
}

fn main() {
    let abc = b"abc";
    println!("input: {:?}", abc);
    println!("output: {:08x}", parity_bit(abc));
    println!();

    let abcd = b"abcd";
    println!("input: {:?}", abcd);
    println!("output: {:08x}", parity_bit(abcd));
}
