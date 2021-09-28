extern crate num;
extern crate serde;

pub mod yosys_parse;

use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, RwLock, RwLockReadGuard},
};

use yosys_parse::YosysRootElem;

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
    output: Vec<WireOut<T>>,
    input: Vec<WireIn<T>>,
}
impl<T> Circuit<T> {
    pub fn set_input(&mut self, index: usize, val: Box<T>) -> Result<Option<Box<T>>, &'static str> {
        self.input.get_mut(index).unwrap().write(val)
    }
    pub fn culc_async(self) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
    where
        T: LogicOps + Send + Sync + 'static,
    {
        todo!()
    }
    pub fn from_yosys(json: &str) -> Option<Self>
    where
        T: LogicOps,
    {
        let yosys = YosysRootElem::from_json(json)?;
        let (_, module) = yosys.modules.iter().last().unwrap();

        let n = {
            module
                .netnames
                .iter()
                .flat_map(|(_, n)| n.bits.iter())
                .map(|x| *x)
                .max()
                .unwrap() as usize
        } + 1;

        let mut input: Vec<WireIn<T>> = Vec::new();
        let mut output: Vec<WireOut<T>> = Vec::new();
        let mut output_id: Vec<u32> = Vec::new();
        let mut input_id: Vec<u32> = Vec::new();
        unsafe {
            let mut nodes: Vec<CircuitNode<T>> = Vec::with_capacity(n);
            nodes.set_len(n);

            // nodesにCircuitNodeをセット
            for (_, port) in module.ports.iter() {
                match port.direction {
                    yosys_parse::Direction::In => {
                        for &id in port.bits.iter() {
                            let node = CircuitNode::PortNode(Default::default());
                            nodes[id as usize] = node;
                            input_id.push(id);
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
            for (_, cell) in module.cells.iter() {
                let out_id = cell.output_wireid() as usize;
                nodes[out_id] = match cell.type_name {
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
                }
            }
            // 各ノードの入力部をセット
            for (_, cell) in module.cells.iter() {
                let out_id = cell.output_wireid() as usize;
                let in_ids = cell.input_wireids();
                match cell.type_name {
                    yosys_parse::CellType::And => {
                        let in0_out = nodes[in_ids[0] as usize].out_wire();
                        let in1_out = nodes[in_ids[1] as usize].out_wire();
                        if let CircuitNode::AndNode(in0, in1, _) = nodes.get_mut(out_id).unwrap() {
                            in0.replace(in0_out);
                            in1.replace(in1_out);
                        } else {
                            unreachable!()
                        }
                    }
                    yosys_parse::CellType::Nand => {
                        let in0_out = nodes[in_ids[0] as usize].out_wire();
                        let in1_out = nodes[in_ids[1] as usize].out_wire();
                        if let CircuitNode::NandNode(in0, in1, _) = nodes.get_mut(out_id).unwrap() {
                            in0.replace(in0_out);
                            in1.replace(in1_out);
                        } else {
                            unreachable!()
                        }
                    }
                    yosys_parse::CellType::Or => {
                        let in0_out = nodes[in_ids[0] as usize].out_wire();
                        let in1_out = nodes[in_ids[1] as usize].out_wire();
                        if let CircuitNode::OrNode(in0, in1, _) = nodes.get_mut(out_id).unwrap() {
                            in0.replace(in0_out);
                            in1.replace(in1_out);
                        } else {
                            unreachable!()
                        }
                    }
                    yosys_parse::CellType::Nor => {
                        let in0_out = nodes[in_ids[0] as usize].out_wire();
                        let in1_out = nodes[in_ids[1] as usize].out_wire();
                        if let CircuitNode::NorNode(in0, in1, _) = nodes.get_mut(out_id).unwrap() {
                            in0.replace(in0_out);
                            in1.replace(in1_out);
                        } else {
                            unreachable!()
                        }
                    }
                    yosys_parse::CellType::Xor => {
                        let in0_out = nodes[in_ids[0] as usize].out_wire();
                        let in1_out = nodes[in_ids[1] as usize].out_wire();
                        if let CircuitNode::XorNode(in0, in1, _) = nodes.get_mut(out_id).unwrap() {
                            in0.replace(in0_out);
                            in1.replace(in1_out);
                        } else {
                            unreachable!()
                        }
                    }
                    yosys_parse::CellType::Nxor => {
                        let in0_out = nodes[in_ids[0] as usize].out_wire();
                        let in1_out = nodes[in_ids[1] as usize].out_wire();
                        if let CircuitNode::NxorNode(in0, in1, _) = nodes.get_mut(out_id).unwrap() {
                            in0.replace(in0_out);
                            in1.replace(in1_out);
                        } else {
                            unreachable!()
                        }
                    }
                    yosys_parse::CellType::Not => {
                        let in0_out = nodes[in_ids[0] as usize].out_wire();
                        if let CircuitNode::NotNode(in0, _) = nodes.get_mut(out_id).unwrap() {
                            in0.replace(in0_out);
                        } else {
                            unreachable!()
                        }
                    }
                }
            }

            /*             let graph = TaskGraph::new();
                        let mem = Vec::with_capacity(n);
                        mem.set_len(n);
                        for (name, cell) in module.cells.iter() {
                            let out_id = cell.output_wireid() as usize;
                            let in_ids = cell.input_wireids();
                            match cell.type_name {
                                yosys_parse::CellType::And => {
                                    let node = nodes[out_id];
                                    let index = graph.add_task(
                                        &[],
                                        Box::pin(async move {
                                            node.culc();
                                        }),
                                    ).ok()?;
                                },
                                yosys_parse::CellType::Nand => todo!(),
                                yosys_parse::CellType::Or => todo!(),
                                yosys_parse::CellType::Nor => todo!(),
                                yosys_parse::CellType::Xor => todo!(),
                                yosys_parse::CellType::Nxor => todo!(),
                                yosys_parse::CellType::Not => todo!(),
                            }
                        }
            */
            Some(Circuit {
                input,
                output,
                //executer: graph,
            })
        }
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
    PortNode(WireIn<T>),
}
impl<T> CircuitNode<T> {
    pub fn out_wire(&self) -> WireOut<T> {
        match self {
            CircuitNode::NandNode(_, _, out) => out.get_out(),
            CircuitNode::AndNode(_, _, out) => out.get_out(),
            CircuitNode::NorNode(_, _, out) => out.get_out(),
            CircuitNode::OrNode(_, _,out) => out.get_out(),
            CircuitNode::NxorNode(_, _, out) => out.get_out(),
            CircuitNode::XorNode(_, _, out) => out.get_out(),
            CircuitNode::NotNode(_, out) => out.get_out(),
            CircuitNode::PortNode(out) => out.get_out(),
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
            CircuitNode::PortNode(_) => Ok(()),
        }
    }
    /*
        #[inline]
        fn culc_async_binary_ops(
            rhs: &mut Option<WireOut<T>>,
            lhs: &mut Option<WireOut<T>>,
            out: &WireIn<T>,
            f: fn(&T, &T) -> T,
        ) -> BoxFuture<()>
        where
            T: LogicOps + Send + Sync + 'static,
        {
            Box::pin(
                async {
                    let rhs = rhs.as_mut().ok_or("connection less").unwrap();
                    let lhs = lhs.as_mut().ok_or("connection less").unwrap();
                    let rhs_ref = rhs.read_on_updated().await.unwrap();
                    let lhs_ref = lhs.read_on_updated().await.unwrap();
                    let rhs = rhs_ref.as_ref().unwrap();
                    let lhs = lhs_ref.as_ref().unwrap();
                    let res = f(rhs, lhs);
                    out.write(Box::new(res)).map_err(|_| "errored").unwrap();
                }
            )
        }

        #[inline]
        fn culc_async_mono_ops(
            input: &mut Option<WireOut<T>>,
            out: &WireIn<T>,
            f: fn(&T) -> T,
        ) -> BoxFuture<()>
        where
            T: LogicOps + Send + Sync + 'static,
        {
            Box::pin(
                async {
                    let input = input.as_mut().ok_or("connection less").unwrap();
                    let input_ref = input.read_on_updated().await.unwrap();
                    let input = input_ref.as_ref().unwrap();
                    let res = f(input);
                    out.write(Box::new(res)).map_err(|_| "errored").unwrap();
                }
            )
        }
        pub fn culc_async(mut self) -> BoxFuture<()>
        where
            T: LogicOps + Send + Sync + 'static,
        {
            fn check_connection<'a,'b,U>(lhs:&'a mut Option<WireOut<U>>,rhs:&'b mut Option<WireOut<U>>) -> (&'a mut WireOut<U>,&'b mut WireOut<U>) {
                let rhs = rhs.as_mut().ok_or("connection less").unwrap();
                let lhs = lhs.as_mut().ok_or("connection less").unwrap();
                (lhs,rhs)
            }
            match self {
                CircuitNode::NandNode(rhs, lhs, out) => {
                    Self::culc_async_binary_ops(&mut rhs, &mut lhs, &out, T::lgc_nand)

                }
                CircuitNode::AndNode(rhs, lhs, out) => {
                    Self::culc_async_binary_ops(&mut rhs, &mut lhs, &out, T::lgc_an)
                }
                CircuitNode::NorNode(rhs, lhs, out) => {
                    Self::culc_async_binary_ops(&mut rhs, &mut lhs, &out, T::lgc_nor)
                }
                CircuitNode::OrNode(rhs, lhs, out) => {
                    Self::culc_async_binary_ops(&mut rhs, &mut lhs, &out, T::lgc_or)
                }
                CircuitNode::NxorNode(rhs, lhs, out) => {
                    Self::culc_async_binary_ops(&mut rhs, &mut lhs, &out, T::lgc_nxor)
                }
                CircuitNode::XorNode(rhs, lhs, out) => {
                    Self::culc_async_binary_ops(&mut rhs, &mut lhs, &out, T::lgc_xor)
                }
                CircuitNode::NotNode(input, out) => {
                    Self::culc_async_mono_ops(&mut input, &out, T::lgc_not)
                }
                CircuitNode::PortNode(_) => {},
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::thread::sleep;

        #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
        async fn tokio_watch_sample() {
            let (tx, mut rx) = tokio::sync::watch::channel("hello");

            let mut rx2 = rx.clone();
            let t = tokio::task::spawn(async move {
                let mut buffer = String::new();
                while rx.changed().await.is_ok() {
                    if (*rx.borrow()).starts_with("end") {
                        break;
                    }
                    buffer.push_str(*rx.borrow());
                }
                buffer
            });
            let s = tokio::task::spawn(async move {
                let mut buffer = String::new();
                while rx2.changed().await.is_ok() {
                    if (*rx2.borrow()).starts_with("endl") {
                        break;
                    }
                    buffer.push_str(*rx2.borrow());
                }
                buffer
            });

            let dur: u64 = 10;

            let res = tx.send("world");
            sleep(std::time::Duration::from_millis(dur));
            let _ = tx.send("a");
            sleep(std::time::Duration::from_millis(dur));
            let _ = tx.send("h");
            sleep(std::time::Duration::from_millis(dur));
            let _ = tx.send("o");
            sleep(std::time::Duration::from_millis(dur));
            let _ = tx.send("end");
            sleep(std::time::Duration::from_millis(dur));
            let _ = tx.send("endl");

            match t.await {
                Ok(s) => println!("res = {}", s),
                Err(_) => {}
            };
            match s.await {
                Ok(s) => println!("res = {}", s),
                Err(_) => {}
            };
        }
        */
}
