use clap::{App, Arg, crate_description, crate_name, crate_version};
use rusqlite::{Connection, NO_PARAMS};

const DEFAULT_DB_NAME: &str = "favlist.sqlite";

fn main() {

    let matches = App::new(crate_name!())
        .version(crate_version!())
        // .about(crate_description!())
        .arg(Arg::with_name("database")
            .long("db")
            .value_name("FILE")
            .help("The file to save/load your list(s)")
            .takes_value(true)
            .default_value(DEFAULT_DB_NAME))
        .get_matches();

    let db = matches.value_of("database").unwrap();

    let conn = Connection::open(db).unwrap();
}
