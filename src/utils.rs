use unic::char::property::EnumeratedCharProperty;
use unic::ucd;

use itertools::Itertools;

/// Returns a string in U+XXXX format representing the code point for the given Unicode character.
pub fn codepoint(c: char) -> String {
    return format!("U+{:0>4x}", c as u32);
}

/// Returns a string in hexadecimal (with spaces) representing the bytes that encode this Unicode character in UTF-8.
fn char_to_bytes_utf8(c: char) -> String {
    let mut b = [0; 4];
    let bytes = c.encode_utf8(&mut b).as_bytes();

    format!("{:x}", bytes.iter().format(" "))
}

fn name(c: char) -> String {
    ucd::Name::of(c)
        .map(|name| name.to_string())
        .unwrap_or_default()
}

fn name_or_corrective_alias(c: char) -> String {
    match ucd::name_aliases_of(c, ucd::NameAliasType::NameCorrections)
        .unwrap_or_default()
        .first()
    {
        Some(v) => format!("* {}", v),
        None => name(c),
    }
}
