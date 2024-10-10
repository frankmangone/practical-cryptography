mod tests;

use num_bigint::BigInt;
use num_traits::Zero;

use crate::modulo::modulo;

pub fn gcd(a: &BigInt, b: &BigInt) -> BigInt {
   if a == &Zero::zero() {
      return b.clone();
   } else if b == &Zero::zero() {
      return a.clone();
   }

  let remainder: BigInt = modulo(a, b);

  if remainder == Zero::zero() {
     b.clone()
  } else {
     gcd(b, &remainder)
  }
}
