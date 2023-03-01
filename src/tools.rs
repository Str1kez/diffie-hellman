use num_bigint::{BigUint, RandBigInt};
use num_traits::One;

use self::{euclidean::euclidean_algorithm, miller_rabin::miller_rabin};

mod euclidean;
mod miller_rabin;

pub fn get_safe_prime(bit_len: u64) -> BigUint {
    let mut rng = rand::thread_rng();
    let mut x = rng.gen_biguint(bit_len);
    while !miller_rabin(&x, bit_len) || !miller_rabin(&((&x - 1u8) / 2u8), bit_len - 1) {
        x = rng.gen_biguint(bit_len);
    }
    x
}

pub fn get_primitive_root(bit_len: u64, safe_prime: &BigUint) -> BigUint {
    let euler = safe_prime - 1u8;
    let mut rng = rand::thread_rng();
    let mut g = rng.gen_biguint(bit_len);
    loop {
        if g.modpow(&euler, safe_prime) == BigUint::one()
            && euclidean_algorithm(&g, safe_prime) == BigUint::one()
            && g.modpow(&(&euler / 2u8), safe_prime) != BigUint::one()
            && miller_rabin(&g, bit_len)
        {
            return g;
        }
        g = rng.gen_biguint(bit_len);
    }
}
