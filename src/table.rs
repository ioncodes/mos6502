use opcode::Opcode;

pub fn resolve(opcode: u8) -> (Opcode, usize, String) {
    let m = match opcode {
        0x78 => (Opcode::SEI, 1, "SEI"),
        0xD8 => (Opcode::CLD, 1, "CLD"),
        0xA9 => (Opcode::LDA, 2, "LDA {}"),
        0x8D => (Opcode::STA, 3, "STA {}"),
        0xA2 => (Opcode::LDX, 2, "LDX {}"),
        0x9A => (Opcode::TXS, 1, "TXS"),
        0xAD => (Opcode::LDA, 3, "LDA {}"),
        0x10 => (Opcode::BPL, 2, "BPL {}"),
        0x24 => (Opcode::BIT, 2, "BIT {}"),
        0x11 => (Opcode::ORA, 2, "ORA {}"),
        0x25 => (Opcode::AND, 2, "AND {}"),
        0x00 => (Opcode::BRK, 1, "BRK"),
        0x16 => (Opcode::ASL, 3, "ASL {}, X"),
        0x18 => (Opcode::CLC, 1, "CLC"),
        0x0A => (Opcode::ASL, 1, "ASL"), // ASL A
        0x30 => (Opcode::BMI, 2, "BMI {}"),
        0x36 => (Opcode::ROL, 2, "ROL {}"),
        0x20 => (Opcode::JSR, 3, "JSR {}"),
        0x9D => (Opcode::STA, 3, "STA {}, X"),
        0x60 => (Opcode::RTS, 1, "RTS"),
        0x68 => (Opcode::PLA, 1, "PLA"),
        0xF0 => (Opcode::BEQ, 2, "BEQ {}"),
        0x48 => (Opcode::PHA, 1, "PHA"),
        0xD0 => (Opcode::BNE, 2, "BNE {}"),
        0x88 => (Opcode::DEY, 1, "DEY"),
        0x3D => (Opcode::AND, 3, "AND {}, X"),
        0x4A => (Opcode::LSR, 1, "LSR"),
        0x29 => (Opcode::AND, 2, "AND {}"),
        0x2A => (Opcode::ROL, 1, "ROL A"),
        0x05 => (Opcode::ORA, 2, "ORA {}"),
        0xBD => (Opcode::LDA, 3, "LDA {}, X"),
        0xE8 => (Opcode::INX, 1, "INX"),
        0xAA => (Opcode::TAX, 1, "TAX"),
        0x85 => (Opcode::STA, 2, "STA {}"),
        0xA0 => (Opcode::LDY, 2, "LDY {}"),
        _ => (Opcode::Unknown, 1, "Unknown"),
    };
    (m.0, m.1, m.2.to_string())
}
