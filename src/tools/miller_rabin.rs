use num_bigint::{BigUint, RandBigInt};
use num_integer::Integer;
use num_traits::{FromPrimitive, One};

pub fn miller_rabin(num: &BigUint, rounds: u64) -> bool {
    let two = BigUint::from_u8(2u8).unwrap();
    let mut rng = rand::thread_rng();
    let mut t = num - 1u8;
    let mut s = 0u32;

    if *num == two {
        return true;
    }

    if num.is_even() {
        return false;
    }
    // if num & &BigUint::one() == BigUint::zero() {
    //     return false;
    // }

    // while &t & &BigUint::one() == BigUint::zero() {
    while t.is_even() {
        t >>= 1u8;
        s += 1;
    }

    for _ in 0..rounds {
        let a = rng.gen_biguint_range(&two, num);
        let mut x = a.modpow(&t, num);
        if x == BigUint::one() || x == num - 1u8 {
            continue;
        }
        if s == 1 {
            return false;
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
