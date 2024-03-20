use crate::modulo::{modulo, inverse};

#[test]
fn modulo_inside_boundary() {
  assert_eq!(modulo(0, 2), 0i128);
  assert_eq!(modulo(10, 20), 10i128);
  assert_eq!(modulo(9, 1200), 9i128);
}

#[test]
fn modulo_higher_than_boundary() {
  assert_eq!(modulo(9, 2), 1i128);
  assert_eq!(modulo(13, 10), 3i128);
  assert_eq!(modulo(126, 11), 5i128);
}

#[test]
fn modulo_negatives() {
  assert_eq!(modulo(-3, 2), 1i128);
  assert_eq!(modulo(-9, 5), 1i128);
  assert_eq!(modulo(-19, 7), 2i128);
}

//

#[test]
fn inverse_works_for_prime_modulo() {
  assert_eq!(inverse(9, 7), 4i128);
  assert_eq!(inverse(2, 5), 3i128);
  assert_eq!(inverse(128, 11), 8i128);
  assert_eq!(inverse(128378, 8713), 1617i128);
  assert_eq!(inverse(129783182, 98764321), 16580311i128);
}

#[test]
fn inverse_for_negatives_works() {
  assert_eq!(inverse(-3, 2), inverse(1, 2));
  assert_eq!(inverse(-9, 5), inverse(1, 5));
  assert_eq!(inverse(-19, 7), inverse(2, 7));

}

// TODO: Check for non-prime modulo.