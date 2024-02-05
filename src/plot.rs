use std::default::Default;
use pyo3::types::{IntoPyDict, PyDict};

fn main() {
    scatter(vec![1.0, 2.0], vec![1.0, 2.0], ScatterConfig{color: "tab:blue".into(), yerrorbar: Some(vec![0.1, 0.3]), fill: false, ..Default::default()});
    plot(vec![1.0, 2.0], [1.0, 2.0], PlotConfig::default());
    Python::with_gil(|py|{
        let plt = PyModule::import(py, "matplotlib.pyplot").unwrap();
        plt.getattr("show").unwrap().call0();
    })
    
}

// Scatter
#[derive(Debug, Clone)]
struct ScatterConfig{
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

impl Default for ScatterConfig{
    fn default() -> Self{
        ScatterConfig{
            color: String::from("tab:blue"),
            fill: true,
            marker: String::from('o'),
            size: 50,
            label: None,
            zorder: 100,
            xerrorbar: None,
            yerrorbar: None,
            errorbarcolor: String::from("tab:red"),
            barzorder: 0
        }
    }
}

impl ScatterConfig {
    fn dotconfig(self, py: Python<'_>) -> PyResult<&pyo3::types::PyDict>{
        let facecolors = if self.fill {self.color.clone()} else {String::from("none")};
        let pydict = PyDict::new(py);
        pydict.set_item("s", self.size)?;
        pydict.set_item("marker", self.marker)?;
        pydict.set_item("facecolors", facecolors)?;
        pydict.set_item("edgecolors", self.color)?;
        pydict.set_item("label", self.label)?;
        pydict.set_item("zorder", self.zorder)?;      
        Ok(pydict)
    }

    fn errorbarconfig(self, py: Python<'_>) -> PyResult<&pyo3::types::PyDict>{
        let pydict = PyDict::new(py);
        pydict.set_item("yerr", self.yerrorbar.clone())?;
        pydict.set_item("xerr", self.xerrorbar.clone())?;
        pydict.set_item("ecolor", self.errorbarcolor.clone())?;
        pydict.set_item("fmt", "none")?;
        pydict.set_item("zorder", self.barzorder)?;
        Ok(pydict)
    }
}

fn scatter(x: impl Into<Vec<f64>>, y: impl Into<Vec<f64>>, conf: ScatterConfig) -> PyResult<()>{
    Python::with_gil(|py| {
        let plt = PyModule::import(py, "matplotlib.pyplot")?;
        let x = x.into();
        let y = y.into();
        plt.getattr("errorbar")?.call((x.clone(), y.clone()), Some(conf.clone().errorbarconfig(py)?))?;
        plt.getattr("scatter")?.call((x, y), Some(conf.dotconfig(py)?))?;
    Ok(())
    })
}


// Plot
#[derive(Debug, Clone)]
struct PlotConfig{
    color: String,
    linestyle: String,
    linewidth: usize,
    label: Option<String>,
    zorder: i32,
}

impl Default for PlotConfig{
    fn default() -> Self {
        PlotConfig{
            color: String::from("tab:blue"),
            linestyle: String::from("-"),
            linewidth: 2,
            label: None,
            zorder: 0
        }
    }
}

impl PlotConfig{
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

fn plot(x: impl Into<Vec<f64>>, y: impl Into<Vec<f64>>, conf: PlotConfig) -> PyResult<()>{
    Python::with_gil(|py| {
        let plt = PyModule::import(py, "matplotlib.pyplot")?;
        let x = x.into();
        let y = y.into();
        plt.getattr("plot")?.call((x, y), Some(conf.plot_config(py)?))?;
        Ok(())
    })
}



fn legend() -> PyResult<()>{
    Python::with_gil(|py| {
        let plt = PyModule::import(py, "matplotlib.pyplot")?;
        plt.getattr("legend")?.call0()?;
        Ok(())
    })
}


#[derive(Debug, Clone, Copy)]
struct LabelConfig{
    fontsize: usize,
}

impl Default for LabelConfig{
   fn default() -> Self {
       LabelConfig{fontsize: 15}
   } 
}

impl LabelConfig{
    fn label_conf(self, py: Python<'_>) -> PyResult<&PyDict>{
        let pydict = PyDict::new(py);
        pydict.set_item("fontsize", self.fontsize)?;
        Ok(pydict)
    }
}

fn xlabel(text: impl Into<String>, conf: LabelConfig) -> PyResult<()>{
    Python::with_gil(|py|{
        let plt = PyModule::import(py, "matplotlib.pyplot")?;
        plt.getattr("xlabel")?.call((text.into(), ), Some(conf.label_conf(py)?))?;
        Ok(())
    })
}

fn ylabel(text: impl Into<String>, conf: LabelConfig) -> PyResult<()>{
    Python::with_gil(|py|{
        let plt = PyModule::import(py, "matplotlib.pyplot")?;
        plt.getattr("ylabel")?.call((text.into(), ), Some(conf.label_conf(py)?))?;
        Ok(())
    })
}


#[derive(Debug, Clone, Copy)]
struct TickConfig{
    fontsize: usize,
}

impl Default for TickConfig{
    fn default() -> Self {
        TickConfig{fontsize: 14}
    }
}

impl TickConfig{
    fn tick_conf(self, py: Python<'_>) -> PyResult<&PyDict> {
       let pydict = PyDict::new(py);
       pydict.set_item("fontsize", self.fontsize)?;
       Ok(pydict)
    }
}

fn xticks(conf: TickConfig) -> PyResult<()>{
    Python::with_gil(|py| {
        let plt = PyModule::import(py, "matplotlib.pyplot")?;
        plt.getattr("xticks")?.call((), Some(conf.tick_conf(py)?))?;
        Ok(())
    })
}

fn yticks(conf: TickConfig) -> PyResult<()>{
    Python::with_gil(|py| {
        let plt = PyModule::import(py, "matplotlib.pyplot")?;
        plt.getattr("yticks")?.call((), Some(conf.tick_conf(py)?))?;
        Ok(())
    })
}


#[derive(Debug, Clone, Copy)]
struct FigConf{
    id: usize,
}

impl Default for FigConf{
    fn default() -> Self {
        FigConf{id: 0}
    }
}

impl FigConf{
    fn fig_conf(self, py: Python<'_>) -> PyResult<&PyDict> {
       let pydict = PyDict::new(py);
       pydict.set_item("id", self.id)?;
       Ok(pydict)
    }
}

fn figure(conf: FigConf) -> PyResult<()>{
    Python::with_gil(|py|{
        let plt = PyModule::import(py, "matplotlib.pyplot")?;
        plt.getattr("figure")?.call((), Some(conf.fig_conf(py)?))?;
        Ok(())
    })
}


#[derive(Debug, Clone, Copy)]
struct PlotSize{
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
    autosize: bool
}

impl Default for PlotSize{
    fn default() -> Self {
        PlotSize{
            left:     0.1,
            right:    0.9,
            bottom:   0.1,
            top:      0.9,
            autosize: true
        }
    }
}

impl PlotSize{
    fn plot_size(self, py: Python<'_>) -> PyResult<&PyDict>{
       let pydict = PyDict::new(py);
       pydict.set_item("left", self.left)?;
       pydict.set_item("right", self.right)?;
       pydict.set_item("bottom", self.bottom)?;
       pydict.set_item("top", self.top)?;
       Ok(pydict)
    }
}

fn plot_size(conf: PlotSize) -> PyResult<()>{
    Python::with_gil(|py| {
        let plt = PyModule::import(py, "matplotlib.pyplot")?;
        if conf.autosize {
            plt.getattr("tight_layout")?.call0()?;
        }
        else {
            plt.getattr("subplots_adjust")?.call((), Some(conf.plot_size(py)?))?;
        }
        Ok(())
    })
}

fn xscale(log: impl Into<String>) -> PyResult<()>{
    Python::with_gil(|py| {
        let plt = PyModule::import(py, "matplotlib.pyplot")?;
        plt.getattr("xscale")?.call1((log.into(), ))?;
        Ok(())
    })
}

fn yscale(log: impl Into<String>) -> PyResult<()>{
    Python::with_gil(|py| {
        let plt = PyModule::import(py, "matplotlib.pyplot")?;
        plt.getattr("yscale")?.call1((log.into(), ))?;
        Ok(())
    })
}

fn use_latex(usar: bool) -> PyResult<()>{
    Python::with_gil(|py|{ 
    let plt = PyModule::import(py, "matplotlib.pyplot")?;
    plt.getattr("rc")?.call(("text", ), Some([("usetex", usar)].into_py_dict(py)))?;
    Ok(())
})
}

fn add_latex_code(code: String) -> PyResult<()>{
    Python::with_gil(|py| {
        let plt = PyModule::import(py, "matplotlib.pyplot")?;
        plt.getattr("rc")?.call(("text.latex", ), Some([("preamble", String::from("\n").push_str(code.as_ref()))].into_py_dict(py)))?;
        Ok(())
    })
}

fn execute_python(code: &str) -> PyResult<()>{
    Python::with_gil(|py| {
        py.eval(code, None, None)?;
        Ok(())
    })
}
