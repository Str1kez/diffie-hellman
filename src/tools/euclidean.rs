use std::mem::swap;

use num_bigint::BigUint;
use num_traits::Zero;

pub fn euclidean_algorithm(a: &BigUint, b: &BigUint) -> BigUint {
    let zero = BigUint::zero();
    let mut temp_a = a.clone();
    let mut temp_b = b.clone();
    let mut helper: BigUint;

    if a < b {
        swap(&mut temp_a, &mut temp_b);
    }

    while temp_b != zero {
        helper = temp_b.clone();
        temp_b = temp_a % temp_b;
        temp_a = helper;
    }

    temp_a
}
