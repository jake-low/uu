use std::io;

use clap::Parser;
use tabwriter::TabWriter;
use utf8;

use crate::errors::{CliError, CliResult};
use crate::utils::CharacterInfo;

/// Reads UTF-8 from stdin and prints info about each code point
#[derive(Parser)]
pub struct CliArgs {
    /// Don't print a header row
    #[arg(short = 'H', long)]
    no_header: bool,

    /// Restrict output to ASCII
    #[arg(short = 'a', long)]
    ascii: bool,
}

pub fn run(args: &CliArgs) -> CliResult<()> {
    let stdin = io::stdin();
    let mut decoder = utf8::BufReadDecoder::new(stdin.lock());

    let tw = TabWriter::new(io::stdout());
    let mut wtr = csv::WriterBuilder::new().delimiter(b'\t').from_writer(tw);

    if !args.no_header {
        if !args.ascii {
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
                    wtr.write_record(codeinfo.to_record(args.ascii))?;
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
