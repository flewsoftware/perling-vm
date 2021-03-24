#[derive(Debug, PartialEq)]
pub enum Opcode {
    HLT,    // halt
    IGL,    // ILLEGAL
    LOAD,   // load
    ADD,    // addition
    SUB,    // subtraction
    DIV,    // division
    JMP,    // jump
    RJMP,   // relative jump
    JMPTL,  // jump to label
    VMCALL, // run commands in VM
    EQ,     // checks if equal
    JEQ,    // jumps if true
    NEQ,    // checks if not equal
    JNEQ,   // jumps if not true
    SWP,    // swaps two register values
    AND,    // AND
    OR,     // OR
    NOT,    // NOT
    GET,    // mv a value from a hidden register to a normal register
    LOCKR,  // locks a register similar to a constant
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode }
    }
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Opcode {
        match v {
            0 => Opcode::HLT,
            1 => Opcode::LOAD,

            2 => Opcode::ADD,
            3 => Opcode::SUB,
            4 => Opcode::DIV,

            5 => Opcode::JMP,
            6 => Opcode::RJMP,
            7 => Opcode::JMPTL,

            8 => Opcode::VMCALL,

            9 => Opcode::EQ,
            10 => Opcode::JEQ,
            11 => Opcode::NEQ,
            12 => Opcode::JNEQ,
            13 => Opcode::SWP,

            14 => Opcode::AND,
            15 => Opcode::OR,
            16 => Opcode::NOT,

            17 => Opcode::GET,
            18 => Opcode::LOCKR,
            _ => Opcode::IGL,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}
