use ark_ff::BigInteger;
use ark_ff::PrimeField;
use sha3::{Digest, Keccak256};
use std::marker::PhantomData;

fn main() {
    println!("Hello, world!");
}

//Helper Functions
fn partial_evaluation<F: PrimeField>(
    polynomial: Vec<F>,
    evaluation_points: usize,
    evaluation_value: F,
) -> Vec<F> {
    let mut result: Vec<F> = Vec::new();
    let mut i = 0;
    let mut j = 0;
    let binary: usize = 2;
    let expected_poly_size = polynomial.len() / 2;

    let jump: usize = binary.pow((evaluation_points - 1).try_into().unwrap());

    let langrange_basis_zero = F::one() - evaluation_value;
    let langrange_basis_one = evaluation_value;

    while i < expected_poly_size {

        if j + jump < polynomial.len() {
            result.push(
                polynomial[j] * langrange_basis_zero + polynomial[j + jump] * langrange_basis_one,
            );
        } else{

            result.push(polynomial[j] * langrange_basis_zero + polynomial[j + 1] * langrange_basis_one);

        }

        if (j + 1) % jump != 0 {
            j += 1;
        } else {
            j += jump + 1;
        }

        i += 1;
    }
    result
}


fn evaluate<F: PrimeField>(mut polynomial: Vec<F>, mut evaluation_points: Vec<F>) -> F {
    let times_to_evaluate = evaluation_points.len();
    polynomial = partial_evaluation(polynomial, evaluation_points.clone().len(), evaluation_points[0]);
  
    let mut i = 1;
  
  println!("line 58: {:?}", polynomial);
    while i < times_to_evaluate {
      evaluation_points.remove(0);
      polynomial = partial_evaluation(polynomial.clone(), evaluation_points.clone().len(), evaluation_points[0]);
      i +=1 ;
    }
  
  println!("line 13: {:?}", polynomial);
    polynomial[0]
  }

// Proover
struct ProoverResponse<F: PrimeField> {
    univariate_polynomials: Vec<Vec<F>>,
    claim_sum: F,
    main_polynomial: Vec<F>,
}

struct FiatShamirTranscript<F: PrimeField> {
    hash_function: Keccak256,
    _marker: PhantomData<F>,
}

impl<F: PrimeField> FiatShamirTranscript<F> {
    fn init_hash_function() -> Self {
        Self {
            hash_function: Keccak256::new(),
            _marker: PhantomData,
        }
    }

    // Message here is what you want to use to generate the hash which is going to be used to generate the challenge
    fn update_hash_function(&mut self, message: &[u8]) {
        self.hash_function.update(message);
    }

    // Returns the hash as bytes
    fn generate_and_return_hash_as_bytes(&mut self) -> Vec<u8> {
        let function_to_hash = self.hash_function.clone();
        let generated_hash = function_to_hash.finalize();
        generated_hash.to_vec()
    }

    // Returns the hash as a field element
    fn generate_and_return_hash_as_field_element(&mut self) -> F {
        let hash_bytes = self.generate_and_return_hash_as_bytes();
        F::from_random_bytes(hash_bytes.as_slice()).unwrap()
    }
}

fn generate_proof<F: PrimeField>(polynomial: Vec<F>) -> ProoverResponse<F> {

    let claimed_sum = polynomial.iter().sum();

    let mut current_polynomial = polynomial.clone();

    let poly_length = polynomial.len() as f64;

    let number_of_challenges = poly_length.log2().ceil() as usize;

    let mut all_univariate_polynomials: Vec<Vec<F>> = Vec::new();

    let mut hash_transcript = FiatShamirTranscript::init_hash_function();

    for _ in 0..number_of_challenges {
        let middle_index = current_polynomial.len() / 2 + current_polynomial.len() % 2;

        let first_univariate_polynomial_element = current_polynomial[..middle_index].iter().sum();

        let second_univariate_polynomial_element = current_polynomial[middle_index..].iter().sum();

        let univariate_polynomial = vec![
            first_univariate_polynomial_element,
            second_univariate_polynomial_element,
        ];

        all_univariate_polynomials.push(univariate_polynomial.clone());

        let vec_of_univarate = univariate_polynomial
            .iter()
            .flat_map(|x| x.into_bigint().to_bytes_le())
            .collect::<Vec<u8>>();

        let bytes_of_univariate: &[u8] = vec_of_univarate.as_slice();

        hash_transcript.update_hash_function(bytes_of_univariate);

        let challenge = hash_transcript.generate_and_return_hash_as_field_element();

        let current_poly_length = current_polynomial.len() as f64;

        let current_number_of_challenges = current_poly_length.log2().ceil() as usize;

        current_polynomial =
            partial_evaluation(current_polynomial, current_number_of_challenges, challenge);
    }

    ProoverResponse {
        univariate_polynomials: all_univariate_polynomials,
        claim_sum: claimed_sum,
        main_polynomial: polynomial,
    }

}

// Verifier
fn verify_proof<F: PrimeField>(proof: ProoverResponse<F>) -> bool {
    let mut hash_transcript = FiatShamirTranscript::init_hash_function();

    let univariate_polynomials = proof.univariate_polynomials.clone();

    let mut claimed_sum = proof.claim_sum;

    let initial_polynomial = proof.main_polynomial.clone();

    let mut challenges: Vec<F> = Vec::new();

    for i in 0..proof.univariate_polynomials.len() {

        let zero = F::from(0);
        let one = F::from(1);

        let claimed_sum_at_i = evaluate(univariate_polynomials[i].clone(), vec![zero]) + evaluate(univariate_polynomials[i].clone(), vec![one]);

        if claimed_sum_at_i != claimed_sum {

            return false

        }

        let vec_of_univarate_poly = univariate_polynomials[i].clone().iter().flat_map(|x|  x.into_bigint().to_bytes_le()).collect::<Vec<u8>>();

        let bytes_of_univariate_poly: &[u8] = vec_of_univarate_poly.as_slice();

        hash_transcript.update_hash_function(bytes_of_univariate_poly);

        let challenge: F = hash_transcript.generate_and_return_hash_as_field_element();

        challenges.push(challenge);

        claimed_sum = evaluate(univariate_polynomials[i].clone(), challenges.clone());
    }

    println!("{:?}", challenges);

    // check if its prover is correct
    if claimed_sum == evaluate(initial_polynomial, challenges) {
        true
    }else {
        false
    }
   
}







#[cfg(test)]
mod test {
    use super::*;
    use ark_bn254::Fq;

    #[test]
    fn test_prover_response() {
        let evaluated_values = vec![Fq::from(0), Fq::from(0), Fq::from(3), Fq::from(8)];
        let proof = generate_proof(evaluated_values.clone());
        assert_eq!(proof.claim_sum, Fq::from(11));
        assert_eq!(proof.main_polynomial, evaluated_values);
    }

    #[test]
    fn test_verifier() {
        let evaluated_values = vec![Fq::from(0), Fq::from(0), Fq::from(3), Fq::from(8)];
        let proof = generate_proof(evaluated_values.clone());
        assert_eq!(verify_proof(proof), true);
    }
}