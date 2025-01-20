use ark_ff::PrimeField;
use ark_std::{test_rng};

fn main(){
//     let p = ShamirSecretSharing {
//         share_params: (5, 3, 19, 257),
//     };

//     let generated_number = p.generate_random_coefficients();
//     println!("Number: {:?}", generated_number);

//     // let evaluation = p.evaluate(10);
//     // println!("Number: {:?}", evaluation);

//     let secrets = p.generate_secrets();
//     println!("Secrets: {:?}", secrets);
}

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

fn evaluate<T: PrimeField>(x: T, coefficients: Vec<T>) -> T {
    let mut sum:T = T::zero();
    for (i, y) in coefficients.iter().enumerate() {
        sum = sum + *y * (x.pow([i as u64]));
    }
    sum
}


struct ShamirSecretSharing<T: PrimeField> {
    share_params: (T, usize, T),
}

impl<T: PrimeField> ShamirSecretSharing <T>{
    fn generate_random_coefficients(&self) -> Vec<T>{
        let mut random_coefficients:Vec<T>  = vec![self.share_params.0];
        for _i in 1..self.share_params.1 {
            let mut rng = test_rng();
            let random_number:T = T::rand(&mut rng);
            random_coefficients.push(random_number);
        }
        random_coefficients
    }

    fn generate_secrets(&self)-> Vec<(T, T)>{
        let mut secrets:Vec<(T, T)> = Vec::new();
        let poly: Vec<T> = self.generate_random_coefficients();
        println!("Poly: {:?}", poly);
        let mut i :T = T::one();
        while i <= self.share_params.2 {
            let part_secret =  evaluate(i, poly.clone());
            secrets.push((i, part_secret));
            i += T::one();
        }
        secrets
    }


    // fn interpolate(&self) -> Vec<T> {
    //     let all_secret_keys:Vec<(T, T)>= self.generate_secrets();
    //     let x:Vec<(T, T)> = all_secret_keys[..self.share_params.2].to_vec();
    //     let mut result: Vec<T> = vec![T::zero(), T::zero(), T::zero()];
    //     for (i, y) in x.iter().enumerate() {
    //         let mut denominator:T = T::one();
    //         let mut numerator: T = T::zero();
    //         let mut multiplication_vector: Vec<Vec<T>> = Vec::new();
    //         for (j, z) in x.iter().enumerate() {
    //             if i != j {
    //                 denominator *= y.0 as T - z.0 as T;
    //                 multiplication_vector.push([-(z.0 as T), 1.0].to_vec());
    //             } else {
    //                 numerator += z.1 as T;
    //             }
    //         }
    //         let result_to_be_added = multiply_polynomials(multiplication_vector)
    //             .iter()
    //             .map(|x| *x * numerator / denominator)
    //             .collect::<Vec<T>>();
    //         result = add_polynomials(result, result_to_be_added);
    //     }
    //     result
    // }
}

// #[cfg(test)]
// mod tests{
//     use crate::ShamirSecretSharing;
//     use ark_bn254::Fq;
//     #[test]
//     fn shamir_secret_sharing_test() {
//         let p = ShamirSecretSharing {
//             share_params: (Fq::from(5), Fq::from(3), Fq::from(10),Fq::from(257)),
//         };
//         p.interpolate();
//         assert_eq!(p.interpolate()[0], Fq::from(5));
//     }
// }

