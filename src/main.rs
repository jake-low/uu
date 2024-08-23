use std::io;
use std::process;

use clap::{Parser, Subcommand};

mod errors;
mod inspect;
mod list;
mod lookup;
mod utils;

use errors::CliError;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    #[command(subcommand)]
    subcommand: Command,
}

#[derive(Subcommand)]
enum Command {
    Inspect(inspect::CliArgs),
    List(list::CliArgs),
    Lookup(lookup::CliArgs),
}

fn main() {
    let matches = CliArgs::parse();

    let result = match matches.subcommand {
        Command::Inspect(args) => inspect::run(&args),
        Command::List(args) => list::run(&args),
        Command::Lookup(args) => lookup::run(&args),
    };

    match result {
        Ok(()) => process::exit(0),
        Err(CliError::Arg(err)) => err.exit(),
        Err(CliError::Csv(err)) => {
            eprintln!("{}", err);
            process::exit(1);
        }
        Err(CliError::Io(ref err)) if err.kind() == io::ErrorKind::BrokenPipe => {
            process::exit(0);
        }
        Err(CliError::Io(err)) => {
            eprintln!("{}", err);
            process::exit(1);
        }
        Err(CliError::Other(msg)) => {
            eprintln!("{}", msg);
            process::exit(1);
        }
    }
}
