use opcode::Opcode;

pub struct Instruction {
    pub opcode: Opcode,
    pub operand: String,
    pub hex: Vec<u8>,
}

impl Instruction {
    pub fn new(opcode: Opcode, operand: String, hex: Vec<u8>) -> Instruction {
        Instruction {
            opcode,
            operand,
            hex,
        }
    }
}
