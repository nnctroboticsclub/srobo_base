use core::result::Result;
use core::result::Result::{Err, Ok};

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

#[derive(Debug)]
pub enum ParseError {
    InvalidData,
    IncompleteData,
}

pub fn u16(input: &[u8]) -> Result<(&[u8], u16), ParseError> {
    if input.len() < 4 {
        return Err(ParseError::IncompleteData);
    }

    let (num, rest) = input.split_at(4);

    let mut value = 0;
    for ch in num {
        if !ch.is_ascii_hexdigit() {
            return Err(ParseError::InvalidData);
        }

        let digit = if ch.is_ascii_digit() {
            ch - b'0'
        } else {
            ch.to_ascii_lowercase() - b'a' + 10
        };
        value = (value << 4) | digit as u16;
    }

    Ok((rest, value))
}

pub fn u8(input: &[u8]) -> Result<(&[u8], u8), ParseError> {
    if input.len() < 2 {
        return Err(ParseError::IncompleteData);
    }

    let (num, rest) = input.split_at(2);

    let mut value = 0;

    for ch in num {
        if !ch.is_ascii_hexdigit() {
            return Err(ParseError::InvalidData);
        }

        let digit = if ch.is_ascii_digit() {
            ch - b'0'
        } else {
            ch.to_ascii_lowercase() - b'a' + 10
        };

        value = (value << 4) | digit as u8;
    }

    Ok((rest, value))
}

pub fn u32(input: &[u8]) -> Result<(&[u8], u32), ParseError> {
    if input.len() < 8 {
        return Err(ParseError::IncompleteData);
    }

    let (num, rest) = input.split_at(8);

    let mut value = 0;
    for ch in num {
        if !ch.is_ascii_hexdigit() {
            return Err(ParseError::InvalidData);
        }

        let digit = if ch.is_ascii_digit() {
            ch - b'0'
        } else {
            ch.to_ascii_lowercase() - b'a' + 10
        };
        value = (value << 4) | digit as u32;
    }

    Ok((rest, value))
}

pub fn comma_separated_u8<'a>(
    input: &'a [u8],
    terminator: u8,
    dest: &mut [u8; 128],
) -> Result<(&'a [u8], usize), ParseError> {
    let mut rest = input;
    let mut i = 0;

    loop {
        let (new_rest, value) = u8(rest)?;
        rest = new_rest;

        dest[i] = value;

        if rest.len() == 0 {
            break;
        }

        if rest[0] == terminator {
            break;
        } else if rest[0] != b',' {
            return Err(ParseError::InvalidData);
        }

        rest = &rest[1..];
        i += 1;
    }

    Ok((rest, 1 + i))
}
