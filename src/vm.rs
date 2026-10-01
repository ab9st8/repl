use crate::{
    code::{Chunk, Opcode},
    error::Error,
    value::Value,
};

pub struct Vm {
    stack_size: usize,
    pub stack: Vec<Value>,
}

type Result<T> = std::result::Result<T, Error>;

impl Vm {
    const DEFAULT_STACK_SIZE: usize = 0x1000;

    pub fn new() -> Self {
        Self::with_stack_size(Self::DEFAULT_STACK_SIZE)
    }

    pub fn with_stack_size(stack_size: usize) -> Self {
        Self {
            stack_size,
            stack: Vec::with_capacity(stack_size),
        }
    }

    pub fn push(&mut self, value: Value) -> Result<()> {
        if self.stack.len() == self.stack_size {
            return Err(Error::StackOverflow);
        }

        self.stack.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<Value> {
        self.stack.pop().ok_or(Error::StackUnderflow)
    }

    pub fn top(&self) -> Result<&Value> {
        self.stack.last().ok_or(Error::StackUnderflow)
    }

    pub fn run(&mut self, chunk: &Chunk) -> Result<()> {
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

    fn run(chunk: Chunk) -> Result<Vec<Value>> {
        let mut vm = Vm::new();
        vm.run(&chunk)?;
        Ok(vm.stack)
    }

    fn run_with_stack_size(stack_size: usize, chunk: Chunk) -> Result<Vec<Value>> {
        let mut vm = Vm::with_stack_size(stack_size);
        vm.run(&chunk)?;
        Ok(vm.stack)
    }

    #[test]
    fn op_push_pushes_constant() {
        let chunk = Chunk::new(
            vec![Value::from(12.0), Value::from("hello world")],
            vec![Opcode::Push(1), Opcode::Push(0)],
        );

        let stack = run(chunk).unwrap();
        assert_eq!(stack, vec![Value::from("hello world"), Value::from(12.0)],)
    }

    #[test]
    fn op_push_overflows_stack_on_full_stack() {
        let chunk = Chunk::new(
            vec![Value::from(42.0)],
            vec![Opcode::Push(0), Opcode::Push(0)],
        );

        let result = run_with_stack_size(1, chunk);
        assert_eq!(result, Err(Error::StackOverflow));
    }

    #[test]
    fn op_push_returns_bad_constant() {
        let chunk = Chunk::new(vec![Value::from(0.0)], vec![Opcode::Push(42)]);

        let result = run(chunk);
        assert_eq!(result, Err(Error::BadConstant(42)))
    }

    #[test]
    fn op_dup_duplicates_top_value() {
        let chunk = Chunk::new(vec![Value::from(42.0)], vec![Opcode::Push(0), Opcode::Dup]);

        let stack = run(chunk).unwrap();
        assert_eq!(stack, vec![Value::from(42.0), Value::from(42.0)],);
    }

    #[test]
    fn op_dup_overflows_stack_on_full_stack() {
        let chunk = Chunk::new(vec![Value::from(42.0)], vec![Opcode::Push(0), Opcode::Dup]);

        let result = run_with_stack_size(1, chunk);
        assert_eq!(result, Err(Error::StackOverflow));
    }

    #[test]
    fn op_dup_underflows_stack_on_empty_stack() {
        let chunk = Chunk::new(vec![], vec![Opcode::Dup]);

        let result = run(chunk);
        assert_eq!(result, Err(Error::StackUnderflow));
    }

    #[test]
    fn op_swap_swaps_top_two_values() {
        let chunk = Chunk::new(
            vec![Value::from(12.0), Value::from("hello world")],
            vec![Opcode::Push(0), Opcode::Push(1), Opcode::Swap],
        );

        let stack = run(chunk).unwrap();
        assert_eq!(stack, vec![Value::from("hello world"), Value::from(12.0)],);
    }

    #[test]
    fn op_swap_underflows_stack_on_one_value() {
        let chunk = Chunk::new(vec![Value::from(0.0)], vec![Opcode::Push(0), Opcode::Swap]);

        let result = run(chunk);
        assert_eq!(result, Err(Error::StackUnderflow));
    }
}
