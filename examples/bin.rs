/*
    Disassembles a binary file
*/

extern crate mos6502;

use std::env;
use std::io::prelude::*;
use std::fs::File;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let mut f = File::open(file).unwrap();
    let mut buffer = Vec::<u8>::new();
    let _ = f.read_to_end(&mut buffer).unwrap();
    let instructions = mos6502::parse(buffer);
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
