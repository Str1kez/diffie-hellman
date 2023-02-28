use std::{io, str::FromStr};

use num_bigint::BigUint;

mod tools;
use tools::miller_rabin::miller_rabin;

fn main() {
    let mut buf = String::new();
    let input_module = io::stdin();

    input_module.read_line(&mut buf).unwrap();
    let a = BigUint::from_str(buf.trim_end()).unwrap();
    let is_prime = miller_rabin(&a, 32);

    println!("a = {a}\nis_prime? {}", is_prime, a = a)
}
