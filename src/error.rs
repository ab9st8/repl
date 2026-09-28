#[derive(Debug, PartialEq)]
pub enum Error {
    BadConstant(usize),
    StackOverflow,
    StackUnderflow,
}
