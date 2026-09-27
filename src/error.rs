pub enum Error {
    BadConstant(usize),
    OldChunk,
    StackOverflow,
    StackUnderflow,
}
