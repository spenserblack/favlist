use std::fmt::{self, Display};

pub struct Sub<'a> {
    table_name: &'a str,
    filter_names: Vec<&'a str>,
}

pub struct Edit<'a> {
    table_name: &'a str,
    /// This *should* always be an int, but, for now, will just let `rustqlite`
    /// handle an invalid value.
    id: &'a str,
    column_names: Vec<&'a str>,
}

pub struct List<'a> {
    table_name: &'a str,
    filter_names: Option<Vec<&'a str>>,
}

impl<'a> Sub<'a> {
    pub fn new(table_name: &'a str, filter_names: Vec<&'a str>) -> Self {
        Sub {
            table_name,
            filter_names,
        }
    }
}

impl<'a> Edit<'a> {
    pub fn new(table_name: &'a str, id: &'a str, column_names: Vec<&'a str>) -> Self {
        Edit {
            table_name,
            id,
            column_names,
        }
    }
}

impl<'a> List<'a> {
    pub fn new(table_name: &'a str, filter_names: Option<Vec<&'a str>>) -> Self {
        List {
            table_name,
            filter_names,
        }
    }
}

impl<'a> Display for Sub<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DELETE FROM {table_name}\nWHERE ", table_name = self.table_name)?;
        let params: Vec<_> = (1..=self.filter_names.len()).map(|n| format!("?{}", n)).collect();
        let filters = self.filter_names
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
        Ok(())
    }
}

impl<'a> Display for Edit<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UPDATE {table_name}\n", table_name = self.table_name)?;
        let params: Vec<_> = (1..=self.column_names.len()).map(|n| format!("?{}", n)).collect();
        let columns = self.column_names
            .iter()
            .zip(params.iter())
            .map(|(column, param)| {
                format!(
                    "{column} = {param}",
                    column = column,
                    param = param,
                )
            })
            .collect::<Vec<_>>()
            .join(",\n  ");
        write!(f, "SET {}\n", columns)?;
        write!(f, "WHERE id = {id}", id = self.id)?;
        Ok(())
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
    fn list_no_filter() {
        let list = List::new("Movies", None);
        let expected = "SELECT * FROM Movies";
        assert_eq!(expected, list.to_string());
    }

    #[test]
    fn list_filters() {
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

    #[test]
    fn sub_filters() {
        let sub = Sub::new("Movies", vec!["Name", "Year",]);
        let expected = "\
DELETE FROM Movies
WHERE Name LIKE '%' || ?1 || '%'
  AND Year LIKE '%' || ?2 || '%'";
        assert_eq!(expected, sub.to_string());
    }

    #[test]
    fn edit() {
        let edit = Edit::new("Movies", "1", vec!["Name", "Year",]);
        let expected = "\
UPDATE Movies
SET Name = ?1,
  Year = ?2
WHERE id = 1";
        assert_eq!(expected, edit.to_string());
    }
}
