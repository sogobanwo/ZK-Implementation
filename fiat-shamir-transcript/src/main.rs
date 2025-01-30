use sha3::{Digest, Keccak256};
use ark_ff::PrimeField;
use std::marker::PhantomData;

fn main() {
    println!("Hello, world!");
}


struct FiatShamirTranscript<T: PrimeField> {
    hash_function: Keccak256,
    _marker: PhantomData<T>,
}

impl<T: PrimeField> FiatShamirTranscript<T> {
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
    fn generate_and_return_hash_as_field_element(&mut self) -> T {
        let hash_bytes = self.generate_and_return_hash_as_bytes();
        T::from_random_bytes(hash_bytes.as_slice()).unwrap()
    }
}

//  cargo test -- --nocapture

#[cfg(test)]
mod test {
    use super::*;
    use ark_bn254::Fq;

    #[test]
    fn test_hash() {
        let mut transcript = FiatShamirTranscript::init_hash_function();
        transcript.update_hash_function("sogo".as_bytes());
        let challange_array = transcript.generate_and_return_hash_as_bytes();
        let challenge_field_element: Fq = transcript.generate_and_return_hash_as_field_element();
        dbg!(challange_array);
        dbg!(challenge_field_element);
    }
}