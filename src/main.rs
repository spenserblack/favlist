use clap::{App, Arg, crate_description, crate_name, crate_version};
use rusqlite::{Connection, NO_PARAMS};

const DEFAULT_DB_NAME: &str = "favlist.sqlite";

fn main() {

    let matches = App::new(crate_name!())
        .version(crate_version!())
        // .about(crate_description!())
        .arg(Arg::with_name("database")
            .long("db")
            .value_name("FILE")
            .help("The file to save/load your list(s)")
            .takes_value(true)
            .default_value(DEFAULT_DB_NAME))
        .get_matches();

    let db = matches.value_of("database").unwrap();

    let conn = Connection::open(db).unwrap();
}

mod table_data {
    use std::str::FromStr;
    
    pub struct Column<'a> {
        name: &'a str,
        not_null: bool,
    }
    
    impl<'a> FromStr for Column<'a> {
        type Err = String; // TODO Use better error type
        
        // TODO Use Regex for better parsing
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut not_null = false;
            let name: String = s.chars().enumerate().filter_map(|(i, c)| {
                if i == 0 && c == '!' {
                    not_null = true;
                    None
                } else {
                    Some(c)
                }
            }).collect();
            let name = name.as_str();
            
            let column = Column {
                not_null,
                name,
            };
            
            Ok(column)
        }
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
        
        #[test]
        fn plain_column() {
            let column = "Title".parse().unwrap();
            assert_eq!("Title", column.name);
            assert_eq!(false, column.not_null);
        }
        
        #[test]
        fn important_column() {
            let column = "!Title".parse().unwrap();
            assert_eq!("Title", column.name);
            assert_eq!(true, column.not_null);
        }
    }
}
