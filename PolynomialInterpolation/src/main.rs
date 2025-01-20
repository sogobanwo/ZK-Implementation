struct UnivariatePoly {
    coeffient: Vec<usize>,
}

fn main() {}

fn multiply_polynomials(x: Vec<Vec<f64>>) -> Vec<f64> {
    let mut multiplication_result: Vec<f64> = x[0].clone();
    for (_i, y) in x.iter().skip(1).enumerate() {
        let mut temp_result: Vec<f64> = vec![0.0; multiplication_result.len() + y.len() - 1];
        for (j, z) in multiplication_result.iter().enumerate() {
            for (k, w) in y.iter().enumerate() {
                temp_result[j + k] += z * w;
            }
        }
        multiplication_result = temp_result;
    }
    return multiplication_result;
}

fn add_polynomials(x: Vec<f64>, y: Vec<f64>) -> Vec<f64> {
    let added_poly = x.iter().zip(y).map(|(x, y)| x + y).collect::<Vec<f64>>();
    added_poly
}

impl UnivariatePoly {
    fn evaluate(&self, x: usize) -> usize {
        let mut sum = 0;
        for (i, y) in self.coeffient.iter().enumerate() {
            sum = sum + y * (x.pow(i.try_into().unwrap()))
        }
        return sum;
    }

    fn degree(&self) -> usize {
        self.coeffient.len() - 1
    }

    fn interpolate(&self, x: Vec<(f64, f64)>) -> Vec<f64> {
        let mut result: Vec<f64> = vec![0.0, 0.0, 0.0];
        for (i, y) in x.iter().enumerate() {
            let mut denominator = 1.0;
            let mut numerator: f64 = 0.0;
            let mut multiplication_vector: Vec<Vec<f64>> = Vec::new();
            for (j, z) in x.iter().enumerate() {
                if i != j {
                    denominator *= z.0 as f64 - y.0 as f64;
                    multiplication_vector.push([-(z.0) as f64, 1.0].to_vec());
                } else {
                    numerator += z.1 as f64;
                }
            }
            let result_to_be_added = multiply_polynomials(multiplication_vector)
                .iter()
                .map(|x| x * numerator / denominator as f64)
                .collect::<Vec<f64>>();
            result = add_polynomials(result, result_to_be_added);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::UnivariatePoly;

    #[test]
    fn test_degree() {
        let p = UnivariatePoly {
            coeffient: vec![1, 2, 3],
        };
        p.degree();

        assert_eq!(p.degree(), 2);
    }

    #[test]
    fn test_eval() {
        let p = UnivariatePoly {
            coeffient: vec![1, 2, 3],
        };
        p.evaluate(1);
        assert_eq!(p.evaluate(1), 6);
    }

    #[test]
    fn test_interpolate() {
        let p = UnivariatePoly {
            coeffient: vec![1, 2, 3],
        };
        let x = vec![(0.0, 2.0), (1.0, 0.0), (2.0, 0.0)];
        let expected_result = vec![2.0, -3.0, 1.0];
        assert_eq!(p.interpolate(x), expected_result);
    }

    #[test]
    fn added_poly() {
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![1.0, 2.0, 3.0];
        assert_eq!(add_polynomials(x, y), vec![2.0, 4.0, 6.0]);
    }

    #[test]
    fn test_multiply_polynomials() {
        let x = vec![vec![-2.0, 1.0], vec![-3.0, 1.0]];
        let expected_result = vec![6.0, -5.0, 1.0];
        assert_eq!(multiply_polynomials(x), expected_result);
    }

    #[test]
    fn test_multiply_polynomials_with_negative_coefficients() {
        let x = vec![vec![-2.0, 1.0], vec![-3.0, 1.0], vec![-4.0, 1.0]];
        let expected_result = vec![-24.0, 26.0, -9.0, 1.0];
        assert_eq!(multiply_polynomials(x), expected_result);
    }
}
