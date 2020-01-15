use rusqlite::{Connection, named_params, NO_PARAMS};
use rusqlite::types::ValueRef;
use std::collections::HashMap;
use std::str::from_utf8;

fn main() {
    let matches = cli::app().get_matches();

    let db = matches.value_of("database").unwrap();

    let conn = Connection::open(db).unwrap();

    if let Some(matches) = matches.subcommand_matches("new") {
        let table_name = matches.value_of("list name").unwrap();
        let columns = matches.values_of("columns").unwrap();
        let table = table_data::Table::new(table_name, columns);
        conn.execute(&table.declaration(), NO_PARAMS).unwrap();
    } else if let Some(matches) = matches.subcommand_matches("rem") {
        let table_name = matches.value_of("list name").unwrap();
        conn.execute(
            &format!("DROP TABLE {table_name}", table_name = table_name),
            NO_PARAMS,
        ).unwrap();
    } else if let Some(matches) = matches.subcommand_matches("add") {
        let (column_names, column_data) = if let Some(columns) = matches.values_of("columns") {
            column_partitioner(columns)
        } else {
            (Vec::new(), Vec::new())
        };
        let column_params: Vec<_> = (1..=column_data.len()).map(|n| format!("?{}", n)).collect();
        let script = format!(
            "INSERT INTO {table_name} ({column_names}) VALUES ({column_params})",
            table_name = matches.value_of("list name").unwrap(),
            column_names = column_names.join(", "),
            column_params = column_params.join(", "),
        );
        conn.execute(&script, column_data).unwrap();
    } else if let Some(matches) = matches.subcommand_matches("list") {
        let table_name = matches.value_of("list name").unwrap();
        let script = format!(
            "SELECT * FROM {table_name}",
            table_name = table_name,
        );

        let mut stmt = conn.prepare(&script).unwrap();
        let column_count = stmt.column_count();
        let header: Vec<_> = stmt.column_names().iter().map(|c| String::from(*c)).collect();
        let mut rows = stmt.query(NO_PARAMS).unwrap();
        // For ease of use to convert to JSON, as array of objects
        let mut table_representation: Vec<HashMap<&str, String>> = Vec::new();
        while let Ok(Some(sql_row)) = rows.next() {
            let mut hashmap = HashMap::new();
            for i in 0..column_count {
                let value_ref = sql_row.get_raw(i);
                let cell = match value_ref {
                    ValueRef::Null => "<NULL>".to_string(),
                    ValueRef::Integer(i) => i.to_string(),
                    ValueRef::Real(r) => r.to_string(),
                    ValueRef::Text(utf8) => from_utf8(utf8).unwrap().to_string(),
                    ValueRef::Blob(utf8) => from_utf8(utf8).unwrap().to_string(),
                };
                hashmap.insert(header[i].as_str(), cell);
            }
            table_representation.push(hashmap);
        }
        println!("{:?}", table_representation);
    }
}

/// Takes values from a `clap` argument representing columns and their data, and
/// separates them from the format `[name, value, name, value]` into a `Vec` of
/// names and a `Vec` of values
fn column_partitioner<'a, I>(clap_values: I) -> (Vec<&'a str>, Vec<&'a str>)
    where I: Iterator<Item = &'a str>
    {
    let (column_names, column_values): (Vec<_>, Vec<_>) = clap_values
        .enumerate()
        .partition(|(i, _v)| i % 2 == 0);

    (
        column_names.iter().map(|(_i, v)| v.to_owned()).collect(),
        column_values.iter().map(|(_i, v)| v.to_owned()).collect(),
    )
}

mod cli;
mod table_data;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn column_partitioner_test() {
        let values = vec![
            "Works",
            "Hopefully",
            "Test Passed",
            "Yes",
        ].into_iter();
        let (column_names, column_data) = column_partitioner(values);
        assert_eq!(vec!["Works", "Test Passed"], column_names);
        assert_eq!(vec!["Hopefully", "Yes"], column_data);
    }
}
