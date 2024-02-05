use ferrilab as fl;
use fl::{measure, Measure, reader::read_to_measures, tables::typst, fit::linear_fit};
fn main() {
    // First let's create some measures manually from data obtained in laboratory.
    let time = measure!([0.0001346, 0.1134, 0.22734, 0.312324, 0.4019256, 0.5127634], [0.0000123, 0.0154, 0.012, 0.02943, 0.02544, 0.04872]; true);

    let position = measure!([0.0023, 1.41134, 2.425, 3.41515, 5.13545, 7.24524], [0.000123, 0.154, 0.2, 0.43, 0.544, 0.872]; true);

    // Or use the reader module to extract data from a file.
    let data = read_to_measures("data.txt", "\t", "\n", ",", 0).unwrap();

    let _time = data[0].clone();
    let _position = data[1].clone();

    // Obtain some other value through the data. FerriLab calculate the errors for you.
    let speed = &position / &time;

    // Let's create a graph to visualize how speed changes over time.
    // Before the graph we have to fit the data.
    let (slope, intercept) = linear_fit(&time, &position);

    // not plot module yet :(

    // Create a table for your favourite report writer tool (latex or typst supported right now)
    println!(
        "{}",
        typst(
            vec![time, position, speed],
            vec!["t/s", "x/m", "v/ms^(-1)"],
            true
        )
    )
}
