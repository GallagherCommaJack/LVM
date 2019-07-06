pub enum Op {
    /// `ADD(l, r, o)` sets `o` to `l + r`, with overflow.
    Add,
    /// `MUL(l, r, o)` sets `o` to `l * r`, with overflow.
    Mul,
    /// `SUB(l, r, o)` sets `o` to `l - r`, with overflow.
    Sub,
    /// `UDIV(l, r, o)` sets `o` to the unsigned quotient of `l` by `r`.
    Udiv,
    /// `SDIV(l, r, o)` sets `o` to the signed quotient of `l` by `r`.
    Sdiv,
    /// `UMOD(l, r, o)` sets `o` to the unsigned modulus of `l` by `r`.
    Umod,
    /// `SMOD(l, r, o)` sets `o` to the signed modulus of `l` by `r`.
    Smod,
    /// `POW(b, e, o)` sets `o` to the value of `b` raised to the `e` power.
    Pow,
    /// `LT(l, r, o)` sets `o` to `TRUE` if `l < r`, otherwise false
    Lt,
    /// `GT(l, r, o)` sets `o` to `TRUE` if `l > r`, otherwise false
    Gt,
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
    /// starting from `l_o`.
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
    /// Assuming `f` is a (constant) valid function index,
    /// `i_1...i_n` and `o_1..o_m` are registers,
    /// and the function pointed to by `f` takes `n` inputs and returns `m` outputs,
    /// `CALL(f, i_1, i_2, ... i_n, o_1, o_2, ... o_m)`
    /// calls `f` on inputs `i_1..i_n` and stores the return values in `o_1..o_m`.
    /// Because each function has a known signature, we can check validity of this at compile time.
    Call,
    /// Like `CALL` but the first input is instead a register containing a function index.
    /// Because we don't know which function is going to be referenced in that register, we can't
    /// check at compile time that the right number of arguments were given. This is left up to the
    /// programmer / the higher-level language targetting this VM.
    CallI,
    /// `RET(r_1, r_2, ... r_n)` returns the values contained in registers `r_1..r_n`.
    /// Because each function has a known signature, we can check validity of this at compile time.
    Ret,
    /// `PANIC` aborts the program. If it has an argument, the contents of those registers serve as
    /// additional diagnostic information.
    Panic,
}

/// An instruction is an opcode and a vector of arguments.
pub struct Instr {
    pub op: Op,
    pub args: Vec<usize>,
}

/// The body of a function. Execution begins from the start of the first vector of instructions.
/// Jumping to marker `l` in `f: FnBody` begins execution from `f[l]`.
pub type FnBody = Vec<Vec<Instr>>;
