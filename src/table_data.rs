use regex::Regex;
use std::str::FromStr;

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

impl FromStr for Column {
    type Err = String; // TODO Use better error type

    // TODO Use Regex for better parsing
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r"(?P<important>^!)?(?P<name>\w+)@?(?P<type>\w+)?").unwrap();
        let caps = regex.captures(s).unwrap();
        let not_null = caps.name("important").is_some();
        let name = caps.name("name").unwrap().as_str().into();
        let data_type = if let Some(cap) = caps.name("type") {
            match cap.as_str() {
                "int" => DataType::Integer,
                "text" => DataType::Text,
                "real" => DataType::Real,
                "num" => DataType::Numeric,
                s => unimplemented!("{}", s),
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
    use assert_matches::assert_matches;
    use super::*;

    #[test]
    fn plain_column() {
        let column: Column = "Title".parse().unwrap();
        assert_eq!("Title", column.name);
        assert_eq!(false, column.not_null);
        let data_type = column.data_type;
        assert_matches!(DataType::Blob, data_type);
    }

    #[test]
    fn int_column() {
        let column: Column = "Year@int".parse().unwrap();
        assert_eq!("Year", column.name);
        assert_eq!(false, column.not_null);
        let data_type = column.data_type;
        assert_matches!(DataType::Integer, data_type);
    }

    #[test]
    fn important_column() {
        let column: Column = "!Title".parse().unwrap();
        assert_eq!("Title", column.name);
        assert_eq!(true, column.not_null);
        let data_type = column.data_type;
        assert_matches!(DataType::Blob, data_type);
    }

    #[test]
    fn important_int_column() {
        let column: Column = "!Year@int".parse().unwrap();
        assert_eq!("Year", column.name);
        assert_eq!(true, column.not_null);
        let data_type = column.data_type;
        assert_matches!(DataType::Integer, data_type);
    }
}
