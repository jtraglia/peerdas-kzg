use bls12_381::ff::Field;
use bls12_381::Scalar;

/// This file will hold the implementation of a polynomial in monomial form

// TODO: These methods are taking ownership which should Ideally be changed.
// TODO: We can also possibly remove the type alias and use a new type

/// A polynomial in monomial form where the lowest degree term is first
/// Layout: x^0 * a_0 + x^1 * a_1 + ... + x^(n-1) * a_(n-1)
pub type PolyCoeff = Vec<Scalar>;

/// For two polynomials, `f(x)` and `g(x)`, this method computes
/// the result of `f(x) + g(x)` and returns the result.
///
/// Note: Polynomials can be of different lengths.
pub fn poly_add(a: PolyCoeff, b: PolyCoeff) -> PolyCoeff {
    let (smaller_poly, mut larger_poly) = if a.len() < b.len() { (a, b) } else { (b, a) };

    for i in 0..smaller_poly.len() {
        larger_poly[i] += smaller_poly[i];
    }

    larger_poly
}

/// For a polynomial, `f(x)`, this method computes the result of `-f(x)`
/// and returns the result.
pub fn poly_neg(mut a: PolyCoeff) -> PolyCoeff {
    for i in 0..a.len() {
        a[i] = -a[i];
    }
    a
}

/// For two polynomials, `f(x)` and `g(x)`, this method computes
/// the result of `f(x) - g(x)` and returns the result.
///
/// Note: Polynomials can be of different lengths
pub fn poly_sub(a: PolyCoeff, b: PolyCoeff) -> PolyCoeff {
    let neg_b = poly_neg(b);
    poly_add(a, neg_b)
}

/// Given a polynomial `f(x)` and a scalar `z`. This method will compute
/// the result of `f(z)` and return the result.
pub fn poly_eval(poly: &PolyCoeff, value: &Scalar) -> Scalar {
    let mut result = Scalar::from(0u64);
    for coeff in poly.iter().rev() {
        result = result * value + coeff;
    }
    result
}

/// For two polynomials, `f(x)` and `g(x)`, this method computes
/// the result of `f(x) * g(x)` and returns the result.
pub fn poly_mul(a: &PolyCoeff, b: &PolyCoeff) -> PolyCoeff {
    let mut result = vec![Scalar::ZERO; a.len() + b.len() - 1];

    for (i, a_coeff) in a.iter().enumerate() {
        for (j, b_coeff) in b.iter().enumerate() {
            result[i + j] += a_coeff * b_coeff;
        }
    }

    result
}

/// Given a list of points, this method will compute the polynomial
/// Z(x) which is equal to zero when evaluated at each point.
///
/// Example: vanishing_poly([1, 2, 3]) = (x - 1)(x - 2)(x - 3)
pub fn vanishing_poly(roots: &[Scalar]) -> Vec<Scalar> {
    let mut poly = vec![Scalar::from(1u64)];
    for root in roots {
        poly = poly_mul(&poly, &vec![-root, Scalar::from(1u64)]);
    }
    poly
}

#[cfg(test)]
mod tests {
    use super::*;
    use bls12_381::ff::Field;

    fn naive_poly_eval(poly: &PolyCoeff, value: &Scalar) -> Scalar {
        let mut result = Scalar::from(0u64);
        for (i, coeff) in poly.iter().enumerate() {
            result += coeff * value.pow_vartime(&[i as u64]);
        }
        result
    }

    #[test]
    fn basic_polynomial_add() {
        let a = vec![Scalar::from(1), Scalar::from(2), Scalar::from(3)];
        let b = vec![Scalar::from(4), Scalar::from(5), Scalar::from(6)];
        let c = vec![Scalar::from(5), Scalar::from(7), Scalar::from(9)];
        assert_eq!(poly_add(a, b), c);

        let a = vec![Scalar::from(2), Scalar::from(3)];
        let b = vec![Scalar::from(4), Scalar::from(5), Scalar::from(6)];
        let c = vec![Scalar::from(6), Scalar::from(8), Scalar::from(6)];
        assert_eq!(poly_add(a, b), c);
    }

    #[test]
    fn polynomial_neg() {
        let a = vec![Scalar::from(1), Scalar::from(2), Scalar::from(3)];
        let b = vec![-Scalar::from(1), -Scalar::from(2), -Scalar::from(3)];
        assert_eq!(poly_neg(a), b);
    }

    #[test]
    fn basic_polynomial_subtraction() {
        let a = vec![Scalar::from(1), Scalar::from(2), Scalar::from(3)];
        let b = vec![Scalar::from(4), Scalar::from(5), Scalar::from(6)];
        let c = vec![-Scalar::from(3), -Scalar::from(3), -Scalar::from(3)];
        assert_eq!(poly_sub(a, b), c);

        let a = vec![Scalar::from(4), Scalar::from(5)];
        let b = vec![Scalar::from(6), Scalar::from(7), Scalar::from(8)];
        let c = vec![-Scalar::from(2), -Scalar::from(2), -Scalar::from(8)];
        assert_eq!(poly_sub(a, b), c);
    }

    #[test]
    fn polynomial_evaluation() {
        // f(x) = 1 + 2x + 3x^2
        // f(2) = 1 + 2*2 + 3*2^2 = 1 + 4 + 12 = 17
        let poly = vec![Scalar::from(1), Scalar::from(2), Scalar::from(3)];
        let value = Scalar::from(2u64);
        assert!(poly_eval(&poly, &value) == naive_poly_eval(&poly, &value));
    }

    #[test]
    fn polynomial_multiplication() {
        // f(x) = 1 + 2x + 3x^2
        // g(x) = 4 + 5x
        // f(x) * g(x) = 4 + 8x + 12x^2 + 5x + 10x^2 + 15x^3 = 4 + 13x + 22x^2 + 15x^3
        let a = vec![Scalar::from(1), Scalar::from(2), Scalar::from(3)];
        let b = vec![Scalar::from(4), Scalar::from(5)];
        let expected = vec![
            Scalar::from(4),
            Scalar::from(13),
            Scalar::from(22),
            Scalar::from(15),
        ];
        assert_eq!(poly_mul(&a, &b), expected);
    }

    #[test]
    fn vanishing_polynomial_smoke_test() {
        // f(x) = (x - 1)(x - 2)(x - 3) = x^3 - 6x^2 + 11x - 6
        let roots = vec![Scalar::from(1u64), Scalar::from(2u64), Scalar::from(3u64)];
        let expected = vec![
            -Scalar::from(6u64),
            Scalar::from(11u64),
            -Scalar::from(6u64),
            Scalar::from(1u64),
        ];
        let poly = vanishing_poly(&roots);
        assert_eq!(&poly, &expected);

        // Check that this polynomial evaluates to zero on the roots
        for root in roots.iter() {
            assert_eq!(poly_eval(&poly, &root), Scalar::from(0u64));
        }
    }
}
