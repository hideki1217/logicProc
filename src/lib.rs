#![feature(async_stream)]
extern crate num;
extern crate serde;
extern crate tokio_dagtask;

pub mod yosys_parse;

use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::{Arc, RwLock, RwLockReadGuard},
};
use tokio_dagtask::TaskGraph;
use tokio_stream::{Stream, StreamExt};
use yosys_parse::{WireId, YosysRootElem};

use traits::LogicOps;
pub mod traits {
    pub trait LogicOps {
        fn lgc_nand(&self, rhs: &Self) -> Self;
        fn lgc_and(&self, rhs: &Self) -> Self;
        fn lgc_nor(&self, rhs: &Self) -> Self;
        fn lgc_or(&self, rhs: &Self) -> Self;
        fn lgc_nxor(&self, rhs: &Self) -> Self;
        fn lgc_xor(&self, rhs: &Self) -> Self;
        fn lgc_not(&self) -> Self;
    }
}
type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

/// # Sammary
/// Binaryを表現するT:LogicBinaryOpsが与えられると、
/// それを回路に則って計算する非同期プロセッサ
/// # contents
/// - Wire: マルチスレッドでもone:multiでのイミュータブルデータ共有をする
/// - Node: culc()でoutに計算結果を流す
pub struct Circuit<T> {
    output: Vec<(WireId, WireOut<T>)>,
    input: Vec<(WireId, WireIn<T>)>,
    engine: TaskGraph<BoxFuture<()>>,
}
impl<T> Circuit<T> {
    pub fn set_input(&mut self, id: WireId, val: Box<T>) -> Result<Option<Box<T>>, &'static str> {
        let i = self
            .input
            .binary_search_by_key(&id, |(x, _)| *x)
            .map_err(|_| "selected id is not found")?;
        let (_, wire) = self.input.get_mut(i).unwrap();
        wire.write(val)
    }
    pub fn get_output(&mut self, id: WireId) -> Result<Option<Box<T>>, &'static str> {
        let i = self
            .output
            .binary_search_by_key(&id, |(x, _)| *x)
            .map_err(|_| "selected id is not found")?;
        let (_, wire) = self.output.get_mut(i).unwrap();
        Ok(wire.read_and_clear()?)
    }
    pub fn culc_async(self) -> BoxFuture<()>
    where
        T: LogicOps + Send + Sync + 'static,
    {
        let (_, exec) = self.engine.execute();
        Box::pin(async move {
            exec.collect::<Vec<_>>().await;
        })
    }
    pub fn from_yosys(json: &str) -> Option<Self>
    where
        T: LogicOps + Send + Sync + 'static,
    {
        let yosys = YosysRootElem::from_json(json)?;
        let (_, module) = yosys.modules.iter().last().unwrap();

        let mut input: Vec<(WireId, WireIn<T>)> = Vec::new();
        let mut output: Vec<(WireId, WireOut<T>)> = Vec::new();
        let mut output_id: Vec<u32> = Vec::new();

        let mut nodes: HashMap<u32, CircuitNode<T>> = HashMap::new();

        // nodesにCircuitNodeをセット
        for (_, port) in module.ports.iter() {
            match port.direction {
                yosys_parse::Direction::In => {
                    for &id in port.bits.iter() {
                        input.push((id, Default::default()));
                    }
                }
                yosys_parse::Direction::Out => {
                    for &id in port.bits.iter() {
                        output_id.push(id);
                    }
                }
                yosys_parse::Direction::InOut => {
                    panic!("inout is not supported");
                }
            }
        }
        input.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
        for (_, cell) in module.cells.iter() {
            let out_id = cell.output_wireid();
            nodes.insert(
                out_id,
                match cell.type_name {
                    yosys_parse::CellType::And => {
                        CircuitNode::AndNode(None, None, Default::default())
                    }
                    yosys_parse::CellType::Nand => {
                        CircuitNode::NandNode(None, None, Default::default())
                    }
                    yosys_parse::CellType::Or => {
                        CircuitNode::OrNode(None, None, Default::default())
                    }
                    yosys_parse::CellType::Nor => {
                        CircuitNode::NorNode(None, None, Default::default())
                    }
                    yosys_parse::CellType::Xor => {
                        CircuitNode::XorNode(None, None, Default::default())
                    }
                    yosys_parse::CellType::Nxor => {
                        CircuitNode::NxorNode(None, None, Default::default())
                    }
                    yosys_parse::CellType::Not => CircuitNode::NotNode(None, Default::default()),
                },
            );
        }
        // 各ノードの入力部をセット
        for (_, cell) in module.cells.iter() {
            let out_id = cell.output_wireid();
            let in_ids = cell.input_wireids();
            match cell.type_name {
                yosys_parse::CellType::And => {
                    let in0_out = nodes.get(&in_ids[0]).map_or(
                        input[input.binary_search_by_key(&in_ids[0], |(a, _)| *a).unwrap()]
                            .1
                            .get_out(),
                        |a| a.out_wire(),
                    );
                    let in1_out = nodes.get(&in_ids[1]).map_or(
                        input[input.binary_search_by_key(&in_ids[1], |(a, _)| *a).unwrap()]
                            .1
                            .get_out(),
                        |a| a.out_wire(),
                    );
                    if let CircuitNode::AndNode(in0, in1, _) = nodes.get_mut(&out_id).unwrap() {
                        in0.replace(in0_out);
                        in1.replace(in1_out);
                    } else {
                        unreachable!()
                    }
                }
                yosys_parse::CellType::Nand => {
                    let in0_out = nodes.get(&in_ids[0]).map_or(
                        input[input.binary_search_by_key(&in_ids[0], |(a, _)| *a).unwrap()]
                            .1
                            .get_out(),
                        |a| a.out_wire(),
                    );
                    let in1_out = nodes.get(&in_ids[1]).map_or(
                        input[input.binary_search_by_key(&in_ids[1], |(a, _)| *a).unwrap()]
                            .1
                            .get_out(),
                        |a| a.out_wire(),
                    );
                    if let CircuitNode::NandNode(in0, in1, _) = nodes.get_mut(&out_id).unwrap() {
                        in0.replace(in0_out);
                        in1.replace(in1_out);
                    } else {
                        unreachable!()
                    }
                }
                yosys_parse::CellType::Or => {
                    let in0_out = nodes.get(&in_ids[0]).map_or(
                        input[input.binary_search_by_key(&in_ids[0], |(a, _)| *a).unwrap()]
                            .1
                            .get_out(),
                        |a| a.out_wire(),
                    );
                    let in1_out = nodes.get(&in_ids[1]).map_or(
                        input[input.binary_search_by_key(&in_ids[1], |(a, _)| *a).unwrap()]
                            .1
                            .get_out(),
                        |a| a.out_wire(),
                    );
                    if let CircuitNode::OrNode(in0, in1, _) = nodes.get_mut(&out_id).unwrap() {
                        in0.replace(in0_out);
                        in1.replace(in1_out);
                    } else {
                        unreachable!()
                    }
                }
                yosys_parse::CellType::Nor => {
                    let in0_out = nodes.get(&in_ids[0]).map_or(
                        input[input.binary_search_by_key(&in_ids[0], |(a, _)| *a).unwrap()]
                            .1
                            .get_out(),
                        |a| a.out_wire(),
                    );
                    let in1_out = nodes.get(&in_ids[1]).map_or(
                        input[input.binary_search_by_key(&in_ids[1], |(a, _)| *a).unwrap()]
                            .1
                            .get_out(),
                        |a| a.out_wire(),
                    );
                    if let CircuitNode::NorNode(in0, in1, _) = nodes.get_mut(&out_id).unwrap() {
                        in0.replace(in0_out);
                        in1.replace(in1_out);
                    } else {
                        unreachable!()
                    }
                }
                yosys_parse::CellType::Xor => {
                    let in0_out = nodes.get(&in_ids[0]).map_or(
                        input[input.binary_search_by_key(&in_ids[0], |(a, _)| *a).unwrap()]
                            .1
                            .get_out(),
                        |a| a.out_wire(),
                    );
                    let in1_out = nodes.get(&in_ids[1]).map_or(
                        input[input.binary_search_by_key(&in_ids[1], |(a, _)| *a).unwrap()]
                            .1
                            .get_out(),
                        |a| a.out_wire(),
                    );
                    if let CircuitNode::XorNode(in0, in1, _) = nodes.get_mut(&out_id).unwrap() {
                        in0.replace(in0_out);
                        in1.replace(in1_out);
                    } else {
                        unreachable!()
                    }
                }
                yosys_parse::CellType::Nxor => {
                    let in0_out = nodes.get(&in_ids[0]).map_or(
                        input[input.binary_search_by_key(&in_ids[0], |(a, _)| *a).unwrap()]
                            .1
                            .get_out(),
                        |a| a.out_wire(),
                    );
                    let in1_out = nodes.get(&in_ids[1]).map_or(
                        input[input.binary_search_by_key(&in_ids[1], |(a, _)| *a).unwrap()]
                            .1
                            .get_out(),
                        |a| a.out_wire(),
                    );
                    if let CircuitNode::NxorNode(in0, in1, _) = nodes.get_mut(&out_id).unwrap() {
                        in0.replace(in0_out);
                        in1.replace(in1_out);
                    } else {
                        unreachable!()
                    }
                }
                yosys_parse::CellType::Not => {
                    let in0_out = nodes.get(&in_ids[0]).map_or(
                        input[input.binary_search_by_key(&in_ids[0], |(a, _)| *a).unwrap()]
                            .1
                            .get_out(),
                        |a| a.out_wire(),
                    );
                    if let CircuitNode::NotNode(in0, _) = nodes.get_mut(&out_id).unwrap() {
                        in0.replace(in0_out);
                    } else {
                        unreachable!()
                    }
                }
            }
        }
        for out_id in output_id {
            output.push((out_id, nodes.get(&out_id).unwrap().out_wire()));
        }
        output.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

        let mut graph = TaskGraph::new();
        let mut mem = HashMap::new();
        fn culc_<U: LogicOps + Send + Sync + 'static>(
            node: CircuitNode<U>,
        ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
            Box::pin(async move {
                node.culc().unwrap();
            })
        }
        for (out_id, node) in nodes.into_iter() {
            let index = graph.add_task(culc_(node)).ok()?;
            mem.insert(out_id, index);
        }
        for (_, cell) in module.cells.iter() {
            let out_id = cell.output_wireid();
            let in_ids = cell.input_wireids();
            match cell.type_name {
                yosys_parse::CellType::And
                | yosys_parse::CellType::Nand
                | yosys_parse::CellType::Or
                | yosys_parse::CellType::Nor
                | yosys_parse::CellType::Xor
                | yosys_parse::CellType::Nxor => {
                    let index_out = mem.get(&out_id).unwrap();
                    if let Some(index_in0) = mem.get(&in_ids[0]) {
                        graph.add_deps(&[index_in0.clone()], &index_out).ok()?;
                    }
                    if let Some(index_in1) = mem.get(&in_ids[1]) {
                        graph.add_deps(&[index_in1.clone()], &index_out).ok()?;
                    }
                }
                yosys_parse::CellType::Not => {
                    let index_out = mem.get(&out_id).unwrap();
                    if let Some(index_in0) = mem.get(&in_ids[0]) {
                        graph.add_deps(&[index_in0.clone()], &index_out).ok()?;
                    }
                }
            }
        }

        Some(Circuit {
            input,
            output,
            engine: graph,
        })
    }
}

pub struct WireIn<T>(Arc<RwLock<Option<Box<T>>>>);
impl<T> WireIn<T> {
    pub fn empty_wire() -> Self {
        WireIn(Arc::new(RwLock::new(None)))
    }
    /// this method return imediately
    pub fn try_write(&self, item: Box<T>) -> Result<Option<Box<T>>, &'static str> {
        let mut lock = self.0.try_write().map_err(|_| "cannot get lock")?;
        Ok(lock.replace(item))
    }
    /// this method block thread until get writelock
    pub fn write(&self, item: Box<T>) -> Result<Option<Box<T>>, &'static str> {
        let mut lock = self.0.write().map_err(|_| "lock poisond")?;
        Ok(lock.replace(item))
    }
    pub fn clear(&self) -> Result<Option<Box<T>>, &'static str> {
        let mut lock = self.0.write().map_err(|_| "lock poisond")?;
        Ok(lock.take())
    }
    pub fn get_out(&self) -> WireOut<T> {
        WireOut(self.0.clone())
    }
}
impl<T> Default for WireIn<T> {
    fn default() -> Self {
        Self::empty_wire()
    }
}
pub struct WireOut<T>(Arc<RwLock<Option<Box<T>>>>);
impl<T> WireOut<T> {
    /// this method return imediately
    pub fn try_read(&self) -> Result<RwLockReadGuard<Option<Box<T>>>, &'static str> {
        self.0.try_read().map_err(|_| "cannot get lock")
    }
    /// this method block thread until get readlock
    pub fn read(&self) -> Result<RwLockReadGuard<Option<Box<T>>>, &'static str> {
        self.0.read().map_err(|_| "lock poisond")
    }

    pub fn read_and_clear(&mut self) -> Result<Option<Box<T>>, &'static str> {
        let mut lock = self.0.try_write().map_err(|_| "lock error")?;
        Ok(lock.take())
    }
    pub fn is_empty(&self) -> bool {
        self.read().expect("poisoned").is_none()
    }
}
impl<T> Clone for WireOut<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub enum CircuitNode<T> {
    NandNode(Option<WireOut<T>>, Option<WireOut<T>>, WireIn<T>),
    AndNode(Option<WireOut<T>>, Option<WireOut<T>>, WireIn<T>),
    NorNode(Option<WireOut<T>>, Option<WireOut<T>>, WireIn<T>),
    OrNode(Option<WireOut<T>>, Option<WireOut<T>>, WireIn<T>),
    NxorNode(Option<WireOut<T>>, Option<WireOut<T>>, WireIn<T>),
    XorNode(Option<WireOut<T>>, Option<WireOut<T>>, WireIn<T>),
    NotNode(Option<WireOut<T>>, WireIn<T>),
}
impl<T> CircuitNode<T> {
    pub fn out_wire(&self) -> WireOut<T> {
        match self {
            CircuitNode::NandNode(_, _, out) => out.get_out(),
            CircuitNode::AndNode(_, _, out) => out.get_out(),
            CircuitNode::NorNode(_, _, out) => out.get_out(),
            CircuitNode::OrNode(_, _, out) => out.get_out(),
            CircuitNode::NxorNode(_, _, out) => out.get_out(),
            CircuitNode::XorNode(_, _, out) => out.get_out(),
            CircuitNode::NotNode(_, out) => out.get_out(),
        }
    }
    #[inline]
    fn culc_binary_ops_(
        rhs: &WireOut<T>,
        lhs: &WireOut<T>,
        out: &WireIn<T>,
        f: fn(&T, &T) -> T,
    ) -> Result<(), &'static str> {
        let res = {
            let rhs_lock = rhs.read()?;
            let lhs_lock = lhs.read()?;
            let rhs = rhs_lock.as_ref().unwrap();
            let lhs = lhs_lock.as_ref().unwrap();
            f(rhs, lhs)
        };
        out.write(Box::new(res))?;
        Ok(())
    }
    #[inline]
    fn culc_mono_ops_(
        input: &WireOut<T>,
        out: &WireIn<T>,
        f: fn(&T) -> T,
    ) -> Result<(), &'static str> {
        let res = {
            let input_lock = input.read()?;
            let input = input_lock.as_ref().unwrap();
            f(input)
        };
        out.write(Box::new(res))?;
        Ok(())
    }
    pub fn culc(&self) -> Result<(), &'static str>
    where
        T: LogicOps,
    {
        fn check<T>(x: &Option<WireOut<T>>) -> Result<&WireOut<T>, &'static str> {
            x.as_ref().ok_or("connection less")
        }
        match self {
            CircuitNode::NandNode(rhs, lhs, out) => {
                Self::culc_binary_ops_(check(rhs)?, check(lhs)?, out, T::lgc_nand)
            }
            CircuitNode::AndNode(rhs, lhs, out) => {
                Self::culc_binary_ops_(check(rhs)?, check(lhs)?, out, T::lgc_and)
            }
            CircuitNode::NorNode(rhs, lhs, out) => {
                Self::culc_binary_ops_(check(rhs)?, check(lhs)?, out, T::lgc_nor)
            }
            CircuitNode::OrNode(rhs, lhs, out) => {
                Self::culc_binary_ops_(check(rhs)?, check(lhs)?, out, T::lgc_or)
            }
            CircuitNode::NxorNode(rhs, lhs, out) => {
                Self::culc_binary_ops_(check(rhs)?, check(lhs)?, out, T::lgc_nxor)
            }
            CircuitNode::XorNode(rhs, lhs, out) => {
                Self::culc_binary_ops_(check(rhs)?, check(lhs)?, out, T::lgc_xor)
            }
            CircuitNode::NotNode(input, out) => {
                Self::culc_mono_ops_(check(input)?, out, T::lgc_not)
            }
        }
    }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::thread::sleep;

        #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
        async fn from_yosys_test() {
            

        }
}
