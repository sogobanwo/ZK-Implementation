use ark_ff::PrimeField;
use ark_bn254::Fr;

fn partial_evaluation<T: PrimeField>(polynomial: Vec<T>, evaluation_points: Vec<T>, evaluation_variable_index: usize) -> Vec<T> {
    let mut result: Vec<T> = Vec::new();
    let mut i = 0;
    let mut j = 0;
    let binary: usize = 2;
    let expected_poly_size = polynomial.len()/2;

    let jump:usize = binary.pow((evaluation_points.len() - evaluation_variable_index - 1).try_into().unwrap());

    let langrange_basis_zero = T::one() - evaluation_points[evaluation_variable_index];
    let langrange_basis_one = evaluation_points[evaluation_variable_index];

    while i < expected_poly_size {
        result.push(polynomial[j] * (langrange_basis_zero) + polynomial[j + jump] * (langrange_basis_one));

        if (j + 1) % jump != 0 {
            j = j + 1
        } else {
            j = j + jump + 1;
        }

        i += 1;
    }
    result
}


fn evaluate<T: PrimeField>(mut polynomial: Vec<T>, mut evaluation_points: Vec<T>) -> T {
  let times_to_evaluate = evaluation_points.len();
  polynomial = partial_evaluation(polynomial, evaluation_points.clone(), 0);

  let mut i = 1;

  while i < times_to_evaluate {
    evaluation_points.remove(0);
    polynomial = partial_evaluation(polynomial.clone(), evaluation_points.clone(), 0);

    println!("Got Here");
    println!("{:?}", polynomial);
    i +=1 ;
  }

  polynomial[0]
}


fn main() {
    let res = partial_evaluation(vec![Fr::from(0), Fr::from(2), Fr::from(0), Fr::from(5)], vec![Fr::from(5), Fr::from(0)], 0);
    println!("{:?}", res);
}

#[cfg(test)]
mod tests{
    use super::*;
    use ark_bn254::Fq;
    #[test]
    fn partial_evaluation_test(){
        let partial_evaluated_result = partial_evaluation(vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(3),Fq::from(0),Fq::from(0), Fq::from(2),Fq::from(5)], vec![Fq::from(5), Fq::from(0), Fq::from(3)], 2);
        assert_eq!(partial_evaluated_result, vec![Fq::from(0), Fq::from(9), Fq::from(0), Fq::from(11)]);
    }

    #[test]
    fn evaluate_test(){
        let partial_evaluated_result = evaluate(vec![Fq::from(0), Fq::from(0), Fq::from(3), Fq::from(8)], vec![Fq::from(6), Fq::from(2)]);
        assert_eq!(partial_evaluated_result, Fq::from(78));
    }
}


