use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

pub fn example() {
    Python::with_gil(|py| {
        let plt = PyModule::import(py, "matplotlib.pyplot").ok().unwrap();
        plt.getattr("plot")
            .expect("plot")
            .call1(([1, 2, 3], [4, 5, 6])).unwrap();
        let kargs = [("usetex", true)].into_py_dict(py);
        plt.getattr("rc").expect("rc").call(("text",), Some(kargs)).unwrap();
        plt.getattr("xlabel").expect("xlabel").call1((r"$\xi$",)).unwrap();
        plt.getattr("ylabel")
            .expect("ylabel")
            .call1((r"$\varepsilon$",)).unwrap();
        plt.getattr("show").expect("show").call0().unwrap();
    });
}
