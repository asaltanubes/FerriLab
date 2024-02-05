use crate::objects::{Measure, Style};

pub struct TableBuilder<'a> {
    data: Vec<Measure>,
    header: Vec<&'a str>,
    transpose: bool,
    caption: &'a str,
    label: &'a str,
}

impl<'a> TableBuilder<'a> {
    pub fn new(data: Vec<Measure>, header: Vec<&str>) -> TableBuilder {
        TableBuilder {
            data,
            header,
            transpose: true,
            caption: "caption",
            label: "label",
        }
    }

    pub fn transpose(mut self, transpose: bool) -> Self {
        self.transpose = transpose;
        self
    }
    
    pub fn caption(mut self, caption: &'a str) -> Self {
        self.caption = caption;
        self
    }
    
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = label;
        self
    }

    pub fn typst(self) -> String {
        typst(self.data, self.header, self.transpose)
    }

    pub fn latex(self) -> String {
        latex(self.data, self.header, self.caption, self.label, self.transpose)
    }

}

/// Creates a table using your measures in typst format.
///
/// # Examples
///
/// ```rust
/// # use ferrilab::{measure, Measure, tables::typst};
/// let time = measure!([0.2, 0.3, 0.40, 0.5], [0.01, 0.02, 0.02, 0.04]);
/// let position = measure!([2.4, 3.4, 5.1, 7.2], [0.2, 0.4, 0.5, 0.8]);
/// let speed = &position / &time;
///
/// println!("{}", typst(vec![time, position, speed], vec!["t/s", "x/m", "v/ms-1"], true))
///
/// // Output
///
/// /*
/// table(
///     columns: 3,
///     align: center,
///         [t/s], [x/m], [v/ms-1]
///         [$0.2 plus.minus 0.01$], [$2.4 plus.minus 0.2$], [$2.6 plus.minus 0.2$]
///         [$0.3 plus.minus 0.02$], [$3.4 plus.minus 0.4$], [$3.7 plus.minus 0.4$]
///         [$0.4 plus.minus 0.02$], [$5.1 plus.minus 0.5$], [$5.5 plus.minus 0.5$]
///         [$0.5 plus.minus 0.04$], [$7.2 plus.minus 0.8$], [$7.7 plus.minus 0.8$]
/// )
///  */
/// ```

pub fn typst(data: Vec<Measure>, header: Vec<&str>, transpose: bool) -> String {
    let mut data = create_table_list(data, header, transpose, Style::TypstTable);

    data = data
        .into_iter()
        .map(|vec| vec.into_iter().map(|str| format!("[{}]", str)).collect())
        .collect();

    let tabular: String = data
        .iter()
        .map(|vec| format!("\n \t\t{}", vec.join(", ")))
        .collect::<Vec<String>>()
        .join("");

    let width = data.into_iter().map(|vec| vec.len()).max().unwrap();

    format!(
        "\t table(\n\t columns: {}, \n\t align: center, \n\t\t{} \n)",
        width, tabular
    )
}

/// Creates a table using your measures in latex format.
///
/// # Examples
///
/// ```rust
/// # use ferrilab::{measure, Measure, tables::latex};
/// let time = measure!([0.2, 0.3, 0.40, 0.5], [0.01, 0.02, 0.02, 0.04]);
/// let position = measure!([2.4, 3.4, 5.1, 7.2], [0.2, 0.4, 0.5, 0.8]);
/// let speed = &position / &time;
///
/// println!("{}", latex(vec![time, position, speed], vec!["t/s", "x/m", "v/ms-1"], "Caption", "label", true))
///
/// // Output
///
/// /*
/// \begin{table}[ht]
///     \centering
///     \caption{Caption}
///     \label{label}
///         \begin{tabular}{|c|c|c|}
///             t/s & x/m & v/ms-1\\
///             $0.2 \pm 0.01$ & $2.4 \pm 0.2$ & $2.6 \pm 0.2$\\
///             $0.3 \pm 0.02$ & $3.4 \pm 0.4$ & $3.7 \pm 0.4$\\
///             $0.4 \pm 0.02$ & $5.1 \pm 0.5$ & $5.5 \pm 0.5$\\
///             $0.5 \pm 0.04$ & $7.2 \pm 0.8$ & $7.7 \pm 0.8$\\
///        \end{tabular}
/// \end{table}
///  */
/// ```
pub fn latex(
    data: Vec<Measure>,
    header: Vec<&str>,
    caption: &str,
    label: &str,
    transpose: bool,
) -> String {
    let data = create_table_list(data, header, transpose, Style::LatexTable);

    let tabular: Vec<String> = data
        .iter()
        .map(|vec| format!("{}\\\\ \n\t\t", vec.join(" & ")))
        .collect();

    let width: usize = data.into_iter().map(|vec| vec.len()).max().unwrap();

    let tabular = format!(
        "\t \\begin{{tabular}}{{{}|}}\n\t\t{}\n\t\\end{{tabular}}",
        vec!["|c"; width].join(""),
        tabular.join("")
    );

    format!("\\begin{{table}}[ht]\n \\centering \n\n\\caption{{{}}}\n\\label{{{}}}\n\n{}\n\n\\end{{table}}", caption, label, tabular)
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn create_table_list(
    data: Vec<Measure>,
    mut header: Vec<&str>,
    transposed: bool,
    style: Style,
) -> Vec<Vec<String>> {
    let mut data: Vec<Vec<String>> = data
        .into_iter()
        .map(|measure| {
            measure
                .list_of_measures()
                .into_iter()
                .map(|measure| format!("{}", measure.change_style(style)))
                .collect()
        })
        .collect();

    let max_len = data.iter().map(|vec| vec.len()).max().unwrap();

    data = data
        .into_iter()
        .map(|mut vec| {
            if max_len > vec.len() {
                vec.extend(vec!["".to_string(); max_len - vec.len()]);
                vec
            } else {
                vec
            }
        })
        .collect();

    if !header.is_empty() {
        if header.len() < data.len() {
            header.extend(vec![""; data.len() - header.len()]);
        }
        data = data
            .into_iter()
            .zip(header)
            .map(|(mut data, head)| {
                data.insert(0, head.to_string());
                data
            })
            .collect::<Vec<Vec<String>>>();
    }

    if transposed {
        return transpose(data);
    }
    data
}
