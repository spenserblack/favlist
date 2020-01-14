use rusqlite::{Connection, named_params, NO_PARAMS};

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
            &format!(include_str!("../resources/favlist-rem.sql"), table_name = table_name),
            NO_PARAMS,
        ).unwrap();
    } else if let Some(matches) = matches.subcommand_matches("add") {
        println!("Table name: {:?}", matches.value_of("list name"));
        let columns = if let Some(columns) = matches.values_of("columns") {
            let (column_names, column_data): (Vec<_>, Vec<_>) = columns
                .enumerate()
                .partition(|(i, _v)| { i % 2 == 0});

            column_names
                .iter()
                .map(|(_i, v)| v.to_owned())
                .zip(column_data
                    .iter()
                    .map(|(_i, v)| v.to_owned()))
                .collect()
        } else {
            Vec::new()
        };
        println!("Column data: {:?}", columns);
    }
}

mod cli;
mod table_data;
