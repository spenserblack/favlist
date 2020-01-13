use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fmt::{self, Display},
    str::FromStr,
};

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

impl Column {
    pub fn declaration(&self) -> String {
        format!(
            "{} {} {}",
            self.name,
            self.data_type,
            if self.not_null { "NOT NULL" } else { "" },
        )
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
                Regex::new(r"(?P<important>^!)?(?P<name>\w+)@?(?P<type>\w+)?").unwrap();
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
        let column: Column = "!Title".parse().unwrap();
        assert_eq!("Title", column.name);
        assert_eq!(true, column.not_null);
        assert_matches!(column.data_type, DataType::Blob);
    }

    #[test]
    fn important_int_column() {
        let column: Column = "!Year@int".parse().unwrap();
        assert_eq!("Year", column.name);
        assert_eq!(true, column.not_null);
        assert_matches!(column.data_type, DataType::Integer);
    }

    #[test]
    fn invalid_column_type() {
        let column: Result<Column, _> = "!Year@what".parse();
        assert_matches!(column, Err(_));
    }

    #[test]
    fn important_int_column_declaration() {
        let column: Column = "!Year@int".parse().unwrap();
        assert_eq!("Year INTEGER NOT NULL", column.declaration());
    }
}
