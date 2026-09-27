use std::vec::Vec;

use crate::value::Value;

pub struct Vm {
    stack: Vec<Value>,
}

impl Vm {
    const STACK_SIZE: usize = 0x1000;

    pub fn new() -> Self {
        Self {
            stack: Vec::with_capacity(Self::STACK_SIZE),
        }
    }
}
