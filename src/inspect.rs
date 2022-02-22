use std::io;

use utf8;

use clap::{self, App, Arg, ArgMatches};
use tabwriter::TabWriter;

use crate::errors::{CliError, CliResult};
use crate::utils::CharacterInfo;

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

pub fn run(matches: &ArgMatches) -> CliResult<()> {
    let stdin = io::stdin();
    let mut decoder = utf8::BufReadDecoder::new(stdin.lock());

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

    while let Some(result) = decoder.next_strict() {
        match result {
            Ok(chunk) => {
                for c in chunk.chars() {
                    let codeinfo = CharacterInfo::from_char(c);
                    wtr.write_record(codeinfo.to_record(ascii_only))?;
                }

                wtr.flush()?;
            }
            Err(_) => {
                return Err(CliError::Other(
                    "Failed to decode input as UTF-8".to_string(),
                ));
            }
        }
    }

    Ok(())
}
