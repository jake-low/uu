use clap::{self, App, Arg, ArgMatches};
use std::io::{self, Write};
use tabwriter::TabWriter;

use unic::char::property::EnumeratedCharProperty;
use unic::ucd;

use crate::utils;

pub fn cmd() -> App<'static> {
    return App::new("lookup")
        .about("show details about a single Unicode code point")
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
    write!(
        &mut tw,
        "Category:\t{}\n",
        ucd::GeneralCategory::of(c).human_name()
    )
    .unwrap();

    tw.flush().unwrap();
}
