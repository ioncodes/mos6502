use opcode::Opcode;

pub fn resolve(opcode: u8) -> (Opcode, usize) {
    match opcode {
        0x78 => (Opcode::SEI, 1),
        0xD8 => (Opcode::CLD, 1),
        0xA9 => (Opcode::LDA, 2),
        0x8D => (Opcode::STA, 3),
        0xA2 => (Opcode::LDX, 2),
        0x9A => (Opcode::TXS, 1),
        0xAD => (Opcode::LDA, 3),
        0x10 => (Opcode::BPL, 2),
        _ => (Opcode::Unknown, 1),
    }
}
