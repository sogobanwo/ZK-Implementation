use ark_ff::PrimeField;
use ark_bn254::Fr;


fn main(){
    print!("Hello World")
}

enum operations {
   add,
   mul
}
struct gate{
    lhs: usize,
    rhs: usize,
    ops: operations,
    output: usize,
}

struct layer {
    gates: Vec<gate>
}

struct circuit {
    layers: Vec<layer>
}


impl gate {
    fn new(lhs:usize, rhs:usize, ops: operations, output: usize) -> Self{
        Self{
            lhs,
            rhs,
            ops,
            output
        }
    }
}

impl layer {
    fn new (x: Vec<gate>) -> Self {
        Self{
            gates: x 
        }
    }
}

impl circuit {
    fn new (x: Vec<layer>) -> Self {
        Self{
            layers: x
        }
    }
}

