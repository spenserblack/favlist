use indexmap::IndexMap;
use rusqlite::{Connection, params, NO_PARAMS};
use rusqlite::types::ValueRef;
use std::io::{self, Write};
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
    } else if let Some(matches) = matches.subcommand_matches("sub") {
        let table_name = matches.value_of("list name").unwrap();

        let id: String;
        let (filter_names, filter_values) = if let Some(row_id) = matches.value_of("row ID") {
            (vec!["id"], vec![row_id])
        } else if let Some(filters) = matches.values_of("filters") {
            column_partitioner(filters)
        } else {
            id = {
                let mut stmt = conn.prepare(&query_builder::List::new(table_name, None).to_string()).unwrap();
                let pretty_table = printer::prettytable(&mut stmt.query(NO_PARAMS).unwrap());
                println!("{}", pretty_table);
                print!("Enter the ID of the row to remove> ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                input
            };
            (vec!["id"], vec![id.as_str()])
        };

        let script = query_builder::Sub::new(table_name, filter_names).to_string();
        conn.execute(&script, filter_values).unwrap();
    } else if let Some(matches) = matches.subcommand_matches("list") {
        let table_name = matches.value_of("list name").unwrap();
        let mut stmt;
        let mut rows = if let Some(filters) = matches.values_of("filters") {
            let (filter_names, filter_values) = column_partitioner(filters);
            let script = query_builder::List::new(table_name, Some(filter_names)).to_string();
            stmt = conn.prepare(&script).unwrap();
            stmt.query(filter_values).unwrap()
        } else {
            let script = query_builder::List::new(table_name, None).to_string();
            stmt = conn.prepare(&script).unwrap();
            stmt.query(NO_PARAMS).unwrap()
        };

        // For ease of use to convert to JSON, as array of objects
        let out = if matches.is_present("json") {
            printer::json(&mut rows)
        } else if matches.is_present("yaml") {
            printer::yaml(&mut rows)
        } else {
            printer::prettytable(&mut rows)
        };
        println!("{}", out);
    } else {
        unimplemented!("Launch a UI when no subcommand supplied");
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
mod printer;
mod query_builder;
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
