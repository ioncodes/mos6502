mod opcode;
mod instruction;
mod table;

use instruction::Instruction;

pub fn parse(hex: Vec<u8>) -> Vec<Instruction> {
    let mut instructions = Vec::<Instruction>::new();
    let mut pc = 0x0; // emulates program counter
    loop {
        let byte = hex[pc];
        let (opcode, size, dasm) = table::resolve(byte);
        let mut bytes = Vec::<u8>::new();
        println!("{:X}", byte);
        bytes.push(byte);
        for i in 1..size {
            bytes.push(hex[pc + i]);
        }
        let mut operand = String::new();
        if bytes.len() > 1 {
            if bytes.len() > 2 {
                operand.push('$');
            } else {
                operand.push_str("#$");
            }
            for byte in &bytes[1..bytes.len()] {
                operand.push_str(&format!("{:02X}", byte));
            }
        }
        instructions.push(Instruction {
            opcode,
            operand,
            hex: bytes,
            dasm
        });
        pc += size;

        println!("{:X}", pc);

        if pc == hex.len() {
            break;
        }
    }

    instructions
}

#[cfg(test)]
mod tests {
    use super::*;
    use opcode::Opcode;

    #[test]
    fn internal() {
        // first sequence of bytes from "Super Mario Bros (E)" with SHA1 "AB30029EFEC6CCFC5D65DFDA7FBC6E6489A80805"
        let instructions = parse(vec![
            0x78,
            0xD8,
            0xA9,
            0x10,
            0x8D,
            0x00,
            0x20,
            0xA2,
            0xFF,
            0x9A,
            0xAD,
            0x02,
            0x20,
            0x10,
            0xFB,
            0xAD,
            0x02,
            0x20,
        ]);
        for i in 0..instructions.len() {
            for j in 0..instructions[i].hex.len() {
                print!("{:02X}", instructions[i].hex[j]);
            }
            println!(":\t{}", instructions[i].dasm.replace("{}", &instructions[i].operand));
        }
        assert_eq!(9, instructions.len());
        assert_eq!(Opcode::SEI, instructions[0].opcode);
        assert_eq!(Opcode::CLD, instructions[1].opcode);
        assert_eq!(Opcode::LDA, instructions[2].opcode);
        assert_eq!(Opcode::STA, instructions[3].opcode);
        assert_eq!(Opcode::LDX, instructions[4].opcode);
        assert_eq!(Opcode::TXS, instructions[5].opcode);
        assert_eq!(Opcode::LDA, instructions[6].opcode);
        assert_eq!(Opcode::BPL, instructions[7].opcode);
        assert_eq!(Opcode::LDA, instructions[8].opcode);
    }
}
