use std::io::{self, Write};

use unic::char::property::EnumeratedCharProperty;
use unic::ucd;

use utf8;

use clap::{self, App, Arg, ArgMatches};
use tabwriter::TabWriter;

use crate::utils;

pub fn cmd() -> App<'static> {
    return App::new("list")
        .about("Print a table of all Unicode code points (useful for grepping)")
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

fn block(c: char) -> String {
    match ucd::Block::of(c) {
        Some(block) => block.name.to_string(),
        None => String::new(),
    }
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

    let mut previous_block = "Basic Latin"; // HACK first block
    let mut lines_since_flush = 0;

    // FIXME currently only prints BMP; should be configurable with flags
    for u in (0 as u32)..=(0xFFFF as u32) {
        let c = char::from_u32(u);

        if c == None {
            continue;
        }
        let c = c.unwrap();

        // TODO most of this logic is duplicated with 'inspect'
        let codepoint = utils::codepoint(c);
        let bytes = utils::char_to_bytes_utf8(c);
        let name = utils::name_or_alias(c);
        let block = ucd::Block::of(c).map(|b| b.name).unwrap_or("");
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

        lines_since_flush += 1;

        if block != previous_block || lines_since_flush > 4096 {
            tw.flush().unwrap();
            previous_block = block;
            lines_since_flush = 0;
        }
    }
}
