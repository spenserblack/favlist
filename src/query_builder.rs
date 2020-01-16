use std::fmt::{self, Display};

pub struct List<'a> {
    table_name: &'a str,
    filter_names: Option<Vec<&'a str>>,
}

impl<'a> List<'a> {
    pub fn new(table_name: &'a str, filter_names: Option<Vec<&'a str>>) -> Self {
        List {
            table_name,
            filter_names,
        }
    }
}

impl<'a> Display for List<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SELECT * FROM {table_name}", table_name = self.table_name)?;
        if let Some(filter_names) = &self.filter_names {
            let params: Vec<_> = (1..=filter_names.len()).map(|n| format!("?{}", n)).collect();
            write!(f, "\nWHERE ")?;
            let filters = filter_names
                .iter()
                .zip(params.iter())
                .map(|(filter, param)| {
                    format!(
                        "{filter} LIKE '%' || {param} || '%'",
                        filter = filter,
                        param = param,
                    )
                })
                .collect::<Vec<_>>()
                .join("\n  AND ");
            write!(f, "{}", filters)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_filter() {
        let list = List::new("Movies", None);
        let expected = "SELECT * FROM Movies";
        assert_eq!(expected, list.to_string());
    }

    #[test]
    fn filters() {
        let list = List::new("Movies", Some(vec![
            "Name",
            "Year",
        ]));
        let expected = "\
SELECT * FROM Movies
WHERE Name LIKE '%' || ?1 || '%'
  AND Year LIKE '%' || ?2 || '%'";
        assert_eq!(expected, list.to_string());
    }
}
