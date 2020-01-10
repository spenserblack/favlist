use rusqlite::{Connection, NO_PARAMS};

const DEFAULT_DB_NAME: &str = "favlist.sqlite";

fn main() {
    let conn = Connection::open(DEFAULT_DB_NAME).unwrap();
}
