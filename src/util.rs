pub fn convert_string_to_u8(value: &str) -> u8 {
    match value.parse::<u8>() {
        Ok(parsed_value) => return parsed_value,
        Err(_) => panic!("Failed to parse value as u8: {}", value),
    }
}
