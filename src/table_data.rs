use std::str::FromStr;

pub struct Column {
    name: String,
    not_null: bool,
}

impl FromStr for Column {
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
        let column: Column = "Title".parse().unwrap();
        assert_eq!("Title", column.name);
        assert_eq!(false, column.not_null);
    }

    #[test]
    fn important_column() {
        let column: Column = "!Title".parse().unwrap();
        assert_eq!("Title", column.name);
        assert_eq!(true, column.not_null);
    }
}
