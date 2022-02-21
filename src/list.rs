use std::io;
use std::process;

use clap::{self, App, Arg, ArgMatches};
use tabwriter::TabWriter;

use crate::utils::CharacterInfo;

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
        )
        .arg(
            Arg::new("start")
                .value_name("START")
                .default_value("U+0000")
                .index(1)
                .help("A Unicode code point, in U+XXXX format"),
        )
        .arg(
            Arg::new("end")
                .value_name("END")
                .default_value("U+FFFF")
                .index(2)
                .help("A Unicode code point, in U+XXXX format"),
        );
}

pub fn run(matches: &ArgMatches) {
    let tw = TabWriter::new(io::stdout());
    let mut wtr = csv::WriterBuilder::new().delimiter(b'\t').from_writer(tw);

    let ascii_only = matches.is_present("ascii");

    if !matches.is_present("no-header") {
        if !ascii_only {
            // only print the glyphs column if we're not in ASCII-only mode
            wtr.write_field("GLYPH").unwrap();
        }
        wtr.write_record(["CODE POINT", "UTF-8 BYTES", "NAME", "BLOCK", "CATEGORY"])
            .unwrap();
    }

    let mut previous_block: Option<String> = None;
    let mut lines_since_flush = 0;

    let start = matches.value_of("start").unwrap();
    if !start.starts_with("U+") {
        eprintln!("Failed to parse start code point: {}", start);
        process::exit(1);
    }

    let start = match u32::from_str_radix(&start[2..], 16) {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Failed to parse start code point: {}", start);
            process::exit(1);
        }
    };

    let end = matches.value_of("end").unwrap();
    if !end.starts_with("U+") {
        eprintln!("Failed to parse start code point: {}", end);
        process::exit(1);
    }

    let end = match u32::from_str_radix(&end[2..], 16) {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Failed to parse start code point: {}", end);
            process::exit(1);
        }
    };

    println!("{:x} {:x}", start, end);

    for u in start..=end {
        let c = char::from_u32(u);
        if c == None {
            continue;
        }
        let c = c.unwrap();

        let codeinfo = CharacterInfo::from_char(c);
        let should_flush = match &previous_block {
            Some(prev) => codeinfo.block != prev.clone() || lines_since_flush >= 4096,
            None => false,
        };

        // if should_flush {
        previous_block = Some(codeinfo.block.clone());
        // }

        wtr.write_record(codeinfo.to_record(ascii_only)).unwrap();

        lines_since_flush += 1;

        if should_flush {
            wtr.flush().unwrap();
            lines_since_flush = 0;
        }
    }

    wtr.flush().unwrap();
}
