extern crate tokio;

use std::ops::Not;

use logicproc::{Circuit, CircuitNode, traits::LogicOps};

#[derive(Clone, Copy, Debug)]
enum Binary {
    Zero = 0,
    One = 1,
}
impl Binary {
    fn as_u32(&self) -> u32 {
        *self as u32
    }
    fn from_u32(u: u32) -> Binary {
        if u == 0 {
            Binary::Zero
        } else {
            Binary::One
        }
    }
}
impl Not for Binary {
    type Output = Self;
    fn not(self) -> Self::Output {
        Binary::from_u32(1 - self.as_u32())
    }
}
impl LogicOps for Binary {
    fn lgc_nand(&self, rhs: &Self) -> Self {
        Self::lgc_not(&Self::lgc_and(self, rhs))
    }

    fn lgc_and(&self, rhs: &Self) -> Self {
        Binary::from_u32(self.as_u32() & rhs.as_u32())
    }

    fn lgc_nor(&self, rhs: &Self) -> Self {
        Self::lgc_not(&Self::lgc_or(self, rhs))
    }

    fn lgc_or(&self, rhs: &Self) -> Self {
        Binary::from_u32(self.as_u32() | rhs.as_u32())
    }

    fn lgc_nxor(&self, rhs: &Self) -> Self {
        Self::lgc_not(&Self::lgc_xor(self, rhs))
    }

    fn lgc_xor(&self, rhs: &Self) -> Self {
        Binary::from_u32(self.as_u32() ^ rhs.as_u32())
    }

    fn lgc_not(&self) -> Self {
        (*self).not()
    }
}
type BinaryCircuit = CircuitNode<Binary>;




#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let json = include_str!("yosys_sample.v");
    let circuit = Circuit::<Binary>::from_yosys(json).unwrap();

    let res = tokio::spawn(circuit.culc_async()).await;
}
