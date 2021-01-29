#![allow(non_snake_case)]

use std::collections::HashMap;

type Result<T> = std::result::Result<T, RomanError>;

#[derive(Debug, Eq, PartialEq)]
enum RomanError {
    DecodeIllegalCharacter(char),
    DecodeIllegalPosition { illegal: char, next: char },
}

fn decode(input: &str) -> Result<u16> {
    let mut last_value: u16 = 0;
    let mut ret: u16 = 0;

    let loookup_table = decode_lookup_table();

    for c in input.chars() {
        let value = loookup_table.get(&c)
            .ok_or(RomanError::DecodeIllegalCharacter(c))?;
        
        if last_value < *value {
            ret -= last_value;
            ret += value - last_value;
        } else {
            ret += value;
        }

        last_value = *value;
    }

    Ok(ret)
}

fn decode_lookup_table() -> HashMap<char, u16> {
    let mut ret = HashMap::new();
    ret.insert('I', 1);
    ret.insert('V', 5);
    ret.insert('X', 10);
    ret.insert('L', 50);
    ret.insert('C', 100);
    ret.insert('D', 500);
    ret.insert('M', 1000);
    ret
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_roman_decode_basics() -> Result<()> {
        assert_eq!(decode("I")?, 1);
        assert_eq!(decode("V")?, 5);
        assert_eq!(decode("X")?, 10);
        assert_eq!(decode("L")?, 50);
        assert_eq!(decode("C")?, 100);
        assert_eq!(decode("D")?, 500);
        assert_eq!(decode("M")?, 1000);
        Ok(())
    }

    #[test]
    fn test_roman_decode_MMMCDLVII() {
        let actual = decode("MMMCDLVII");
        assert_eq!(actual, Ok(3457));
    }

    #[test]
    fn test_roman_decode_MMMCMXCIX() {
        let actual = decode("MMMCMXCIX");
        assert_eq!(actual, Ok(3999));
    }

    #[test]
    fn test_roman_decode_error_illegal_character() {
        let actual = decode("MMMaC");
        assert_eq!(actual, Err(RomanError::DecodeIllegalCharacter('a')));
    }

    #[test]
    fn test_roman_decode_error_invalid_legal_character_in_illegal_position() {
        let actual = decode("IM");
        assert_eq!(actual, Err(RomanError::DecodeIllegalPosition { illegal: 'I', next: 'M' }));
    }
    
}
