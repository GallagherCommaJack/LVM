use crate::instr::*;
use std::convert::TryInto;
use std::ops::{Add, Deref, Mul, Sub};

/// Each stack frame has an array of associated registers. These each have a known, static size.
/// We do not enforce any particular constraints on register size.
#[derive(Eq, PartialEq, Debug)]
pub struct Frame {
    curr: TopFrame,
    awaiting: Vec<usize>,
}
pub type TopFrame = Vec<Box<[u8]>>;

/// A stack is just a `Vec` of `Frame`s.
#[derive(Eq, PartialEq, Debug)]
pub struct Stack {
    top: TopFrame,
    rest: Vec<Frame>,
}

macro_rules! arop_case_macro {
    ($tyname: tt, $len: tt, $stack: ident, $args: ident, $op: tt) => {
        let rl = &$stack.top[$args[0]];
        let rr = &$stack.top[$args[1]];
        assert!(rl.len() == $len);
        assert!(rr.len() == $len);
        assert!($stack.top[$args[2]].len() == $len);
        let al: Option<&[u8; $len]> = rl.deref().try_into().ok();
        let ar: Option<&[u8; $len]> = rr.deref().try_into().ok();
        let vl: $tyname = $tyname::from_le_bytes(al.unwrap().clone());
        let vr: $tyname = $tyname::from_le_bytes(ar.unwrap().clone());
        $stack.top[$args[2]] = Box::new($tyname::to_le_bytes(vl.$op(vr)));
    };
}

macro_rules! arop_macro {
    ($stack: ident, $args: ident, $ty: ident, $op: tt) => {
        match $ty {
            U8 => {
                arop_case_macro!(u8, 1, $stack, $args, $op);
            }
            U16 => {
                arop_case_macro!(u16, 2, $stack, $args, $op);
            }
            U32 => {
                arop_case_macro!(u32, 4, $stack, $args, $op);
            }
            U64 => {
                arop_case_macro!(u64, 8, $stack, $args, $op);
            }
            I8 => {
                arop_case_macro!(i8, 1, $stack, $args, $op);
            }
            I16 => {
                arop_case_macro!(i16, 2, $stack, $args, $op);
            }
            I32 => {
                arop_case_macro!(i32, 4, $stack, $args, $op);
            }
            I64 => {
                arop_case_macro!(i64, 8, $stack, $args, $op);
            }
            _ => panic!("todo"),
        }
    };
}

impl Stack {
    pub fn run_op(&mut self, instr: &Instr) -> Option<TopFrame> {
        use Op::*;
        use Type::*;

        let Instr { op, args } = instr;
        match (op, args.len()) {
            (Add(t), 3) => {
                arop_macro!(self, args, t, add);
            }
            (Mul(t), 3) => {
                arop_macro!(self, args, t, mul);
            }
            (Sub(t), 3) => {
                arop_macro!(self, args, t, sub);
            }
            _ => panic!("todo"),
        };
        None
    }
}
