#[cfg(test)]
mod tests {
    use crate::gcd::gcd;
    use num_bigint::BigInt;
    use num_traits::{Zero, One};

    #[test]
    fn test_gcd_simple_case() {
        let a = BigInt::from(48);
        let b = BigInt::from(18);
        let result = gcd(&a, &b);
        assert_eq!(result, BigInt::from(6), "GCD of 48 and 18 should be 6");
    }

    #[test]
    fn test_gcd_coprime() {
        let a = BigInt::from(17);
        let b = BigInt::from(13);
        let result = gcd(&a, &b);
        assert_eq!(result, One::one(), "GCD of 17 and 13 should be 1 (coprime numbers)");
    }

    // failing
    #[test]
    fn test_gcd_one_is_zero() {
        let a = BigInt::from(0);
        let b = BigInt::from(15);
        let result = gcd(&a, &b);
        assert_eq!(result, BigInt::from(15), "GCD of 0 and 15 should be 15");

        let a = BigInt::from(15);
        let b = BigInt::from(0);
        let result = gcd(&a, &b);
        assert_eq!(result, BigInt::from(15), "GCD of 15 and 0 should be 15");
    }

    // failing
    #[test]
    fn test_gcd_both_are_zero() {
        let a = BigInt::zero();
        let b = BigInt::zero();
        let result = gcd(&a, &b);
        assert_eq!(result, BigInt::zero(), "GCD of 0 and 0 should be 0");
    }

    // failing
    #[test]
    fn test_gcd_large_numbers() {
        let a = BigInt::parse_bytes(b"12345678901234567890", 10).unwrap();
        let b = BigInt::parse_bytes(b"98765432109876543210", 10).unwrap();
        let result = gcd(&a, &b);
        assert_eq!(result, BigInt::parse_bytes(b"900000000090", 10).unwrap(), "GCD of large numbers should be 900000000090");
    }

    #[test]
    fn test_gcd_equal_numbers() {
        let a = BigInt::from(25);
        let b = BigInt::from(25);
        let result = gcd(&a, &b);
        assert_eq!(result, BigInt::from(25), "GCD of 25 and 25 should be 25");
    }
}
