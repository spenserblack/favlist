use indexmap::IndexMap;
use prettytable::{Cell, Row, Table};
use rusqlite::types::ValueRef;
use rusqlite::Rows;
use std::str::from_utf8;

/// `header` is redundant data, but useful in case of empty table
pub fn prettytable(rows: &mut Rows) -> String {
    let mut prettytable = Table::new();
    // let header = Row::new(header.iter().map(|c| Cell::new(c)).collect());
    let header = Row::new(
        rows.column_names()
            .unwrap()
            .iter()
            .map(|c| Cell::new(c))
            .collect(),
    );
    let column_count = rows.column_count().unwrap();
    prettytable.add_row(header);
    while let Ok(Some(row)) = rows.next() {
        let row = (0..column_count)
            .map(|i| row.get_raw(i))
            .map(|v| {
                use ValueRef::*;
                match v {
                    Null => "<NULL>".to_string(),
                    Integer(i) => i.to_string(),
                    Real(r) => r.to_string(),
                    Text(utf8) => from_utf8(utf8).unwrap().to_string(),
                    Blob(utf8) => from_utf8(utf8).unwrap().to_string(),
                }
            })
            .map(|v| Cell::new(&v))
            .collect();
        let row = Row::new(row);
        prettytable.add_row(row);
    }
    prettytable.to_string()
}

pub fn json(rows: &mut Rows) -> String {
    use serde_json::Value;
    let header: Vec<_> = rows
        .column_names()
        .unwrap()
        .iter()
        .map(|c| String::from(*c))
        .collect();
    let column_count = rows.column_count().unwrap();
    let mut table: Vec<IndexMap<_, _>> = Vec::new();
    while let Ok(Some(row)) = rows.next() {
        let row = (0..column_count)
            .map(|i| (header[i].as_str(), row.get_raw(i)))
            .map(|(k, v)| {
                let v = match v {
                    ValueRef::Null => Value::Null,
                    ValueRef::Integer(i) => Value::from(i),
                    ValueRef::Real(r) => Value::from(r),
                    ValueRef::Text(utf8) => Value::from(from_utf8(utf8).unwrap()),
                    ValueRef::Blob(utf8) => Value::from(from_utf8(utf8).unwrap()),
                };
                (k, v)
            })
            .collect();
        table.push(row);
    }
    serde_json::to_string_pretty(&table).unwrap()
}

pub fn yaml(rows: &mut Rows) -> String {
    use serde_yaml::Value;
    let header: Vec<_> = rows
        .column_names()
        .unwrap()
        .iter()
        .map(|c| String::from(*c))
        .collect();
    let column_count = rows.column_count().unwrap();
    let mut table: Vec<IndexMap<_, _>> = Vec::new();
    while let Ok(Some(row)) = rows.next() {
        let row = (0..column_count)
            .map(|i| (header[i].as_str(), row.get_raw(i)))
            .map(|(k, v)| {
                let v = match v {
                    ValueRef::Null => Value::Null,
                    ValueRef::Integer(i) => Value::from(i),
                    ValueRef::Real(r) => Value::from(r),
                    ValueRef::Text(utf8) => Value::from(from_utf8(utf8).unwrap()),
                    ValueRef::Blob(utf8) => Value::from(from_utf8(utf8).unwrap()),
                };
                (k, v)
            })
            .collect();
        table.push(row);
    }
    serde_yaml::to_string(&table).unwrap()
}
