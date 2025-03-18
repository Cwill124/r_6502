pub fn convert_string_to_u8(value: &str) -> u8 {
    match value.parse::<u8>() {
        Ok(parsed_value) => return parsed_value,
        Err(_) => panic!("Failed to parse value as u8: {}", value),
    }
}
pub fn convert_hex_string_to_u8(value: &str) -> u8 {
    let u8_value = u8::from_str_radix(value, 16)
        .unwrap_or_else(|_| panic!("Failed to parse hex value: {}", value));
    u8_value
}
pub fn is_zero_page(value: &str) -> bool {
    let converted_value = u16::from_str_radix(value, 16)
        .unwrap_or_else(|_| panic!("Failed to parse hex value: {}", value));
    return converted_value < 256;
}
pub fn convert_string_to_u16(value: &str) -> u8 {
    match value.parse::<u8>() {
        Ok(parsed_value) => return parsed_value,
        Err(_) => panic!("Failed to parse value as u8: {}", value),
    }
}
