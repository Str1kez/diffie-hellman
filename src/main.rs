use tools::{get_primitive_root, get_safe_prime};

mod tools;

fn main() {
    let bit_len = 512u64;
    let safe_prime = get_safe_prime(bit_len);
    println!("safe prime = {}", safe_prime);
    let primitive_root = get_primitive_root(bit_len, &safe_prime);
    println!("primitive root = {}", primitive_root)
}
