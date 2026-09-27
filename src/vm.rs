use crate::{
    code::{Chunk, Opcode},
    error::Error,
    value::Value,
};

pub struct Vm {
    stack: Vec<Value>,
}

type Result<T> = std::result::Result<T, Error>;

impl Vm {
    const STACK_SIZE: usize = 0x1000;

    pub fn new() -> Self {
        Self {
            stack: Vec::with_capacity(Self::STACK_SIZE),
        }
    }

    pub fn push(&mut self, value: Value) -> Result<()> {
        if self.stack.len() == Self::STACK_SIZE {
            return Err(Error::StackOverflow);
        }

        self.stack.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<Value> {
        if self.stack.is_empty() {
            return Err(Error::StackUnderflow);
        }

        Ok(self.stack.pop().unwrap())
    }

    pub fn top(&self) -> Result<&Value> {
        self.stack.last().ok_or(Error::StackUnderflow)
    }

    pub fn run(&mut self, chunk: Chunk) -> Result<()> {
        let mut ip = 0;
        while ip < chunk.code.len() {
            let op = chunk.code[ip];
            ip += 1;

            match op {
                Opcode::Swap => {
                    let n = self.stack.len();
                    if n < 2 {
                        return Err(Error::StackUnderflow);
                    }

                    self.stack.swap(n - 1, n - 2);
                }
                Opcode::Dup => {
                    let top = self.top()?.clone();
                    self.push(top)?;
                }
                Opcode::Push(idx) => self.push(chunk.constant_at(idx)?)?,
            }
        }

        Ok(())
    }
}

impl Default for Vm {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO
}
