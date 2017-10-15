/*
    Reads defined bytes taken from from "Super Mario Bros (E)" with SHA1 "AB30029EFEC6CCFC5D65DFDA7FBC6E6489A80805"
*/

extern crate mos6502;

pub fn main() {
    let instructions = mos6502::parse(vec![
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
        print!(":\t");
        print!("{:?}", instructions[i].opcode);
        if instructions[i].operand != "" {
            print!(" {}", instructions[i].operand);
        }
        println!("");
    }
}
