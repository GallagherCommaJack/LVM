#[derive(Eq, PartialEq, Debug)]
pub enum Type {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    // should we have floats?
    // F32,
    // F64,
    Bool,
    Bytes(usize),
}

#[derive(Eq, PartialEq, Debug)]
pub enum Op {
    /// `ADD(l, r, o)` sets `o` to `l + r`, with overflow.
    Add(Type),
    /// `MUL(l, r, o)` sets `o` to `l * r`, with overflow.
    Mul(Type),
    /// `SUB(l, r, o)` sets `o` to `l - r`, with overflow.
    Sub(Type),
    /// `UDIV(l, r, o)` sets `o` to the unsigned quotient of `l` by `r`.
    Udiv(Type),
    /// `SDIV(l, r, o)` sets `o` to the signed quotient of `l` by `r`.
    Sdiv(Type),
    /// `UMOD(l, r, o)` sets `o` to the unsigned modulus of `l` by `r`.
    Umod(Type),
    /// `SMOD(l, r, o)` sets `o` to the signed modulus of `l` by `r`.
    Smod(Type),
    /// `POW(b, e, o)` sets `o` to the value of `b` raised to the `e` power.
    Pow(Type),
    /// `LT(l, r, o)` sets `o` to `TRUE` if `l < r`, otherwise false
    Lt(Type),
    /// `GT(l, r, o)` sets `o` to `TRUE` if `l > r`, otherwise false
    Gt(Type),
    /// `EQ(l, r, o)` sets `o` to `TRUE` if `l` and `r` are bitwise equal, otherwise false
    Eq,
    /// `AND(l, r, o)` stores the bitwise and of `l` and `r` at `o`.
    And,
    /// `OR(l, r, o)` stores the bitwise or of `l` and `r` at `o`.
    Or,
    /// `XOR(l, r, o)` stores the bitwise xor of `l` and `r` at `o`.
    Xor,
    /// `NOT(i, o)` stores the bitwise negation of `i` in `o`.
    Not,
    /// `SHL(i, s, o)` stores the result of shifting `i` to the left by `s` bits in `o`.
    Shl,
    /// `SHR(i, s, o)` stores the result of shifting `i` to the right by `s` bits in `o`.
    Shr,
    /// `SHA3(i, o)` stores the sha3 hash code of the contents of `i` in `o`.
    Sha3,
    /// `COPY(i, l_i, r_i, o, l_o)` copies the slice of bytes from `l_i` to `r_i` from `i` to `o`,
    /// starting from `l_o`. In case that was hard to understand, here's an ASCII diagram:
    ///
    /// ```text
    /// +-----+-----+---+    +-----+-----+--------+
    /// |  l_i|  V  |r_i|    |  l_o|  V  |        |
    /// +-----+-----+---+    +-----+-----+--------+
    ///       |     |              |     |
    ///       |      \            /      |
    ///       |        -----------       |
    ///        \                        /
    ///         ------------------------
    /// ```
    Copy,
    /// `LOAD(i, o)` fills `o` with the contents of memory starting at `i`.
    Load,
    /// `STORE(i, o)` stores the contents of `i` in memory, starting at address `o`.
    Store,
    /// `JUMP(l)` goes to marker `l` in the current function.
    ///
    /// `JUMP(f, l)` goes to marker `l` in the function `f`.
    Jump,
    /// `JUMPIF(c, l)` goes to marker `l` in the current function if `c` is true,
    /// otherwise continues with execution.
    ///
    /// `JUMPIF(c, f, l)` goes to marker `l` in the function `f` if `c` is true, otherwise
    /// continues with execution.
    JumpIf,
    /// Call is a varargs instruction.
    /// Assuming the following:
    ///
    /// - `f` is a (constant) valid function index,
    /// - `i_1..i_n` and `o_1..o_m` are registers,
    /// - the function pointed to by `f` takes `n` inputs of the same respective sizes as
    /// `i_1..i_n`,
    /// - the function pointed to by `f` returns `m` outputs of the same respective sizes as
    /// `o_1..o_m`,
    ///
    /// `CALL(f, i_1, i_2, ... i_n, o_1, o_2, ... o_m)`
    /// calls `f` on inputs `i_1..i_n` and stores the return values in `o_1..o_m`.
    ///
    /// Because each function has a known signature, we can check validity of this at compile time.
    Call,
    /// `CALLI(r_f, r_1, r_2, .. r_n)` attempts to call the function referenced by the value
    /// in `r_f` on registers `1..n`. Unlike with `CALL`, we can only check the validity of the
    /// arguments at runtime.
    CallI,
    /// `RET(r_1, r_2, ... r_n)` returns the values contained in registers `r_1..r_n`.
    /// Because each function has a known signature, we can check validity of this at compile time.
    Ret,
    /// `PANIC` aborts the program. If it has an argument, the contents of those registers serve as
    /// additional diagnostic information.
    Panic,
}

/// An instruction is an opcode and a vector of arguments.
#[derive(Eq, PartialEq, Debug)]
pub struct Instr {
    pub op: Op,
    pub args: Vec<usize>,
}

/// The body of a function. Execution begins from the start of the first vector of instructions.
/// Jumping to marker `l` in `f: FnBody` begins execution from `f[l]`.
pub type FnBody = Vec<Vec<Instr>>;
