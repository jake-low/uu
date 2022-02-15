use std::io::{self, BufRead, Write};

use unic::char::property::EnumeratedCharProperty;
use unic::ucd;

use clap::{self, App, Arg, ArgMatches};
use itertools::Itertools;
use tabwriter::TabWriter;

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

/// Returns a string in U+XXXX format representing the code point for the given Unicode character.
fn codepoint(c: char) -> String {
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

    for line in stdin.lock().lines() {
        for c in line.unwrap().chars() {
            let codepoint = codepoint(c);
            let bytes = char_to_bytes_utf8(c);
            let name = name_or_corrective_alias(c);
            let block = format!("{}", ucd::Block::of(c).unwrap().name);
            let category = ucd::GeneralCategory::of(c).human_name();

            if !matches.is_present("ascii") {
                // only print the glyphs column if we're not in ASCII-only mode
                write!(&mut tw, "{}\t", c).unwrap();
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
