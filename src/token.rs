use std::collections::HashMap;

#[repr(u8)] // Optional, specifies the underlying representation of the enum (e.g., as a number)
#[derive(Clone, Debug)]
pub enum Token {
    LDA = 0x89,
}
