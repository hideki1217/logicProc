extern crate tokio;

use std::ops::Not;

use logicproc::{CircuitNode, traits::LogicOps};

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

    
    /* 
    let i0 = Arc::new(BinaryCircuit::InputNode(Arc::new(WireIn::new(Box::new(
        Binary::One,
    )))));
    let i1 = Arc::new(BinaryCircuit::InputNode(Arc::new(WireIn::new(Box::new(
        Binary::Zero,
    )))));
    let n0 = Arc::new(BinaryCircuit::AndNode(
        i0.clone(),
        i1.clone(),
        Default::default(),
    )); // Binary::Zero
    let n1 = Arc::new(BinaryCircuit::XorNode(
        i0.clone(),
        i1.clone(),
        Default::default(),
    )); // Binary::One
    let o0 = Arc::new(BinaryCircuit::NorNode(n0, n1, Default::default())); // Binary::Zero

    {
        o0.culc_async().await;
        let res = o0.out_wire();
        let lock = res.read();
        let value = lock.as_ref().unwrap();
        println!("res async = {:?}", *value);
    }

    o0.reset_wire(); // 入力以外を消去

    {
        o0.culc();
        let res = o0.out_wire();
        let lock = res.read();
        let value = lock.as_ref().unwrap();
        println!("res = {:?}", *value);
    }*/
}
