use ark_ff::PrimeField;
use ark_bn254::Fr;


struct Multilinear<T: PrimeField> {
    polynomial: Vec<T>,
}
impl<T: PrimeField > Multilinear <T> {
    fn partial_evaluation(&self, evaluation_points: Vec<T>, evaluation_variable_index: usize) -> Vec<T> {
        let mut result: Vec<T> = Vec::new();
        let mut i = 0;
        let mut j = 0;
        let binary: usize = 2;
        let jump:usize = binary.pow((evaluation_points.len() - evaluation_variable_index - 1).try_into().unwrap());
        let langrange_basis_zero = T::one() - evaluation_points[evaluation_variable_index];
        let langrange_basis_one = evaluation_points[evaluation_variable_index];

        while i < self.polynomial.len() && j < evaluation_points.len() {
            result.push(self.polynomial[i] * (langrange_basis_zero) + self.polynomial[i + jump] * (langrange_basis_one));
            if (i + 1) % jump != 0 {
                i = i + 1
            } else {
                i = i + jump;
            }
            j+=1
        }
        result
    }
}

fn main() {
    let mlp = Multilinear {
        polynomial: vec![Fr::from(0), Fr::from(2), Fr::from(0), Fr::from(5)]
    };

    let res = mlp.partial_evaluation(vec![Fr::from(5), Fr::from(0)], 0);

    println!("{:?}", res);
}

#[cfg(test)]
mod tests{
    use crate::Multilinear;
    use ark_bn254::Fq;
    #[test]
    fn partial_evaluation_test(){
        let p = Multilinear {
            polynomial: vec![Fq::from(0), Fq::from(2), Fq::from(0), Fq::from(5)],
        };
        let partial_evaluated_result = p.partial_evaluation(vec![Fq::from(5), Fq::from(0)], 0);
        assert_eq!(partial_evaluated_result, vec![Fq::from(0), Fq::from(17)]);
    }
}


