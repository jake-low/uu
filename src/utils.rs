use unic::char::basics;
use unic::ucd;

use ascii::{self, AsciiChar};
use itertools::Itertools;

/// Returns a string containing the input char, or an ASCII representation of it
/// if it's a control character.
pub fn repr(c: char) -> String {
    if let Some(ascii_char) = AsciiChar::from_ascii(c).ok() {
        if let Some(caret_escape) = ascii::caret_encode(ascii_char) {
            return format!("^{}", caret_escape.as_char());
        }
    }
    return c.to_string();
}

/// Returns a string in U+XXXX format representing the code point for the given Unicode character.
pub fn codepoint(c: char) -> String {
    // format!("U+{:0>4x}", c as u32)
    basics::notation::unicode_notation(c).to_string()
}

/// Returns a string in hexadecimal (with spaces), representing the bytes
/// that encode this Unicode character in UTF-8.
pub fn char_to_bytes_utf8(c: char) -> String {
    let mut b = [0; 4];
    let bytes = c.encode_utf8(&mut b).as_bytes();

    format!("{:0>2x}", bytes.iter().format(" "))
}

/// Returns a string in hexadecimal (with spaces), representing the bytes
/// that encode this Unicode character in UTF-16 LE.
pub fn char_to_bytes_utf16le(c: char) -> String {
    let mut buf: [u16; 2] = [0; 2];
    let utf16 = c.encode_utf16(&mut buf);
    let bytes: Vec<u8> = utf16
        .iter()
        .map(|i| i.to_le_bytes().to_vec())
        .flatten()
        .collect();

    format!("{:0>2x}", bytes.iter().format(" "))
}

/// Returns a string in hexadecimal (with spaces), representing the bytes
/// that encode this Unicode character in UTF-16 BE.
pub fn char_to_bytes_utf16be(c: char) -> String {
    let mut buf: [u16; 2] = [0; 2];
    let utf16 = c.encode_utf16(&mut buf);
    let bytes: Vec<u8> = utf16
        .iter()
        .map(|i| i.to_be_bytes().to_vec())
        .flatten()
        .collect();

    format!("{:0>2x}", bytes.iter().format(" "))
}

/// Returns a string in hexadecimal (with spaces), representing the bytes
/// that encode this Unicode character in UTF-32 LE.
pub fn char_to_bytes_utf32le(c: char) -> String {
    let bytes = (c as u32).to_le_bytes();
    format!("{:0>2x}", bytes.iter().format(" "))
}

/// Returns a string in hexadecimal (with spaces), representing the bytes
/// that encode this Unicode character in UTF-32 LE.
pub fn char_to_bytes_utf32be(c: char) -> String {
    let bytes = (c as u32).to_be_bytes();
    format!("{:0>2x}", bytes.iter().format(" "))
}

pub fn name(c: char) -> String {
    ucd::Name::of(c)
        .map(|name| name.to_string())
        .unwrap_or_default()
}

pub fn name_or_alias(c: char) -> String {
    if let Some(correction) = ucd::name_aliases_of(c, ucd::NameAliasType::NameCorrections)
        .unwrap_or_default()
        .first()
    {
        return format!("* {}", correction);
    }

    if let Some(control) = ucd::name_aliases_of(c, ucd::NameAliasType::ControlCodeNames)
        .unwrap_or_default()
        .first()
    {
        return format!("<{}>", control);
    }

    return name(c);
}
