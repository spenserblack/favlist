use clap::{crate_description, crate_name, crate_version, App, Arg, SubCommand};

const DEFAULT_DB_NAME: &str = "favlist.sqlite";

pub fn app<'a, 'b>() -> App<'a, 'b> {
    App::new(crate_name!())
        .version(crate_version!())
        // .about(crate_description!())
        .arg(
            Arg::with_name("database")
                .long("db")
                .value_name("FILE")
                .help("The file to save/load your list(s)")
                .takes_value(true)
                .default_value(DEFAULT_DB_NAME),
        )
        .subcommand(SubCommand::with_name("new")
            .about("Creates a new list")
            .arg(Arg::with_name("list name")
                .help("The name of the list to be created")
                .required(true)
                .index(1))
            .arg(Arg::with_name("columns")
                .help("Column definition(s)")
                .required(true)
                .multiple(true)))
        .subcommand(SubCommand::with_name("rem")
            .about("Removes a list")
            .arg(Arg::with_name("list name")
                .help("The name of the list to be deleted")
                .required(true)))
}
