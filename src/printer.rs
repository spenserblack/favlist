use indexmap::IndexMap;
use prettytable::{Table, Row, Cell};

/// `header` is redundant data, but useful in case of empty table
pub fn prettytable(table: &Vec<IndexMap<&str, String>>, header: &Vec<String>) -> String {
    let mut prettytable = Table::new();
    let header = Row::new(header.iter().map(|c| Cell::new(c)).collect());
    prettytable.add_row(header);
    for row in table.iter() {
        let row = row.iter().map(|(_k, v)| Cell::new(v)).collect();
        let row = Row::new(row);
        prettytable.add_row(row);
    }
    prettytable.to_string()
}

pub fn json(table: &Vec<IndexMap<&str, String>>) -> String {
    serde_json::to_string_pretty(&table).unwrap()
}

pub fn yaml(table: &Vec<IndexMap<&str, String>>) -> String {
    serde_yaml::to_string(&table).unwrap()
}
