/// Each stack frame has an array of associated registers. These each have a known, static size.
/// We do not enforce any particular constraints on register size.
pub type Frame = Vec<Box<[u8]>>;
/// A stack is just a `Vec` of `Frame`s.
pub type Stack = Vec<Frame>;
