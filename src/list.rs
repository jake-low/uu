use std::io;

use clap::Parser;
use tabwriter::TabWriter;

use crate::errors::{CliError, CliResult};
use crate::utils::CharacterInfo;

/// Print a table of all Unicode code points (useful for grepping)
#[derive(Parser)]
pub struct CliArgs {
    /// Don't print a header row
    #[arg(short = 'H', long)]
    no_header: bool,

    /// Restrict output to ASCII
    #[arg(short = 'a', long)]
    ascii: bool,

    /// Unicode code point to begin at, in U+XXXX format
    start: String,
    /// Unicode code point to stop at, in U+XXXX format
    end: String,
}

pub fn run(args: &CliArgs) -> CliResult<()> {
    let tw = TabWriter::new(io::stdout());
    let mut wtr = csv::WriterBuilder::new().delimiter(b'\t').from_writer(tw);

    if !args.no_header {
        if !args.ascii {
            // only print the glyphs column if we're not in ASCII-only mode
            wtr.write_field("GLYPH")?;
        }
        wtr.write_record(["CODE POINT", "UTF-8 BYTES", "NAME", "BLOCK", "CATEGORY"])?;
    }

    let mut previous_block: Option<String> = None;
    let mut lines_since_flush = 0;

    if !args.start.starts_with("U+") {
        return Err(CliError::Other(format!(
            "Failed to parse start code point: {}",
            args.start
        )));
    }

    let start = match u32::from_str_radix(&args.start[2..], 16) {
        Ok(value) => value,
        Err(_) => {
            return Err(CliError::Other(format!(
                "Failed to parse start code point: {}",
                args.start
            )));
        }
    };

    if !args.end.starts_with("U+") {
        return Err(CliError::Other(format!(
            "Failed to parse end code point: {}",
            args.end
        )));
    }

    let end = match u32::from_str_radix(&args.end[2..], 16) {
        Ok(value) => value,
        Err(_) => {
            return Err(CliError::Other(format!(
                "Failed to parse end code point: {}",
                args.end
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

                wtr.write_record(codeinfo.into_record(args.ascii))?;

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
