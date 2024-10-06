use crate::modulo::modulo;

use num_bigint::BigInt;

#[test]
fn zero() {
    let modulus = BigInt::from(7);
    assert_eq!(modulo(&BigInt::from(0), &modulus), BigInt::from(0));
}

#[test]
fn value_lower_than_modulus() {
    let modulus = BigInt::from(7);
    assert_eq!(modulo(&BigInt::from(4), &modulus), BigInt::from(4));
}

#[test]
fn value_greater_than_modulus() {
    let modulus = BigInt::from(7);
    assert_eq!(modulo(&BigInt::from(13), &modulus), BigInt::from(6));
}

#[test]
fn negative_value() {
    let modulus = BigInt::from(7);
    assert_eq!(modulo(&BigInt::from(-5), &modulus), BigInt::from(2));
    assert_eq!(modulo(&BigInt::from(-18), &modulus), BigInt::from(3));
}

#[test]
fn large_numbers() {
    let modulus = BigInt::parse_bytes(b"340282366920938463463374607431768211455", 10).unwrap();
    let num = BigInt::parse_bytes(b"680564733841876926926749214863536422910", 10).unwrap();
    let expected = num.clone() % &modulus;
    assert_eq!(modulo(&num, &modulus), expected);
}

#[test]
fn negative_large_numbers() {
    let modulus = BigInt::parse_bytes(b"340282366920938463463374607431768211455", 10).unwrap();
    let num = BigInt::parse_bytes(b"-680564733841876926926749214863536422910", 10).unwrap();
    let mut expected = num.clone() % &modulus;
    if expected < BigInt::from(0) {
        expected += &modulus;
    }
    assert_eq!(modulo(&num, &modulus), expected);
}

#[test]
#[should_panic(expected = "Modulus must be a positive number")]
fn zero_modulus() {
    let _ = modulo(&BigInt::from(5), &BigInt::from(0));
}

#[test]
#[should_panic(expected = "Modulus must be a positive number")]
fn negative_modulus() {
    let _ = modulo(&BigInt::from(5), &BigInt::from(-7));
}
