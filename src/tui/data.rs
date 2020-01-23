use rusqlite::{Connection, NO_PARAMS};

pub fn available_tables(conn: &Connection) -> Vec<String> {
    const SCRIPT: &str = "SELECT * FROM sqlite_master WHERE type='table'";
    let mut stmt = conn.prepare(SCRIPT).unwrap();
    let mut rows = stmt.query(NO_PARAMS).unwrap();
    let mut table_names = Vec::new();
    while let Ok(Some(row)) = rows.next() {
        let table_name = row.get_unwrap("tbl_name");
        table_names.push(table_name);
    }
    table_names
}
