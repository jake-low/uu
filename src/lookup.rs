use clap::{self, App, Arg, ArgMatches};
use std::io::{self, Write};
use tabwriter::TabWriter;

use unic::char::property::EnumeratedCharProperty;
use unic::ucd; //::{self, BidiClass};

use crate::utils;

pub fn cmd() -> App<'static> {
    return App::new("lookup")
        .about("Show details about a single Unicode code point")
        .arg(
            Arg::new("glyph")
                .index(1)
                .help("Either a single UTF-8 glyph, or a string in U+XXXX format"),
        );
}

pub fn run(matches: &ArgMatches) {
    let glyph = matches.value_of("glyph").unwrap();
    let c: char;

    if glyph.starts_with("U+") {
        c = char::from_u32(u32::from_str_radix(&glyph[2..], 16).ok().unwrap()).unwrap();
    } else {
        c = glyph.chars().nth(0).unwrap();
    }

    let mut tw = TabWriter::new(io::stdout());

    write!(&mut tw, "Glyph:\t{}\n", utils::repr(c)).unwrap();
    write!(&mut tw, "Code point:\t{}\n", utils::codepoint(c)).unwrap();
    write!(&mut tw, "Name:\t{}\n", utils::name_or_alias(c)).unwrap();
    write!(&mut tw, "Block:\t{}\n", ucd::Block::of(c).unwrap().name).unwrap();

    let category = ucd::GeneralCategory::of(c);
    write!(
        &mut tw,
        "Category:\t{} ({})\n",
        category.human_name(),
        category.abbr_name()
    )
    .unwrap();

    let bidi_class = ucd::BidiClass::of(c);
    write!(
        &mut tw,
        "Bidirectional Class:\t{} ({})\n",
        bidi_class.human_name(),
        bidi_class.abbr_name()
    )
    .unwrap();

    /*
    let combi_class = ucd::CanonicalCombiningClass::of(c);
    write!(&mut tw, "Combining Class:\t{}\n", combi_class).unwrap();
    */

    write!(
        &mut tw,
        "Added in version:\t{}\n",
        ucd::Age::of(c).unwrap().actual()
    )
    .unwrap();

    write!(&mut tw, "UTF-8:\t{}\n", utils::char_to_bytes_utf8(c)).unwrap();
    write!(&mut tw, "UTF-16BE:\t{}\n", utils::char_to_bytes_utf16be(c)).unwrap();
    write!(&mut tw, "UTF-16LE:\t{}\n", utils::char_to_bytes_utf16le(c)).unwrap();
    write!(&mut tw, "UTF-32BE:\t{}\n", utils::char_to_bytes_utf32be(c)).unwrap();
    write!(&mut tw, "UTF-32LE:\t{}\n", utils::char_to_bytes_utf32le(c)).unwrap();

    tw.flush().unwrap();
}
