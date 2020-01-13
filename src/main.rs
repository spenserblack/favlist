use rusqlite::{Connection, NO_PARAMS};

const DEFAULT_DB_NAME: &str = "favlist.sqlite";

fn main() {
    let matches = cli::app().get_matches();

    let db = matches.value_of("database").unwrap();

    let conn = Connection::open(db).unwrap();
}

mod cli;
mod table_data;
