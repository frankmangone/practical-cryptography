#[cfg(test)]
mod tests {
    use crate::modulo::div_rem;
    use num_bigint::BigInt;

    #[test]
    fn zero() {
        let divisor = BigInt::from(7);
        let (quotient, remainder) = div_rem(&BigInt::from(0), &divisor);

        assert_eq!(quotient, BigInt::from(0));
        assert_eq!(remainder, BigInt::from(0));
    }

    #[test]
    fn value_lower_than_divisor() {
        let dividend = BigInt::from(4);
        let divisor = BigInt::from(7);

        let (quotient, remainder) = div_rem(&dividend, &divisor);

        assert_eq!(quotient, BigInt::from(0));
        assert_eq!(remainder, BigInt::from(4));
    }

    #[test]
    fn value_greater_than_divisor() {
        let dividend = BigInt::from(13);
        let divisor = BigInt::from(7);

        let (quotient, remainder) = div_rem(&dividend, &divisor);

        assert_eq!(quotient, BigInt::from(1));
        assert_eq!(remainder, BigInt::from(6));
    }

    #[test]
    fn negative_dividend() {
        let dividend = BigInt::from(-13);
        let divisor = BigInt::from(7);

        let (quotient, remainder) = div_rem(&dividend, &divisor);

        assert_eq!(quotient, BigInt::from(-2));
        assert_eq!(remainder, BigInt::from(1));
    }

    #[test]
    fn negative_divisor() {
        let dividend = BigInt::from(15);
        let divisor = BigInt::from(-7);

        let (quotient, remainder) = div_rem(&dividend, &divisor);

        assert_eq!(quotient, BigInt::from(-2));
        assert_eq!(remainder, BigInt::from(1));
    }
}