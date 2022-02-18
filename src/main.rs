use clap::{self, App, AppSettings};

mod inspect;
mod lookup;
mod utils;

fn app() -> App<'static> {
    return App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(inspect::cmd())
        .subcommand(lookup::cmd());
}

fn main() {
    let matches = app().get_matches();

    match matches.subcommand() {
        Some(("inspect", m)) => inspect::run(m),
        Some(("lookup", m)) => lookup::run(m),
        _ => unreachable!(),
    }
}
