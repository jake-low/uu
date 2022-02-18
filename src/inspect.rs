use std::io::{self, Write};

use unic::char::property::EnumeratedCharProperty;
use unic::ucd;

use utf8;

use clap::{self, App, Arg, ArgMatches};
use tabwriter::TabWriter;

use crate::utils;

pub fn cmd() -> App<'static> {
    return App::new("inspect")
        .about("Reads UTF-8 from stdin and prints info about each code point")
        .arg(
            Arg::new("no-header")
                .short('H')
                .long("no-header")
                .takes_value(false)
                .help("Don't print a header row"),
        )
        .arg(
            Arg::new("ascii")
                .short('a')
                .long("ascii")
                .takes_value(false)
                .help("Restrict output to ASCII"),
        );
}

pub fn run(matches: &ArgMatches) {
    let stdin = io::stdin();

    let mut tw = TabWriter::new(io::stdout());

    if !matches.is_present("no-header") {
        if !matches.is_present("ascii") {
            // only print the glyphs column if we're not in ASCII-only mode
            write!(&mut tw, "GLYPH\t").unwrap();
        }

        write!(
            &mut tw,
            "{}\t{}\t{}\t{}\t{}\n",
            "CODE POINT", "UTF-8 BYTES", "NAME", "BLOCK", "CATEGORY"
        )
        .unwrap();
    }

    let mut decoder = utf8::BufReadDecoder::new(stdin.lock());

    while let Some(result) = decoder.next_strict() {
        let chunk = result.ok().unwrap();
        for c in chunk.chars() {
            let codepoint = utils::codepoint(c);
            let bytes = utils::char_to_bytes_utf8(c);
            let name = utils::name_or_alias(c);
            let block = format!("{}", ucd::Block::of(c).unwrap().name);
            let category = ucd::GeneralCategory::of(c).human_name();

            if !matches.is_present("ascii") {
                // only print the glyphs column if we're not in ASCII-only mode
                write!(&mut tw, "{}\t", utils::repr(c)).unwrap();
            }

            write!(
                &mut tw,
                "{}\t{}\t{}\t{}\t{}\n",
                codepoint, bytes, name, block, category
            )
            .unwrap();
        }

        tw.flush().unwrap();
    }
}
