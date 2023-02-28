use std::ops::BitAnd;

use num_bigint::{BigUint, RandBigInt};
use num_traits::{FromPrimitive, One, Zero};

pub fn miller_rabin(num: &BigUint, rounds: u32) -> bool {
    let two = BigUint::from_u8(2u8).unwrap();
    let mut rng = rand::thread_rng();

    if *num == two {
        return true;
    }

    if BigUint::one().bitand(num) == BigUint::zero() {
        return false;
    }
    let mut t = num.clone() - 1u8;
    let mut s = 0u32;
    while BigUint::one().bitand(&t) == BigUint::zero() {
        t /= 2u8;
        s += 1;
    }

    for _ in 0..rounds {
        let a = rng.gen_biguint_range(&two, num);
        let mut x = a.modpow(&t, num);
        if x == BigUint::one() || x == num - 1u8 {
            continue;
        }
        for j in 0..s - 1 {
            x = x.modpow(&two, num);
            if x == BigUint::one() {
                return false;
            }
            if x == num - 1u8 {
                break;
            }
            if j == s - 2 {
                return false;
            }
        }
    }
    true
}
