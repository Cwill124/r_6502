#[repr(u8)] // Optional, specifies the underlying representation of the enum (e.g., as a number)
#[derive(Clone, Debug)]
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
}
