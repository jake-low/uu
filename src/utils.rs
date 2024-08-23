use unic::char::property::EnumeratedCharProperty;
use unic::ucd;

use ascii::{self, AsciiChar};
use itertools::Itertools;

/// Returns a string containing the input char, or an ASCII representation of it
/// if it's a control character.
pub fn repr(c: char) -> String {
    if let Ok(ascii_char) = AsciiChar::from_ascii(c) {
        if let Some(caret_escape) = ascii::caret_encode(ascii_char) {
            return format!("^{}", caret_escape.as_char());
        }
    }
    c.to_string()
}

/// Returns a string in U+XXXX format representing the code point for the given Unicode character.
pub fn codepoint(c: char) -> String {
    format!("U+{:0>4X}", c as u32)
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
        .flat_map(|i| i.to_le_bytes().to_vec())
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
        .flat_map(|i| i.to_be_bytes().to_vec())
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

    name(c)
}

pub struct CharacterInfo {
    pub repr: String,
    pub codepoint: String,
    pub bytes: String,
    pub name: String,
    pub block: String,
    pub category: String,
}

impl CharacterInfo {
    pub fn from_char(c: char) -> CharacterInfo {
        let repr = repr(c);
        let codepoint = codepoint(c);
        let bytes = char_to_bytes_utf8(c);
        let name = name_or_alias(c);
        let block = ucd::Block::of(c)
            .map(|block| block.name)
            .unwrap_or("")
            .to_string();
        let category = ucd::GeneralCategory::of(c).human_name().to_string();

        CharacterInfo {
            repr,
            codepoint,
            bytes,
            name,
            block,
            category,
        }
    }

    pub fn into_record(self, ascii_only: bool) -> Vec<String> {
        let mut record = vec![
            self.codepoint,
            self.bytes,
            self.name,
            self.block,
            self.category,
        ];

        if !ascii_only {
            record.insert(0, self.repr);
        }

        record
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repr() {
        let cases: Vec<(char, &str)> = vec![
            ('a', "a"),
            ('2', "2"),
            ('!', "!"),
            (' ', " "),
            ('μ', "μ"),
            ('チ', "チ"),
            ('\t', "^I"),   // tab
            ('\n', "^J"),   // line feed
            ('\x08', "^H"), // backspace
            ('\x1b', "^["), // escape
            ('\0', "^@"),   // null
        ];

        for (input, expected) in cases {
            let actual = repr(input);
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_codepoint() {
        let cases: Vec<(char, &str)> = vec![
            ('a', "U+0061"),
            ('2', "U+0032"),
            ('!', "U+0021"),
            (' ', "U+0020"),
            ('μ', "U+03BC"),
            ('チ', "U+30C1"),
            ('\t', "U+0009"),   // tab
            ('\n', "U+000A"),   // line feed
            ('\x08', "U+0008"), // backspace
            ('\x1b', "U+001B"), // escape
            ('\0', "U+0000"),   // null
            ('�', "U+FFFD"),    // replacement character
            ('𠃅', "U+200C5"),  // from CJK Extension B
            // private use areas
            (char::from_u32(0xE000).unwrap(), "U+E000"), // Private Use Area (BMP)
            (char::from_u32(0xF0000).unwrap(), "U+F0000"), // Supplementary Private Use Area-A
            (char::from_u32(0x100000).unwrap(), "U+100000"), // Supplementy Private Use Area-B
        ];

        for (input, expected) in cases {
            let actual = codepoint(input);
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_char_to_bytes_utf8() {
        let cases: Vec<(char, &str)> = vec![
            ('a', "61"),
            ('μ', "ce bc"),
            ('チ', "e3 83 81"),
            ('�', "ef bf bd"),
            ('𠃅', "f0 a0 83 85"),
        ];

        for (input, expected) in cases {
            let actual = char_to_bytes_utf8(input);
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_char_to_bytes_utf16le() {
        let cases: Vec<(char, &str)> = vec![
            ('a', "61 00"),
            ('μ', "bc 03"),
            ('チ', "c1 30"),
            ('�', "fd ff"),
            ('𠃅', "40 d8 c5 dc"),
        ];

        for (input, expected) in cases {
            let actual = char_to_bytes_utf16le(input);
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_char_to_bytes_utf16be() {
        let cases: Vec<(char, &str)> = vec![
            ('a', "00 61"),
            ('μ', "03 bc"),
            ('チ', "30 c1"),
            ('�', "ff fd"),        // replacement character
            ('𠃅', "d8 40 dc c5"), // from CJK Extension B
        ];

        for (input, expected) in cases {
            let actual = char_to_bytes_utf16be(input);
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_name_or_alias() {
        let cases: Vec<(char, &str)> = vec![
            ('a', "LATIN SMALL LETTER A"),
            ('2', "DIGIT TWO"),
            ('!', "EXCLAMATION MARK"),
            (' ', "SPACE"),
            ('μ', "GREEK SMALL LETTER MU"),
            ('チ', "KATAKANA LETTER TI"),
            ('\t', "<CHARACTER TABULATION>"),
            ('\n', "<LINE FEED>"),
            ('\x08', "<BACKSPACE>"),
            ('\x1b', "<ESCAPE>"),
            ('\0', "<NULL>"),
            ('\u{0E9D}', "* LAO LETTER FO FON"), // correction of original name
            ('\u{A015}', "* YI SYLLABLE ITERATION MARK"), // correction of original name
            // ('ꩮ', "* MYANMAR LETTER KHAMTI LLA"),         // correction of original name (added in Unicode 14, so unsupported by ucd crate)
            ('�', "REPLACEMENT CHARACTER"),
            ('𠃅', "CJK UNIFIED IDEOGRAPH-200C5"),
            // private use areas
            (char::from_u32(0xE000).unwrap(), ""), // Private Use Area (BMP)
            (char::from_u32(0xF0000).unwrap(), ""), // Supplementary Private Use Area-A
            (char::from_u32(0x100000).unwrap(), ""), // Supplementy Private Use Area-B
        ];

        for (input, expected) in cases {
            let actual = name_or_alias(input);
            assert_eq!(expected, actual);
        }
    }
}
