#[repr(u8)]
pub enum OpCode {
    //> op-constant
    OpConstant = 1,
    //< op-constant
    //> Types of Values literal-ops
    OpNil,
    OpTrue,
    OpFalse,
    //< Types of Values literal-ops
    //> Global Variables pop-op
    OpPop,
    //< Global Variables pop-op
    //> Local Variables get-local-op
    OpGetLocal,
    //< Local Variables get-local-op
    //> Local Variables set-local-op
    OpSetLocal,
    //< Local Variables set-local-op
    //> Global Variables get-global-op
    OpGetGlobal,
    //< Global Variables get-global-op
    //> Global Variables define-global-op
    OpDefineGlobal,
    //< Global Variables define-global-op
    //> Global Variables set-global-op
    OpSetGlobal,
    //< Global Variables set-global-op
    //> Closures upvalue-ops
    OpGetUpvalue,
    OpSetUpvalue,
    //< Closures upvalue-ops
    //> Classes and Instances property-ops
    OpGetProperty,
    OpSetProperty,
    //< Classes and Instances property-ops
    //> Superclasses get-super-op
    OpGetSuper,
    //< Superclasses get-super-op
    //> Types of Values comparison-ops
    OpEqual,
    OpGreater,
    OpLess,
    //< Types of Values comparison-ops
    //> A Virtual Machine binary-ops
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    //> Types of Values not-op
    OpNot,
    //< Types of Values not-op
    //< A Virtual Machine binary-ops
    //> A Virtual Machine negate-op
    OpNegate,
    //< A Virtual Machine negate-op
    //> Global Variables op-print
    OpPrint,
    //< Global Variables op-print
    //> Jumping Back and Forth jump-op
    OpJump,
    //< Jumping Back and Forth jump-op
    //> Jumping Back and Forth jump-if-false-op
    OpJumpIfFalse,
    //< Jumping Back and Forth jump-if-false-op
    //> Jumping Back and Forth loop-op
    OpLoop,
}

pub struct Chunk {
    code: Vec<u8>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::<u8>::new(),
        }
    }

    pub fn disassemble_chunk(&self, name: &str) {
        println!("== {} ==", name);

        let mut offset: usize = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(&offset);
        }
    }
    fn disassemble_instruction(&self, offset: &usize) -> usize {
        print!("{} ", offset);

        return 0;
    }
}
