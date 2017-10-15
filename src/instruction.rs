use opcode::Opcode;

pub struct Instruction {
    pub opcode: Opcode,
    pub operand: String,
    pub hex: Vec<u8>,
    pub dasm: String,
}
