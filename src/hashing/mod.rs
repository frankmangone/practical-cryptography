use num_bigint::BigUint;
use num_traits::FromPrimitive;
use sha2::{Sha256, Digest};

pub fn hash(data: &str, modulo: i128) -> i128 {
  let mut hasher = Sha256::new();

  hasher.update(data);

  let bytes = hasher.finalize();
  let big_int = BigUint::from_bytes_be(&bytes);
    
  let modulo_big_int = BigUint::from_i128(modulo).unwrap();
  let result = big_int % modulo_big_int;

  result.try_into().unwrap()
}