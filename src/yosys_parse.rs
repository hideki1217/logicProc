use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct YosysRootElem {
    pub creator: String,
    pub modules: HashMap<String, ModuleElem>,
}

impl YosysRootElem {
    pub fn from_json(json: &str) -> Option<Self> {
        serde_json::from_str(json).ok()
    }
}

#[derive(Deserialize)]
pub struct ModuleElem {
    attributes: HashMap<String, String>,
    pub ports: HashMap<String, PortElem>,
    pub cells: HashMap<String, CellElem>,
    pub netnames: HashMap<String, NetNameElem>,
}

#[derive(Deserialize)]
pub struct NetNameElem {
    hide_name: i32,
    pub bits: Vec<u32>,
    attribute: serde_json::Value,
}

pub type WireId = u32;

#[derive(Deserialize, PartialEq, Clone, Copy)]
pub enum Direction {
    #[serde(rename = "input")]
    In,
    #[serde(rename = "output")]
    Out,
    #[serde(rename = "inout")]
    InOut,
}

#[derive(Deserialize)]
pub struct PortElem {
    pub direction: Direction,
    pub bits: Vec<WireId>,
}
impl PortElem {}

#[derive(Deserialize)]
pub enum CellType {
    #[serde(rename = "$_AND_")]
    And,
    #[serde(rename = "$_NAND_")]
    Nand,
    #[serde(rename = "$_OR_")]
    Or,
    #[serde(rename = "$_NOR_")]
    Nor,
    #[serde(rename = "$_XOR_")]
    Xor,
    #[serde(rename = "$_NXOR_")]
    Nxor,
    #[serde(rename = "$_NOT_")]
    Not,
}
#[derive(Deserialize)]
pub struct CellElem {
    pub hide_name: i32,
    #[serde(rename = "type")]
    pub type_name: CellType,
    parameters: serde_json::Value,
    attributes: serde_json::Value,
    pub port_directions: HashMap<String, Direction>,
    pub connections: HashMap<String, Vec<WireId>>,
}
impl CellElem {
    fn output_wirename(&self) -> &String {
        self.port_directions
            .iter()
            .filter(|(_, &d)| d == Direction::Out)
            .last()
            .unwrap()
            .0
    }
    pub fn output_wireids(&self) -> &Vec<WireId> {
        self.connections.get(self.output_wirename()).unwrap()
    }
    pub fn output_wireid(&self) -> WireId {
        *self.output_wireids().last().unwrap()
    }
    fn input_wirenames(&self) -> Vec<&String> {
        self.port_directions
            .iter()
            .filter(|(_, &d)| d == Direction::In)
            .map(|(x, y)| x)
            .collect()
    }
    pub fn input_wireids(&self) -> Vec<WireId> {
        self.input_wirenames()
            .iter()
            .map(|&x| *self.connections.get(x).unwrap().last().unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::YosysRootElem;

    #[test]
    fn serialize() {
        let json = include_str!("yosys_sample.v");
        let _res = YosysRootElem::from_json(json);
        println!("Deserilzie success!!");
    }
}
