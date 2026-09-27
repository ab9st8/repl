use crate::value::Value;

pub enum Opcode {
    Push(usize),
    Swap,
    Dup,
}

pub struct Chunk {
    constants: Vec<Value>,
    code: Vec<Opcode>,
}
