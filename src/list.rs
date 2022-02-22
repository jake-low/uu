use std::io;

use clap::{self, App, Arg, ArgMatches};
use tabwriter::TabWriter;

use crate::errors::{CliError, CliResult};

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

pub fn run(matches: &ArgMatches) -> CliResult<()> {
    let tw = TabWriter::new(io::stdout());
    let mut wtr = csv::WriterBuilder::new().delimiter(b'\t').from_writer(tw);

    let ascii_only = matches.is_present("ascii");

    if !matches.is_present("no-header") {
        if !ascii_only {
            // only print the glyphs column if we're not in ASCII-only mode
            wtr.write_field("GLYPH")?;
        }
        wtr.write_record(["CODE POINT", "UTF-8 BYTES", "NAME", "BLOCK", "CATEGORY"])?;
    }

    let mut previous_block: Option<String> = None;
    let mut lines_since_flush = 0;

    let start = matches.value_of("start").unwrap();
    if !start.starts_with("U+") {
        return Err(CliError::Other(format!(
            "Failed to parse start code point: {}",
            start
        )));
    }

    let start = match u32::from_str_radix(&start[2..], 16) {
        Ok(value) => value,
        Err(_) => {
            return Err(CliError::Other(format!(
                "Failed to parse start code point: {}",
                start
            )));
        }
    };

    let end = matches.value_of("end").unwrap();
    if !end.starts_with("U+") {
        return Err(CliError::Other(format!(
            "Failed to parse end code point: {}",
            start
        )));
    }

    let end = match u32::from_str_radix(&end[2..], 16) {
        Ok(value) => value,
        Err(_) => {
            return Err(CliError::Other(format!(
                "Failed to parse end code point: {}",
                start
            )));
        }
    };

    for u in start..=end {
        match char::from_u32(u) {
            None => continue,
            Some(c) => {
                let codeinfo = CharacterInfo::from_char(c);
                let should_flush = match &previous_block {
                    Some(prev) => codeinfo.block != prev.clone() || lines_since_flush >= 4096,
                    None => false,
                };

                previous_block = Some(codeinfo.block.clone());

                wtr.write_record(codeinfo.to_record(ascii_only))?;

                lines_since_flush += 1;

                if should_flush {
                    wtr.flush()?;
                    lines_since_flush = 0;
                }
            }
        };
    }

    wtr.flush()?;

    Ok(())
}
