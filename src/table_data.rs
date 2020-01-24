use clap::Values;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fmt::{self, Display},
    str::FromStr,
};

pub struct Table {
    name: String,
    columns: Vec<Column>,
}

#[derive(Debug)]
pub struct Column {
    name: String,
    not_null: bool,
    data_type: DataType,
}

#[derive(Debug)]
enum DataType {
    Integer,
    Text,
    /// No datatype
    Blob,
    Real,
    Numeric,
}

impl Table {
    // TODO Pass Iterator instead of Vec?
    // TODO Return `Result` (and eliminate `unwrap`s)
    pub fn new<'a>(name: &str, columns: Values<'a>) -> Self {
        let name = name.into();
        let columns = columns.map(|s| s.parse().unwrap()).collect();
        Table { name, columns }
    }

    pub fn declaration(&self) -> String {
        format!(
            "CREATE TABLE {table_name} (
id INTEGER PRIMARY KEY,
{columns}
)\n",
            table_name = self.name,
            columns = self
                .columns
                .iter()
                .map(|c| c.declaration())
                .collect::<Vec<_>>()
                .join(",\n"),
        )
    }
}

impl Column {
    fn declaration(&self) -> String {
        let declaration = format!("{} {}", self.name, self.data_type,);
        if self.not_null {
            format!("{} NOT NULL", declaration)
        } else {
            declaration
        }
    }
}

impl Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DataType::*;

        match self {
            Integer => write!(f, "INTEGER"),
            Text => write!(f, "TEXT"),
            Blob => write!(f, "BLOB"),
            Real => write!(f, "REAL"),
            Numeric => write!(f, "NUMERIC"),
        }
    }
}

impl FromStr for Column {
    type Err = String; // TODO Use better error type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref COLUMN_REGEX: Regex =
                Regex::new(r"(?P<important>^\~)?(?P<name>\w+)@?(?P<type>\w+)?").unwrap();
        }
        let caps = COLUMN_REGEX.captures(s).unwrap();
        let not_null = caps.name("important").is_some();
        let name = caps.name("name").unwrap().as_str().into();
        let data_type = if let Some(cap) = caps.name("type") {
            match cap.as_str() {
                "int" => DataType::Integer,
                "text" => DataType::Text,
                "real" => DataType::Real,
                "num" => DataType::Numeric,
                s => return Err(format!("Invalid column name: {}", s)),
            }
        } else {
            DataType::Blob
        };

        let column = Column {
            not_null,
            name,
            data_type,
        };

        Ok(column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;
    use pretty_assertions::assert_eq;

    #[test]
    fn plain_column() {
        let column: Column = "Title".parse().unwrap();
        assert_eq!("Title", column.name);
        assert_eq!(false, column.not_null);
        assert_matches!(column.data_type, DataType::Blob);
    }

    #[test]
    fn int_column() {
        let column: Column = "Year@int".parse().unwrap();
        assert_eq!("Year", column.name);
        assert_eq!(false, column.not_null);
        assert_matches!(column.data_type, DataType::Integer);
    }

    #[test]
    fn important_column() {
        let column: Column = "~Title".parse().unwrap();
        assert_eq!("Title", column.name);
        assert_eq!(true, column.not_null);
        assert_matches!(column.data_type, DataType::Blob);
    }

    #[test]
    fn important_int_column() {
        let column: Column = "~Year@int".parse().unwrap();
        assert_eq!("Year", column.name);
        assert_eq!(true, column.not_null);
        assert_matches!(column.data_type, DataType::Integer);
    }

    #[test]
    fn invalid_column_type() {
        let column: Result<Column, _> = "~Year@what".parse();
        assert_matches!(column, Err(_));
    }

    #[test]
    fn important_int_column_declaration() {
        let column: Column = "~Year@int".parse().unwrap();
        assert_eq!("Year INTEGER NOT NULL", column.declaration());
    }

    #[test]
    fn table_declaration() {
        let expected = "\
CREATE TABLE Movies (
id INTEGER PRIMARY KEY,
Title BLOB NOT NULL,
Year INTEGER
)\n";
        let table = Table {
            name: "Movies".into(),
            columns: vec!["~Title".parse().unwrap(), "Year@int".parse().unwrap()],
        };

        assert_eq!(expected, table.declaration(),);
    }
}
