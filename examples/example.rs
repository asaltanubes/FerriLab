use ferrilab::*;
use pyo3::prelude::*;

fn main() -> PyResult<()> {
    // First let's create some measures manually from data obtained in laboratory.
    let time = measure!([0.0001346, 0.1134, 0.22734, 0.312324, 0.4019256, 0.5127634], [0.0000123, 0.0154, 0.012, 0.02943, 0.02544, 0.04872]; true);

    let position = measure!([0.0023, 1.41134, 2.425, 3.41515, 5.13545, 7.24524], [0.000123, 0.154, 0.2, 0.43, 0.544, 0.872]; true);

    // Or use the reader module to extract data from a file.
    let data = Reader::new("examples/data.txt", 0).read_to_measures();

    let _time = data[0].clone();
    let _position = data[1].clone();

    // Obtain some other value through the data. FerriLab calculate the errors for you.
    let speed = &position / &time;

    // Let's create a graph to visualize how speed changes over time.
    // Before the graph we have to fit the data.
    let (slope, intercept) = LinearFit::new(&time, &position).fit();

    // Text in plot is render with latex.
    use_latex(true)?;

    // Plots the values.
    Scatter::new(&time, &position).color("red").scatter()?;

    // Plots the line obtanied from the linear fit.
    Plot::new(&time, &time * &slope + &intercept)
        .color("black")
        .plot()?;

    // Render the x and y labels.
    Labels::new().xlabel(r#"$t / s$"#)?.ylabel(r#"$d / m$"#)?;

    // Shows the figure.
    show()?;

    // Create a table for your favourite report writer tool (latex or typst supported right now)
    println!(
        "{}",
        Table::new(
            vec![time.aprox(), position.aprox(), speed.aprox()],
            vec!["t/s", "x/m", "v/ms^(-1)"],
        )
        .typst()
    );

    Ok(())
}
