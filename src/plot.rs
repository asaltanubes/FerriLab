use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict};
use std::default::Default;

// Scatter
#[derive(Debug, Clone)]
pub struct Scatter {
    x_values: Vec<f64>,
    y_values: Vec<f64>,
    color: String,
    fill: bool,
    marker: String,
    size: usize,
    label: Option<String>,
    zorder: i32,
    xerrorbar: Option<Vec<f64>>,
    yerrorbar: Option<Vec<f64>>,
    errorbarcolor: String,
    barzorder: i32,
}

impl Scatter {
    pub fn new(x: impl Into<Vec<f64>>, y: impl Into<Vec<f64>>) -> Self {
        Scatter {
            x_values: x.into(),
            y_values: y.into(),
            color: String::from("tab:blue"),
            fill: true,
            marker: String::from('o'),
            size: 50,
            label: None,
            zorder: 100,
            xerrorbar: None,
            yerrorbar: None,
            errorbarcolor: String::from("tab:red"),
            barzorder: 0,
        }
    }

    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = color.into();
        self
    }
    pub fn fill(mut self, fill: bool) -> Self {
        self.fill = fill;
        self
    }
    pub fn marker(mut self, marker: impl Into<String>) -> Self {
        self.marker = marker.into();
        self
    }
    pub fn size(mut self, size: usize) -> Self {
        self.size = size;
        self
    }
    pub fn label(mut self, label: impl Into<Option<String>>) -> Self {
        self.label = label.into();
        self
    }
    pub fn zorder(mut self, zorder: i32) -> Self {
        self.zorder = zorder;
        self
    }
    pub fn xerrorbar(mut self, xerrorbar: impl Into<Option<Vec<f64>>>) -> Self {
        self.xerrorbar = xerrorbar.into();
        self
    }
    pub fn yerrorbar(mut self, yerrorbar: impl Into<Option<Vec<f64>>>) -> Self {
        self.yerrorbar = yerrorbar.into();
        self
    }
    pub fn errorbarcolor(mut self, errorbarcolor: impl Into<String>) -> Self {
        self.errorbarcolor = errorbarcolor.into();
        self
    }
    pub fn barzorder(mut self, barzorder: i32) -> Self {
        self.barzorder = barzorder;
        self
    }
    pub fn scatter(self) -> PyResult<()> {
        Python::with_gil(|py| {
            let plt = PyModule::import(py, "matplotlib.pyplot")?;
            plt.getattr("errorbar")?.call(
                (self.x_values.clone(), self.y_values.clone()),
                Some(self.clone().errorbarconfig(py)?),
            )?;
            plt.getattr("scatter")?.call(
                (self.x_values.clone(), self.y_values.clone()),
                Some(self.dotconfig(py)?),
            )?;
            Ok(())
        })
    }

    fn dotconfig(self, py: Python<'_>) -> PyResult<&pyo3::types::PyDict> {
        let facecolors = if self.fill {
            self.color.clone()
        } else {
            String::from("none")
        };
        let pydict = PyDict::new(py);
        pydict.set_item("s", self.size)?;
        pydict.set_item("marker", self.marker)?;
        pydict.set_item("facecolors", facecolors)?;
        pydict.set_item("edgecolors", self.color)?;
        pydict.set_item("label", self.label)?;
        pydict.set_item("zorder", self.zorder)?;
        Ok(pydict)
    }

    fn errorbarconfig(self, py: Python<'_>) -> PyResult<&pyo3::types::PyDict> {
        let pydict = PyDict::new(py);
        pydict.set_item("yerr", self.yerrorbar.clone())?;
        pydict.set_item("xerr", self.xerrorbar.clone())?;
        pydict.set_item("ecolor", self.errorbarcolor.clone())?;
        pydict.set_item("fmt", "none")?;
        pydict.set_item("zorder", self.barzorder)?;
        Ok(pydict)
    }
}

// Plot
#[derive(Debug, Clone)]
pub struct Plot {
    x_values: Vec<f64>,
    y_values: Vec<f64>,
    color: String,
    linestyle: String,
    linewidth: usize,
    label: Option<String>,
    zorder: i32,
}

impl Plot {
    pub fn new(x: impl Into<Vec<f64>>, y: impl Into<Vec<f64>>) -> Self {
        Plot {
            x_values: x.into(),
            y_values: y.into(),
            color: String::from("tab:blue"),
            linestyle: String::from("-"),
            linewidth: 2,
            label: None,
            zorder: 0,
        }
    }
    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = color.into();
        self
    }
    pub fn linestyle(mut self, linestyle: impl Into<String>) -> Self {
        self.linestyle = linestyle.into();
        self
    }
    pub fn linewidth(mut self, linewidth: usize) -> Self {
        self.linewidth = linewidth;
        self
    }
    pub fn label(mut self, label: impl Into<Option<String>>) -> Self {
        self.label = label.into();
        self
    }
    pub fn zorder(mut self, zorder: i32) -> Self {
        self.zorder = zorder;
        self
    }

    pub fn plot(self) -> PyResult<()> {
        Python::with_gil(|py| {
            let plt = PyModule::import(py, "matplotlib.pyplot")?;
            plt.getattr("plot")?.call(
                (self.x_values.clone(), self.y_values.clone()),
                Some(self.plot_config(py)?),
            )?;
            Ok(())
        })
    }
    fn plot_config(self, py: Python<'_>) -> PyResult<&PyDict> {
        let pydict = PyDict::new(py);
        pydict.set_item("color", self.color)?;
        pydict.set_item("linestyle", self.linestyle)?;
        pydict.set_item("linewidth", self.linewidth)?;
        pydict.set_item("label", self.label)?;
        pydict.set_item("zorder", self.zorder)?;
        Ok(pydict)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Labels {
    fontsize: usize,
}

impl Default for Labels {
    fn default() -> Self {
        Labels { fontsize: 18 }
    }
}

impl Labels {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fontsize(mut self, fontsize: usize) -> Self {
        self.fontsize = fontsize;
        self
    }

    pub fn xlabel(self, text: impl Into<String>) -> PyResult<Self> {
        Python::with_gil(|py| {
            let plt = PyModule::import(py, "matplotlib.pyplot")?;
            plt.getattr("xlabel")?.call(
                (text.into(),),
                Some([("fontsize", self.fontsize)].into_py_dict(py)),
            )?;
            Ok(self)
        })
    }
    pub fn ylabel(self, text: impl Into<String>) -> PyResult<Self> {
        Python::with_gil(|py| {
            let plt = PyModule::import(py, "matplotlib.pyplot")?;
            plt.getattr("ylabel")?.call(
                (text.into(),),
                Some([("fontsize", self.fontsize)].into_py_dict(py)),
            )?;
            Ok(self)
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ticks {
    fontsize: usize,
}

impl Default for Ticks {
    fn default() -> Self {
        Ticks { fontsize: 14 }
    }
}

impl Ticks {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn fontsize(mut self, fontsize: usize) -> Self {
        self.fontsize = fontsize;
        self
    }
    pub fn xticks(self) -> PyResult<Self> {
        Python::with_gil(|py| {
            let plt = PyModule::import(py, "matplotlib.pyplot")?;
            plt.getattr("xticks")?
                .call((), Some([("fontsize", self.fontsize)].into_py_dict(py)))?;
            Ok(self)
        })
    }

    pub fn yticks(self) -> PyResult<Self> {
        Python::with_gil(|py| {
            let plt = PyModule::import(py, "matplotlib.pyplot")?;
            plt.getattr("yticks")?
                .call((), Some([("fontsize", self.fontsize)].into_py_dict(py)))?;
            Ok(self)
        })
    }
}

pub fn figure(id: usize) -> PyResult<()> {
    Python::with_gil(|py| {
        let plt = PyModule::import(py, "matplotlib.pyplot")?;
        plt.getattr("figure")?
            .call((), Some([("id", id)].into_py_dict(py)))?;
        Ok(())
    })
}

#[derive(Debug, Clone, Copy)]
pub struct PlotSize {
    pub left: f64,
    pub right: f64,
    pub top: f64,
    pub bottom: f64,
    pub autosize: bool,
}

impl Default for PlotSize {
    fn default() -> Self {
        PlotSize {
            left: 0.1,
            right: 0.9,
            bottom: 0.1,
            top: 0.9,
            autosize: true,
        }
    }
}

impl PlotSize {
    fn plot_size(self, py: Python<'_>) -> PyResult<&PyDict> {
        let pydict = PyDict::new(py);
        pydict.set_item("left", self.left)?;
        pydict.set_item("right", self.right)?;
        pydict.set_item("bottom", self.bottom)?;
        pydict.set_item("top", self.top)?;
        Ok(pydict)
    }
}

pub fn plot_size(conf: PlotSize) -> PyResult<()> {
    Python::with_gil(|py| {
        let plt = PyModule::import(py, "matplotlib.pyplot")?;
        if conf.autosize {
            plt.getattr("tight_layout")?.call0()?;
        } else {
            plt.getattr("subplots_adjust")?
                .call((), Some(conf.plot_size(py)?))?;
        }
        Ok(())
    })
}

pub fn xscale(log: impl Into<String>) -> PyResult<()> {
    Python::with_gil(|py| {
        let plt = PyModule::import(py, "matplotlib.pyplot")?;
        plt.getattr("xscale")?.call1((log.into(),))?;
        Ok(())
    })
}

pub fn yscale(log: impl Into<String>) -> PyResult<()> {
    Python::with_gil(|py| {
        let plt = PyModule::import(py, "matplotlib.pyplot")?;
        plt.getattr("yscale")?.call1((log.into(),))?;
        Ok(())
    })
}

pub fn legend() -> PyResult<()> {
    Python::with_gil(|py| {
        let plt = PyModule::import(py, "matplotlib.pyplot")?;
        plt.getattr("legend")?.call0()?;
        Ok(())
    })
}

pub fn limits(
    bottom: Option<f64>,
    top: Option<f64>,
    left: Option<f64>,
    right: Option<f64>,
) -> PyResult<()> {
    Python::with_gil(|py| {
        let plt = PyModule::import(py, "matplotlib.pyplot")?;
        plt.getattr("xlim")?.call1((left, right))?;
        plt.getattr("ylim")?.call1((bottom, top))?;
        Ok(())
    })
}

pub fn use_latex(latex: bool) -> PyResult<()> {
    Python::with_gil(|py| {
        let plt = PyModule::import(py, "matplotlib.pyplot")?;
        plt.getattr("rcParams")?.set_item("text.usetex", latex)?;
        Ok(())
    })
}

pub fn save(path: &str) -> PyResult<()> {
    show_conf()?;
    Python::with_gil(|py| {
        let plt = PyModule::import(py, "matplotlib.pyplot")?;
        plt.getattr("savefig")?
            .call((path,), Some([("dpi", "figure")].into_py_dict(py)))?;
        Ok(())
    })
}

pub fn show() -> PyResult<()> {
    show_conf()?;
    Python::with_gil(|py| {
        let plt = PyModule::import(py, "matplotlib.pyplot")?;
        println!("{:?}", plt.getattr("rcParams")?.get_item("text.usetex")?);
        plt.getattr("show")?.call0()?;
        Ok(())
    })
}
fn show_conf() -> PyResult<()> {
    Python::with_gil(|py| {
        let loc = PyModule::import(py, "locale")?;
        loc.getattr("setlocale")?
            .call1((loc.getattr("LC_ALL")?, ""))?;

        let plt = PyModule::import(py, "matplotlib.pyplot")?;
        // plt.getattr("rcParams")?.set_item(ç"text.usetex", true)?;
        let pyd = PyDict::new(py);
        pyd.set_item("useLocale", true)?;
        pyd.set_item("style", "sci")?;
        pyd.set_item("scilimits", (-2, 2))?;
        plt.getattr("ticklabel_format")?.call((), Some(pyd))?;
        Ok(())
    })
}

pub fn add_latex_code(code: String) -> PyResult<()> {
    Python::with_gil(|py| {
        let plt = PyModule::import(py, "matplotlib.pyplot")?;
        plt.getattr("rc")?.call(
            ("text.latex",),
            Some([("preamble", String::from("\n").push_str(code.as_ref()))].into_py_dict(py)),
        )?;
        Ok(())
    })
}

pub fn execute_python(code: &str) -> PyResult<()> {
    Python::with_gil(|py| {
        py.eval(code, None, None)?;
        Ok(())
    })
}
