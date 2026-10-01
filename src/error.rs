#[derive(Debug, PartialEq)]
pub enum Error {
    BadConstant(usize),
    StackOverflow,
    StackUnderflow,
}

pub type Result<T> = std::result::Result<T, Error>;
