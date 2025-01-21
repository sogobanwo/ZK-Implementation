use ark_ff::PrimeField;
use ark_std::test_rng;

struct UnivariatePoly <T: PrimeField>{
    coeffient: Vec<T>,
}

fn main() {}

fn multiply_polynomials<T: PrimeField>(x: Vec<Vec<T>>) -> Vec<T> {
    let mut multiplication_result: Vec<T> = x[0].clone();
    for (_i, y) in x.iter().skip(1).enumerate() {
        let mut temp_result: Vec<T> = vec![T::zero(); multiplication_result.len() + y.len() - 1];
        for (j, z) in multiplication_result.iter().enumerate() {
            for (k, w) in y.iter().enumerate() {
                temp_result[j + k] += *z * w;
            }
        }
        multiplication_result = temp_result;
    }
    multiplication_result
}

fn add_polynomials<T: PrimeField>(x: Vec<T>, y: Vec<T>) -> Vec<T> {
    let added_poly = x.iter().zip(y).map(|(x, y)| *x + y).collect::<Vec<T>>();
    added_poly
}

impl <T: PrimeField> UnivariatePoly <T> {
    fn evaluate(&self, x: T) -> T {
        let mut sum: T = T::zero();
        for (i, y) in self.coeffient.iter().enumerate() {
            sum = sum + *y * (x.pow([i as u64]));
        }
        sum
    }

    fn degree(&self) -> usize {
        self.coeffient.len() - 1
    }

    fn interpolate(&self, all_secret_keys:Vec<(T, T)>) -> Vec<T> {
        let mut result: Vec<T> = vec![T::zero(), T::zero(), T::zero()];
        for (i, y) in all_secret_keys.iter().enumerate() {
            let mut denominator:T = T::one();
            let mut numerator: T = T::zero();
            let mut multiplication_vector: Vec<Vec<T>> = Vec::new();
            for (j, z) in all_secret_keys.iter().enumerate() {
                if i != j {
                    denominator *= y.0 as T - z.0 as T;
                    multiplication_vector.push([-(z.0 as T), T::one()].to_vec());
                } else {
                    numerator += z.1 as T;
                }
            }
            let result_to_be_added = multiply_polynomials(multiplication_vector)
                .iter()
                .map(|all_secret_keys| *all_secret_keys * numerator / denominator)
                .collect::<Vec<T>>();
            result = add_polynomials(result, result_to_be_added);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::UnivariatePoly;
    use ark_bn254::Fq;
    use ark_ff::AdditiveGroup;


    #[test]
    fn test_degree() {
        let p = UnivariatePoly {
            coeffient: vec![Fq::from(1), Fq::from(2), Fq::from(3)],
        };
        p.degree();

        assert_eq!(p.degree(), 2);
    }

    #[test]
    fn test_eval() {
        let p = UnivariatePoly {
            coeffient: vec![Fq::from(1), Fq::from(2), Fq::from(3)],
        };
        p.evaluate(Fq::from(1));
        assert_eq!(p.evaluate(Fq::from(1)), Fq::from(6));
    }

    #[test]
    fn added_poly() {
        let x = vec![Fq::from(1), Fq::from(2), Fq::from(3)];
        let y = vec![Fq::from(1), Fq::from(2), Fq::from(3)];
        assert_eq!(add_polynomials(x, y), vec![Fq::from(2), Fq::from(4), Fq::from(6)]);
    }

    #[test]
    fn test_multiply_polynomials() {
        let x = vec![vec![Fq::from(-2), Fq::from(1)], vec![Fq::from(-3), Fq::from(1)]];
        let expected_result = vec![Fq::from(6), Fq::from(-5), Fq::from(1)];
        assert_eq!(multiply_polynomials(x), expected_result);
    }

    #[test]
    fn test_multiply_polynomials_with_negative_coefficients() {
        let x = vec![vec![Fq::from(-2), Fq::from(1)], vec![Fq::from(-3), Fq::from(1)], vec![Fq::from(-4), Fq::from(1)]];
        let expected_result = vec![Fq::from(-24), Fq::from(26), Fq::from(-9), Fq::from(1)];
        assert_eq!(multiply_polynomials(x), expected_result);
    }

    #[test]
    fn test_interpolate() {
        let p = UnivariatePoly {
            coeffient: vec![Fq::from(1), Fq::from(2), Fq::from(3)],
        };
        let x = vec![(Fq::ZERO, Fq::from(2)), (Fq::from(1), Fq::from(0)), (Fq::from(2), Fq::from(0))];
        let expected_result = vec![Fq::from(2), Fq::from(-3), Fq::from(1)];
        assert_eq!(p.interpolate(x), expected_result);
    }

}
