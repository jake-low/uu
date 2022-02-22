use std::io;
use std::process;

use clap::{self, App, AppSettings};

mod errors;
mod inspect;
mod list;
mod lookup;
mod utils;

use errors::CliError;

fn app() -> App<'static> {
    return App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(inspect::cmd())
        .subcommand(list::cmd())
        .subcommand(lookup::cmd());
}

fn main() {
    let matches = app().get_matches();

    let result = match matches.subcommand() {
        Some(("inspect", m)) => inspect::run(m),
        Some(("list", m)) => list::run(m),
        Some(("lookup", m)) => lookup::run(m),
        _ => unreachable!(),
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
