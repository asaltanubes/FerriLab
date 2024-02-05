use {
    crate::Measure,
    std::{fs::read_to_string, io::Error, path::Path},
};

/// Object to read data from a file with all required parameters.
pub struct Reader<'a> {
    file: &'a str,
    separator: &'a str,
    line: &'a str,
    decimal: &'a str,
    headers: usize,
    by_columns: bool,
}

impl<'a> Reader<'a> {
    /// Constructs a new Reader with some default values that can be changed.
    pub fn new(file: &str, headers: usize) -> Reader {
        Reader {
            file,
            separator: "\t",
            line: "\n",
            decimal: ",",
            headers,
            by_columns: true,
        }
    }
    /// Character separating the columns in a row, by default "\t".
    pub fn separator(mut self, separator: &'a str) -> Self {
        self.separator = separator;
        self
    }
    /// Character separating rows, by default "\n".
    pub fn line(mut self, line: &'a str) -> Self {
        self.line = line;
        self
    }
    /// Decimal separator, "," by default.
    pub fn decimal(mut self, decimal: &'a str) -> Self {
        self.decimal = decimal;
        self
    }
    /// Changes how to read the data, false for horizontal and true for vertical,
    /// true by default.
    pub fn by_columns(mut self, by_columns: bool) -> Self {
        self.by_columns = by_columns;
        self
    }
    /// Extracts data from a file with csv format or similar.
    pub fn read_file(self) -> Result<Vec<Vec<Option<f64>>>, Error> {
        read_file(
            self.file,
            self.separator,
            self.line,
            self.decimal,
            self.headers,
            self.by_columns,
        )
    }
    /// Extracts data from a file creating measures by asuming each pair of columns
    /// correspond to the value and error of a measure.
    pub fn read_to_measures(self) -> Vec<Measure> {
        read_to_measures(
            self.file,
            self.separator,
            self.line,
            self.decimal,
            self.headers,
        ).unwrap()
    }
}

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
