mod tests;

use num_bigint::BigInt;
use num_traits::Zero;

//
/// Modulo operation, including negatives
pub fn modulo(num: &BigInt, modulus: &BigInt) -> BigInt {
    if modulus.is_zero() || modulus < &Zero::zero() {
        panic!("Modulus must be a positive number");
    }

    let result = num % modulus;
    
    if result >= Zero::zero() {
        result
    } else {
        result + modulus
    }
}
