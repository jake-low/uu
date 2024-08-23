use std::io::{self, Write};

use clap::Parser;
use tabwriter::TabWriter;
use unic::char::property::EnumeratedCharProperty;
use unic::ucd; //::{self, BidiClass};

use crate::errors::CliResult;
use crate::utils;

/// Show details about a single Unicode code point
#[derive(Parser)]
pub struct CliArgs {
    /// Either a single UTF-8 glyph, or a string in U+XXXX format
    glyph: String,
}

pub fn run(args: &CliArgs) -> CliResult<()> {
    let c: char = if args.glyph.starts_with("U+") {
        char::from_u32(u32::from_str_radix(&args.glyph[2..], 16).ok().unwrap()).unwrap()
    } else {
        args.glyph.chars().nth(0).unwrap()
    };

    let mut tw = TabWriter::new(io::stdout());

    write!(&mut tw, "Glyph:\t{}\n", utils::repr(c))?;
    write!(&mut tw, "Code point:\t{}\n", utils::codepoint(c))?;
    write!(&mut tw, "Name:\t{}\n", utils::name_or_alias(c))?;
    write!(&mut tw, "Block:\t{}\n", ucd::Block::of(c).unwrap().name)?;

    let category = ucd::GeneralCategory::of(c);
    write!(
        &mut tw,
        "Category:\t{} ({})\n",
        category.human_name(),
        category.abbr_name()
    )?;

    let bidi_class = ucd::BidiClass::of(c);
    write!(
        &mut tw,
        "Bidirectional Class:\t{} ({})\n",
        bidi_class.human_name(),
        bidi_class.abbr_name()
    )?;

    /*
    let combi_class = ucd::CanonicalCombiningClass::of(c);
    write!(&mut tw, "Combining Class:\t{}\n", combi_class).unwrap();
    */

    write!(
        &mut tw,
        "Added in version:\t{}\n",
        ucd::Age::of(c).unwrap().actual()
    )?;

    write!(&mut tw, "UTF-8:\t{}\n", utils::char_to_bytes_utf8(c))?;
    write!(&mut tw, "UTF-16BE:\t{}\n", utils::char_to_bytes_utf16be(c))?;
    write!(&mut tw, "UTF-16LE:\t{}\n", utils::char_to_bytes_utf16le(c))?;
    write!(&mut tw, "UTF-32BE:\t{}\n", utils::char_to_bytes_utf32be(c))?;
    write!(&mut tw, "UTF-32LE:\t{}\n", utils::char_to_bytes_utf32le(c))?;

    tw.flush()?;

    Ok(())
}
