use {
    crate::Measure,
    std::{fs::read_to_string, io::Error, path::Path},
};

pub struct ReadBuilder<'a> {
    file: &'a str,
    separator: &'a str,
    line: &'a str,
    decimal: &'a str,
    headers: usize,
    by_columns: bool,
}

impl<'a> ReadBuilder<'a> {
    pub fn new(file: &str, headers: usize) -> ReadBuilder {
        ReadBuilder {
            file,
            separator: "\t",
            line: "\n",
            decimal: ",",
            headers,
            by_columns: true
        }
    }

    pub fn separator(mut self, separator: &'a str) -> Self {
        self.separator = separator;
        self
    }
    
    pub fn line(mut self, line: &'a str) -> Self {
        self.line = line;
        self
    }
    
    pub fn decimal(mut self, decimal: &'a str) -> Self {
        self.decimal = decimal;
        self
    }
    
    pub fn by_columns(mut self, by_columns: bool) -> Self {
        self.by_columns = by_columns;
        self
    }

    pub fn read_file(self) ->  Result<Vec<Vec<Option<f64>>>, Error> {
        read_file(self.file, self.separator, self.line, self.decimal, self.headers, self.by_columns)
    }
    
    pub fn read_to_measures(self) ->  Result<Vec<Measure>, Error> {
        read_to_measures(self.file, self.separator, self.line, self.decimal, self.headers)
    }

}


/// Extracts data from a file with csv format or similar.
fn read_file(
    file: &str,
    separator: &str,
    line: &str,
    decimal: &str,
    headers: usize,
    by_columns: bool,
) -> Result<Vec<Vec<Option<f64>>>, Error> {
    let file = read_to_string(Path::new(file))?;

    let rows: Vec<&str> = file
        .split(line)
        .filter(|str| !str.trim().is_empty())
        .skip(headers)
        .collect();

    let mut data: Vec<Vec<Option<f64>>> = rows
        .into_iter()
        .map(|row| {
            row.split(separator)
                .map(|str| {
                    if str.trim().is_empty() {
                        None
                    } else {
                        Some(str.trim())
                    }
                })
                .collect()
        })
        .collect::<Vec<Vec<Option<&str>>>>()
        .into_iter()
        .map(|row_vec| {
            row_vec
                .into_iter()
                .map(|str_option| {
                    str_option.as_ref().map(|str| {
                        str.trim()
                            .replace(decimal, ".")
                            .parse()
                            .expect("Non number found")
                    })
                })
                .collect()
        })
        .collect();

    if by_columns {
        let max_len = data.iter().map(|vec| vec.len()).max().unwrap();

        data = (0..max_len)
            .map(|index| {
                data.iter()
                    .filter(|vec| index < vec.len() || vec[index].is_some())
                    .map(|vec| vec[index])
                    .collect()
            })
            .collect();
    }

    Ok(data)
}

/// Extracts data from a file creating measures by asuming each pair of columns
/// correspond to the value and error of a measure.

fn read_to_measures(
    file: &str,
    separator: &str,
    line: &str,
    decimal: &str,
    headers: usize,
) -> Result<Vec<Measure>, Error> {
    let data = read_file(file, separator, line, decimal, headers, true)?;
    Ok(data
        .iter()
        .step_by(2)
        .zip(data.iter().skip(1).step_by(2))
        .map(|(value, error)| {
            Measure::new(
                value
                    .iter()
                    .take_while(|val| val.is_some())
                    .map(|val| val.unwrap())
                    .collect(),
                error
                    .iter()
                    .take_while(|err| err.is_some())
                    .map(|err| err.unwrap())
                    .collect(),
                true,
            )
            .unwrap()
        })
        .collect())
}
