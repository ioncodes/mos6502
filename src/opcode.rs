#[derive(PartialEq, Debug)]
pub enum Opcode {
    SEI,
    CLD,
    LDA,
    STA,
    LDX,
    TXS,
    BPL,
    Unknown,
}
