use std::mem::transmute;

const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

fn main() {
    // let a: u16 = 50115;
    // let b: i16 = -15421;

    // println!("a: {:016b} {}", a, a);
    // println!("b: {:016b} {}", b, b);

    // --------------------------------------

    let a: f32 = 42.42;

    let frankentype: u32 = unsafe { std::mem::transmute(a) };

    println!("frankentype: {}", frankentype);
    println!("frankentype: {:032b}", frankentype);

    let b: f32 = unsafe { std::mem::transmute(frankentype) };
    println!("b: {}", b);
    assert_eq!(a, b);

    // ----------------------------------------------

    // let mut i: u16 = 0;
    // println!("{}..", i);

    // loop {
    //     i += 1000;
    //     println!("{}...", i);
    //     if i % 10000 == 0 {
    //         println!("\n")
    //     }
    // }

    // ----- Endianness

    let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
    let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];

    let a: i32 = unsafe { transmute(big_endian) };
    let b: i32 = unsafe { transmute(little_endian) };

    println!("{} vs {}", a, b);

    // -- isolate the sign bit

    let n: f32 = 42.42;
    let n_bits: u32 = n.to_bits();
    let sign_bit = n_bits >> 31;
    println!("sign bit: {}", sign_bit);

    // isolate the exponent

    let exponent = n_bits >> 23;
    let exponent = exponent & 0xff;
    let exponent = (exponent as i32) - 127;
    println!("exponent: {}", exponent);

    // isolating the mantissa - non special cases
    let mut mantissa: f32 = 1.0;

    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = n_bits & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }

    println!("mantissa: {}", mantissa);

    // ----- Deconstructing a floating point value

    let n: f32 = 42.42;

    let (sign, exp, frac) = to_parts(n);
    let (sign_, exp_, mant) = decode(sign, exp, frac);
    let n_ = from_parts(sign_, exp_, mant);

    println!("{} -> {}", n, n_);
    println!("field     | as bits | as real number");
    println!("sign      |   {:0b} | {}", sign, sign_);
    println!("exponent  | {:08b}  | {}", exp, exp_);
    println!("mantiassa | {:023b} | {}", frac, mant);
}

fn to_parts(n: f32) -> (u32, u32, u32) {
    let bits = n.to_bits();

    let sign = (bits >> 31) & 1;
    let exponent = (bits >> 23) & 0xff;
    let fraction = bits & 0x7fffff;

    (sign, exponent, fraction)
}

fn decode(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
    let signed_1 = (-1.0_f32).powf(sign as f32);

    let exponent = (exponent as i32) - BIAS;
    let exponent = RADIX.powf(exponent as f32);

    let mut mantissa: f32 = 1.0;

    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = fraction & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }

    (signed_1, exponent, mantissa)
}

fn from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
    sign * exponent * mantissa
}
