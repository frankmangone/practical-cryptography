mod tests;

//
/// Modulo operation, including negatives
pub fn modulo(num: i128, modulo: i128) -> i128 {
  let value: i128;
  
  if num >= 0 {
    value = num % modulo
  } else {
    value = modulo + (num % modulo)
  }

  value
}

// Calculates (n^x) % p
fn modular_exponent(mut n: i128, mut x: i128, p: i128) -> i128 {
  let mut ans = 1;

  if x <= 0 { return 1; }

  loop {
    if x == 1 { return modulo(ans * n, p); }
    if x&1 == 0 {
      n = modulo(n * n, p);
      x >>= 1;
      continue;
    }
    else {
      ans = modulo(ans * n, p);
      x -= 1;
    }
  }
}

/// Find GCD
fn gcd(mut a: i128, mut b: i128) -> i128 {
  if a == b { return a; }
  if b > a {
      let temp = a;
      a = b;
      b = temp;
  }
  while b > 0 {
      let temp = a;
      a = b;
      b = modulo(temp, b);
  }
  return a;
}

pub fn inverse(n: i128, p: i128) -> i128 {
  // Returns 0 if no Modular Multiplicative Inverse exist
  if p <= 1 || gcd(n, p) > 1 {
    return 0;
  }

  // Return Modular Multiplicative Inverse, that is (n^(p-2)) mod p
  // From Fermat's little theorem
  return modular_exponent(n, p - 2, p);
}
