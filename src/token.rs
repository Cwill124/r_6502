use std::hash::Hash;
// TODO: REFACTOR THE TOKENS THAT DO NOT HAVE IMMEDIATE COMMAND TO HAVE ZERO PAGE AS DEFAULT

#[repr(u8)] // Optional, specifies the underlying representation of the enum (e.g., as a number)
#[derive(Clone, Debug)]
#[derive(Hash,PartialEq, Eq)]
pub enum Token {
    LDA = 0x89,
    LdaZP = 0xA5,
    LdaAP = 0xAD,
    LDX = 0xA2,
    LdxZP = 0xA6,
    LdxAP = 0xAE,
    LDY = 0xA0,
    LdyZP = 0xA4,
    LdyAP = 0xAC,
    ADC = 0x69,
    AdcZP = 0x65,
    AdcAP = 0x6D,
    STA = 0x95,
    StaAP = 0x8D,
    STX = 0x86,
    StxAP = 0x96,
    STY = 0x84,
    StyAP = 0x94,
    JMP = 0x4C,
    JmpID = 0x6C,
    JSR = 0x20,
    AND = 0x29,
    AndZP = 0x25,
    AndAP = 0x2D,
    ASL = 0x0A,
    AslZP = 0x06,
    AslAP = 0x0E,
    BCC = 0x90,
    BCS = 0xB0,
    BEQ = 0xF0,
    BIT = 0x24,
    BitAP = 0x2C,
    BMI = 0x30,
    BNE = 0xD0,
    BPL = 0x10,
    BRK = 0x00,
    BVC = 0x50,
    BVS = 0x70,
    CLC = 0x18,
    CLD = 0xD8,
    CLI = 0x58,
    CLV = 0xB8,
    CMP = 0xC9,
    CmpZP = 0xC5,
    CmpAP = 0xCD,
    CPX = 0xE0,
    CpxZP = 0xE4,
    CpxAP = 0xEC,
    CPY = 0xC0,
    CpyZP = 0xC4,
    CpyAP = 0xCC,
    DEC = 0xC6,
    DecAP = 0xCE,
    DEX = 0xCA,
    DEY = 0x88,
    EOR = 0x49,
    EorZP = 0x45,
    EorAP = 0x4D,
    INC = 0xE6,
    IncAP = 0xEE,
    INX = 0xE8,
    INY = 0xC8,
    LSR = 0x4A,
    LsrZP = 0x46,
    LsrAP = 0x4E,
    NOP = 0xEA,
    ORA = 0x09,
    OraZP = 0x05,
    OraAP = 0x0D,
    PHA = 0x48,
    PHP = 0x08,
    PLA = 0x68,
    PLP = 0x28,
    ROL = 0x2A,
    RolZP = 0x26,
    RolAP = 0x2E,
    ROR = 0x6A,
    RorZP = 0x66,
    RorAP = 0x6E,
    RTI = 0x40,
    RTS = 0x60,
    SBC = 0xE9,
    SbcZP = 0xE5,
    SbcAP = 0xED,
    SEC = 0x38,
    SED = 0xF8,
    SEI = 0x78,
    TAX = 0xAA,
    TAY = 0xA8,
    TSX = 0xBA,
    TXA = 0x8A,
    TXS = 0x9A,
    TYA = 0x98,
}

