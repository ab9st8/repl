use crate::value::Value;

#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    Push(usize),
    Swap,
    Dup,
}

#[derive(Default)]
pub struct Chunk {
    constants: Vec<Value>,
    pub code: Vec<Opcode>,
}

impl Chunk {
    pub fn constant_at(&self, idx: usize) -> Value {
        self.constants[idx].clone()
    }
}
