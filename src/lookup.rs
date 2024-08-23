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
        args.glyph.chars().next().unwrap()
    };

    let mut tw = TabWriter::new(io::stdout());

    writeln!(&mut tw, "Glyph:\t{}", utils::repr(c))?;
    writeln!(&mut tw, "Code point:\t{}", utils::codepoint(c))?;
    writeln!(&mut tw, "Name:\t{}", utils::name_or_alias(c))?;
    writeln!(&mut tw, "Block:\t{}", ucd::Block::of(c).unwrap().name)?;

    let category = ucd::GeneralCategory::of(c);
    writeln!(
        &mut tw,
        "Category:\t{} ({})",
        category.human_name(),
        category.abbr_name()
    )?;

    let bidi_class = ucd::BidiClass::of(c);
    writeln!(
        &mut tw,
        "Bidirectional Class:\t{} ({})",
        bidi_class.human_name(),
        bidi_class.abbr_name()
    )?;

    /*
    let combi_class = ucd::CanonicalCombiningClass::of(c);
    write!(&mut tw, "Combining Class:\t{}\n", combi_class).unwrap();
    */

    writeln!(
        &mut tw,
        "Added in version:\t{}",
        ucd::Age::of(c).unwrap().actual()
    )?;

    writeln!(&mut tw, "UTF-8:\t{}", utils::char_to_bytes_utf8(c))?;
    writeln!(&mut tw, "UTF-16BE:\t{}", utils::char_to_bytes_utf16be(c))?;
    writeln!(&mut tw, "UTF-16LE:\t{}", utils::char_to_bytes_utf16le(c))?;
    writeln!(&mut tw, "UTF-32BE:\t{}", utils::char_to_bytes_utf32be(c))?;
    writeln!(&mut tw, "UTF-32LE:\t{}", utils::char_to_bytes_utf32le(c))?;

    tw.flush()?;

    Ok(())
}
