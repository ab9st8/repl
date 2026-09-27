use std::vec::Vec;

use crate::{
    code::{Chunk, Opcode},
    error::Error,
    value::Value,
};

pub struct Vm {
    stack: Vec<Value>,
    pub chunk: Chunk,
}

type Result<T> = std::result::Result<T, Error>;

impl Vm {
    const STACK_SIZE: usize = 0x1000;

    pub fn new(chunk: Chunk) -> Self {
        Self {
            stack: Vec::with_capacity(Self::STACK_SIZE),
            chunk,
        }
    }

    pub fn push(&mut self, value: Value) -> Result<()> {
        if self.stack.len() == Self::STACK_SIZE {
            return Err(Error::StackOverflow);
        }

        self.stack.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.stack.pop()
    }

    pub fn top(&self) -> Option<&Value> {
        self.stack.last()
    }

    pub fn run(&mut self) -> Result<()> {
        if self.chunk.consumed {
            return Err(Error::OldChunk);
        }

        let mut ip = 0;
        while ip < self.chunk.code.len() {
            let op = self.chunk.code[ip];
            ip += 1;

            match op {
                Opcode::Swap => {
                    let (a, b) = (self.pop(), self.pop());
                    match (a, b) {
                        (Some(val_a), Some(val_b)) => {
                            self.push(val_a)?;
                            self.push(val_b)?;
                        }
                        _ => return Err(Error::StackUnderflow),
                    }
                }
                Opcode::Dup => {
                    if let Some(top) = self.top() {
                        if self.push(top.clone()).is_err() {
                            return Err(Error::StackOverflow);
                        }
                    } else {
                        return Err(Error::StackUnderflow);
                    }
                }
                Opcode::Push(idx) => self.push(self.chunk.constant_at(idx))?,
            }
        }

        self.chunk.consumed = true;
        Ok(())
    }
}
