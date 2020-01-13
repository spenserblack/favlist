use clap::{App, Arg, crate_description, crate_name, crate_version};
use super::DEFAULT_DB_NAME;

pub fn app<'a, 'b>() -> App<'a, 'b> {
    App::new(crate_name!())
    .version(crate_version!())
    // .about(crate_description!())
    .arg(Arg::with_name("database")
        .long("db")
        .value_name("FILE")
        .help("The file to save/load your list(s)")
        .takes_value(true)
        .default_value(DEFAULT_DB_NAME))
}
