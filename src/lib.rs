use std::{
    future::Future,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    pin::Pin,
    sync::{Arc, Mutex, RwLock, RwLockReadGuard},
};

/// # Sammary
/// Binaryを表現するT:LogicBinaryOpsが与えられると、
/// それを回路に則って計算する非同期プロセッサ
/// # contents
/// - LogicCircuit:
/// - Wire: マルチスレッドでもone:multiでのイミュータブルデータ共有をする
/// - Node: culc()でoutに計算結果を流す

pub trait LogicOps {
    fn nand(&self, rhs: &Self) -> Self;
    fn and(&self, rhs: &Self) -> Self;
    fn nor(&self, rhs: &Self) -> Self;
    fn or(&self, rhs: &Self) -> Self;
    fn nxor(&self, rhs: &Self) -> Self;
    fn xor(&self, rhs: &Self) -> Self;
    fn not(&self) -> Self;
}

pub struct LogicCircuit<T: LogicOps> {
    wires: Vec<Wire<T>>,
}
impl<T: LogicOps> LogicCircuit<T> {
    pub fn read_wire(&self, index: usize) -> RwLockReadGuard<Option<Box<T>>> {
        self.wires.get(index).unwrap().read()
    }
    pub fn write_wire(&mut self, index: usize, item: Box<T>) {
        self.wires.get_mut(index).unwrap().write(item);
    }
}

struct Wire<T> {
    item: RwLock<Option<Box<T>>>,
    // item_in:RwLock<*mut T>
}
impl<T> Wire<T> {
    pub fn read(&self) -> RwLockReadGuard<Option<Box<T>>> {
        self.item.read().unwrap()
    }
    pub fn write(&mut self, item: Box<T>) -> Option<Box<T>> {
        self.item.write().unwrap().replace(item)
    }
}

enum CircuitNode {
    NandNode(Arc<CircuitNode>, Arc<CircuitNode>, usize),
    AndNode(Arc<CircuitNode>, Arc<CircuitNode>, usize),
    NorNode(Arc<CircuitNode>, Arc<CircuitNode>, usize),
    OrNode(Arc<CircuitNode>, Arc<CircuitNode>, usize),
    NxorNode(Arc<CircuitNode>, Arc<CircuitNode>, usize),
    XorNode(Arc<CircuitNode>, Arc<CircuitNode>, usize),
    NotNode(Arc<CircuitNode>, usize),
}
impl CircuitNode {
    fn out_index(&self) -> usize {
        match self {
            CircuitNode::NandNode(_, _, out) => *out,
            CircuitNode::AndNode(_, _, out) => *out,
            CircuitNode::NorNode(_, _, out) => *out,
            CircuitNode::OrNode(_, _, out) => *out,
            CircuitNode::NxorNode(_, _, out) => *out,
            CircuitNode::XorNode(_, _, out) => *out,
            CircuitNode::NotNode(_, out) => *out,
        }
    }
    #[inline]
    fn culc_binary_ops<T: LogicOps>(
        circuit: &mut LogicCircuit<T>,
        rhs: &Arc<CircuitNode>,
        lhs: &Arc<CircuitNode>,
        out: &usize,
        f: fn(&T, &T) -> T,
    ) {
        let res = {
            let rhs_lock = circuit.read_wire(rhs.out_index());
            let lhs_lock = circuit.read_wire(lhs.out_index());
            let rhs = rhs_lock.as_ref().unwrap();
            let lhs = lhs_lock.as_ref().unwrap();
            f(rhs, lhs)
        };
        circuit.write_wire(*out, Box::new(res));
    }
    #[inline]
    fn culc_mono_ops<T: LogicOps>(
        circuit: &mut LogicCircuit<T>,
        input: &Arc<CircuitNode>,
        out: &usize,
        f: fn(&T) -> T,
    ) {
        let res = {
            let input_lock = circuit.read_wire(input.out_index());
            let input = input_lock.as_ref().unwrap();
            f(input)
        };
        circuit.write_wire(*out, Box::new(res));
    }
    fn culc<T: LogicOps>(&self, circuit: &mut LogicCircuit<T>) {
        match self {
            CircuitNode::NandNode(rhs, lhs, out) => {
                Self::culc_binary_ops(circuit, rhs, lhs, out, T::nand);
            }
            CircuitNode::AndNode(rhs, lhs, out) => {
                Self::culc_binary_ops(circuit, rhs, lhs, out, T::and);
            }
            CircuitNode::NorNode(rhs, lhs, out) => {
                Self::culc_binary_ops(circuit, rhs, lhs, out, T::nor);
            }
            CircuitNode::OrNode(rhs, lhs, out) => {
                Self::culc_binary_ops(circuit, rhs, lhs, out, T::or);
            }
            CircuitNode::NxorNode(rhs, lhs, out) => {
                Self::culc_binary_ops(circuit, rhs, lhs, out, T::nxor);
            }
            CircuitNode::XorNode(rhs, lhs, out) => {
                Self::culc_binary_ops(circuit, rhs, lhs, out, T::xor);
            }
            CircuitNode::NotNode(input, out) => Self::culc_mono_ops(circuit, input, out, T::not),
        }
    }
    fn culc_async_binary_ops<T: LogicOps + 'static>(
        circuit: &'static mut LogicCircuit<T>,
        rhs: &Arc<CircuitNode>,
        lhs: &Arc<CircuitNode>,
        out: &usize,
        f: fn(&T, &T) -> T,
    ) -> impl Future {
        let rhs = rhs.clone();
        let lhs = lhs.clone();
        let out = *out;
        async move {
            rhs.culc_async(circuit).await;
            lhs.culc_async(circuit).await;
            Self::culc_binary_ops(circuit, &rhs, &lhs, &out, f);
        }
    }
    fn culc_async_mono_ops<T: LogicOps + 'static>(
        circuit: &'static mut LogicCircuit<T>,
        input: &Arc<CircuitNode>,
        out: &usize,
        f: fn(&T) -> T,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
        let input = input.clone();
        let out = *out;
        Box::pin(async move {
            input.culc_async(circuit).await;
            Self::culc_mono_ops(circuit, &input, &out, f);
        })
    }
    fn culc_async<'a, 'b: 'a, T: LogicOps + 'static>(
        &'b self,
        circuit: &'a mut LogicCircuit<T>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
        match self {
            CircuitNode::NandNode(rhs, lhs, out) => {
                Box::pin(Self::culc_async_binary_ops(circuit, &rhs, &lhs, &out, T::nand))
            }
            CircuitNode::AndNode(rhs, lhs, out) => {
                Self::culc_async_binary_ops(circuit, &rhs, &lhs, &out, T::and)
            }
            CircuitNode::NorNode(rhs, lhs, out) => {
                Self::culc_async_binary_ops(circuit, rhs, lhs, out, T::nor)
            }
            CircuitNode::OrNode(rhs, lhs, out) => {
                Self::culc_async_binary_ops(circuit, rhs, lhs, out, T::or)
            }
            CircuitNode::NxorNode(rhs, lhs, out) => {
                Self::culc_async_binary_ops(circuit, rhs, lhs, out, T::nxor)
            }
            CircuitNode::XorNode(rhs, lhs, out) => {
                Self::culc_async_binary_ops(circuit, rhs, lhs, out, T::xor)
            }
            CircuitNode::NotNode(input, out) => {
                Self::culc_async_mono_ops(circuit, input, out, T::not)
            }
        }
    }
}
