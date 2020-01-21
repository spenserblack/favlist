use clap::{crate_description, crate_name, crate_version, App, Arg, SubCommand};

const DEFAULT_DB_NAME: &str = "favlist.sqlite";
const COLUMN_DEFINITION_HELP: &str = "\
DEFINING A COLUMN:
- A basic column: `ColumnName`
- An important column: `~ColumnName`
- An integer column: `Column@int
- A text column: `Column@text
- A real column: `Column@real
- A numeric column: `Column@num";

pub fn app<'a, 'b>() -> App<'a, 'b> {
    // Subcommand definitions: {{{
    let subcommand_new = SubCommand::with_name("new")
        .about("Creates a new list")
        .arg(
            Arg::with_name("list name")
                .help("The name of the list to be created")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("columns")
                .help("Column definition(s)")
                .required(true)
                .multiple(true),
        )
        .after_help(COLUMN_DEFINITION_HELP);
    let subcommand_rem = SubCommand::with_name("rem").about("Removes a list").arg(
        Arg::with_name("list name")
            .help("The name of the list to be deleted")
            .required(true),
    );
    let subcommand_add = SubCommand::with_name("add")
        .about("Adds data to a list")
        .arg(
            Arg::with_name("list name")
                .help("The list to be modified")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("columns")
                .help("Column and data to be inserted in that column")
                .short("c")
                .long("column")
                .required(true)
                .number_of_values(2)
                .value_names(&["column", "data"])
                .multiple(true),
        );
    let subcommand_sub = SubCommand::with_name("sub")
        .about("Removes an entry from list")
        .arg(
            Arg::with_name("list name")
                .help("The list to be modified")
                .required(true)
                .index(1),
        )
        .arg(Arg::with_name("row ID").help("The row to be removed"))
        .arg(
            Arg::with_name("filters")
                .help("Column and data to be filtered on")
                .short("f")
                .long("filter")
                .number_of_values(2)
                .value_names(&["column", "value"])
                .multiple(true),
        );
    let subcommand_edit = SubCommand::with_name("edit")
        .about("Edits a row in a list")
        .arg(
            Arg::with_name("list name")
                .help("The list to be modified")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("row ID")
                .help("The row to be modified")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("columns")
                .help("Column and data to be inserted in that column")
                .short("c")
                .long("column")
                .required(true)
                .number_of_values(2)
                .value_names(&["column", "data"])
                .multiple(true),
        );
    let subcommand_list = SubCommand::with_name("list")
        .about("Displays the data in a list")
        .arg(
            Arg::with_name("list name")
                .help("The list to be displayed")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("filters")
                .help("Column and data to be filtered on")
                .short("f")
                .long("filter")
                .number_of_values(2)
                .value_names(&["column", "value"])
                .multiple(true),
        )
        .arg(
            Arg::with_name("yaml")
                .help("Print result in YAML format")
                .long("yaml"),
        )
        .arg(
            Arg::with_name("json")
                .help("Print result in JSON format")
                .long("json")
                .conflicts_with("yaml"),
        );
    // }}}

    let app = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("database")
                .long("db")
                .value_name("FILE")
                .help("The file to save/load your list(s)")
                .takes_value(true)
                .default_value(DEFAULT_DB_NAME),
        )
        .subcommand(subcommand_new)
        .subcommand(subcommand_rem)
        .subcommand(subcommand_add)
        .subcommand(subcommand_sub)
        .subcommand(subcommand_edit)
        .subcommand(subcommand_list);

    app
}
