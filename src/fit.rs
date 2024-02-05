use crate::Measure;

/// Object to create a CurveFit with all required parameters.
#[derive(Debug, Clone)]
pub struct CurveFit<F: Fn(&f64, &[f64]) -> f64> {
    model: F,
    x_values: Vec<f64>,
    y_values: Vec<f64>,
    yerr: Vec<f64>,
    initial_point: Vec<f64>,
    tolerance: f64,
    max_iterations: Option<usize>,
    initial_simplex_scale: f64,
}

impl<F: Fn(&f64, &[f64]) -> f64> CurveFit<F> {
    /// Constructs a new CurveFit with some default values that can be changed.
    pub fn new(model: F, x_values: impl Into<Vec<f64>>, y_values: impl Into<Vec<f64>>) -> Self {
        let x_values = x_values.into();
        let n = x_values.len();
        CurveFit {
            model,
            x_values,
            y_values: y_values.into(),
            yerr: vec![1.0; n],
            initial_point: Vec::new(),
            tolerance: 1e-6,
            max_iterations: None,
            initial_simplex_scale: 0.5,
        }
    }
    /// Initial points required for calculating the curve fit.
    pub fn initial_point(mut self, initial_point: impl Into<Vec<f64>>) -> Self {
        self.initial_point = initial_point.into();
        self
    }
    /// If passed, calculates the weigthed curve fit considaring the y error.
    pub fn y_error(mut self, yerr: Vec<f64>) -> Self {
        self.yerr = yerr;
        self
    }

    /// In case you want the curve fit algorithm to stop at some point, by default None.
    pub fn max_iterations(mut self, max_iterations: impl Into<Option<usize>>) -> Self {
        self.max_iterations = max_iterations.into();
        self
    }
    /// Set initial points to zero.
    pub fn initial_zeros(mut self, number_of_components: usize) -> Self {
        self.initial_point = std::iter::repeat(0.0_f64)
            .take(number_of_components)
            .collect::<Vec<_>>();
        self
    }
    /// Set initial points to one.
    pub fn initial_ones(mut self, number_of_components: usize) -> Self {
        self.initial_point = std::iter::repeat(1.0_f64)
            .take(number_of_components)
            .collect();
        self
    }
    /// Custom precision on the curve fit, by default 1e-6.
    pub fn tolerance(mut self, tol: impl Into<f64>) -> Self {
        self.tolerance = tol.into();
        self
    }
    /// Generates n+1 points using the initial one for calculating the curve
    /// fit, by default 0.5.
    /// If the scale is a lot smaller than the expected value of the coeficients
    /// it may result in errors in the fit.
    pub fn initial_simplex_scale(mut self, scale: impl Into<f64>) -> Self {
        self.initial_simplex_scale = scale.into();
        self
    }

    /// Takes the arbitrary function and aproximates to the curve using
    /// every parameter established.
    pub fn fit(&self) -> Vec<Measure> {
        curve_fit(
            &self.model,
            &self.x_values,
            &self.y_values,
            &self.yerr,
            &self.initial_point,
            self.max_iterations,
            self.tolerance,
            self.initial_simplex_scale,
        )
    }

    pub fn r_value(&self) -> f64 {
        let parameters = self.fit();
        let ss_res = self
            .x_values
            .iter()
            .zip(self.y_values.iter())
            .map(|(x, y)| {
                y - (self.model)(
                    x,
                    &parameters
                        .iter()
                        .map(|par| par.value()[0])
                        .collect::<Vec<_>>(),
                )
            })
            .map(|r| r.powi(2))
            .sum::<f64>();

        let ss_total = self
            .y_values
            .iter()
            .map(|yi| {
                (yi - self.y_values.iter().sum::<f64>() / (self.y_values.len() as f64)).powi(2)
            })
            .sum::<f64>();

        (1.0 - (ss_res / ss_total)).sqrt()
    }
}

/// Object to create a LinearFit with all required parameters.
#[derive(Debug, Clone)]
pub struct LinearFit {
    x_values: Vec<f64>,
    y_values: Vec<f64>,
    yerr: Option<Vec<f64>>,
}

impl LinearFit {
    /// Constructs a new LinearFit with some default values that can be changed.
    pub fn new(x_values: impl Into<Vec<f64>>, y_values: impl Into<Vec<f64>>) -> Self {
        LinearFit {
            x_values: x_values.into(),
            y_values: y_values.into(),
            yerr: None,
        }
    }
    /// If passed, calculates the weigthed curve fit considaring the y error.
    pub fn y_error(mut self, yerr: Vec<f64>) -> Self {
        self.yerr = Some(yerr);
        self
    }

    /// Given the x and y values returns the slope and the intercept of a
    /// straight line by least squares method or weighted least squares method
    /// if yerr is given.
    pub fn fit(&self) -> (Measure, Measure) {
        if let Some(yerr) = &self.yerr {
            wlinear_fit(&self.x_values, &self.y_values, yerr)
        } else {
            linear_fit(&self.x_values, &self.y_values)
        }
    }
    /// Calculates the coeficient of linear correlation
    pub fn r_value(&self) -> f64 {
        let x_mean = self.x_values.iter().sum::<f64>() / (self.x_values.len() as f64);
        let y_mean = self.y_values.iter().sum::<f64>() / (self.y_values.len() as f64);
        let x_deviation: Vec<f64> = self.x_values.iter().map(|val| val - x_mean).collect();
        let y_deviation: Vec<f64> = self.y_values.iter().map(|val| val - y_mean).collect();
        let sigma_x = x_deviation.iter().map(|val| val.powi(2)).sum::<f64>();
        let sigma_y = y_deviation.iter().map(|val| val.powi(2)).sum::<f64>();

        x_deviation
            .into_iter()
            .zip(y_deviation)
            .map(|(xd, yd)| xd * yd)
            .sum::<f64>()
            / (sigma_x * sigma_y).sqrt()
    }
}

// ------------- Linear fit and Weigthed linear fit -------------

fn linear_fit(x: &[f64], y: &[f64]) -> (Measure, Measure) {
    assert_eq!(
        x.len(),
        y.len(),
        "Expected x and y vectors to be the same length, got x.len() = {}, y.len() = {}",
        x.len(),
        y.len()
    );
    let n = x.len() as f64;
    let sum_x: f64 = x.iter().sum();
    let sum_y: f64 = y.iter().sum();
    let sum_xy: f64 = x.iter().zip(y.iter()).map(|(x, y)| x * y).sum();
    let sum_x2: f64 = x.iter().map(|x| x.powf(2.0)).sum();

    let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x.powf(2.0));

    let n0 = (sum_y * sum_x2 - sum_x * sum_xy) / (n * sum_x2 - sum_x.powf(2.0));

    let sigma_y: f64 = x
        .iter()
        .zip(y.iter())
        .map(|(x, y)| (y - (slope * x + n0)).powi(2) / (n - 2.0))
        .sum::<f64>()
        .sqrt();

    let sigma_slope = sigma_y * (n / (n * sum_x2 - sum_x.powi(2))).sqrt();
    let sigma_n0 = sigma_y * (sum_x2 / (n * sum_x2 - sum_x.powi(2))).sqrt();

    let slope = Measure::new(vec![slope], vec![sigma_slope], false).unwrap();
    let n0 = Measure::new(vec![n0], vec![sigma_n0], false).unwrap();

    (slope, n0)
}

fn wlinear_fit(x: &[f64], y: &[f64], yerr: &[f64]) -> (Measure, Measure) {
    assert_eq!(
        x.len(),
        y.len(),
        "Expected x and y vectors to be the same length, got x.len() = {}, y.len() = {}",
        x.len(),
        y.len()
    );
    assert_eq!(
        x.len(),
        yerr.len(),
        "Expected y error, x and y vectors to be the same length, got x.len() = {}, y.len() = {}, yerr.len() = {}",
        x.len(),
        y.len(),
        yerr.len()
    );
    let w: Vec<f64> = yerr.iter().map(|yerr| 1.0 / yerr.powi(2)).collect();
    let sum_w: f64 = w.iter().sum();
    let sum_xw: f64 = x.iter().zip(w.iter()).map(|(x, w)| x * w).sum();
    let sum_x2w: f64 = x.iter().zip(w.iter()).map(|(x, w)| x.powi(2) * w).sum();
    let sum_yw: f64 = y.iter().zip(w.iter()).map(|(y, w)| y * w).sum();
    let sum_xyw: f64 = x
        .iter()
        .zip(w.iter().zip(y.iter()))
        .map(|(x, (w, y))| x * y * w)
        .sum();

    let wslope: f64 = (sum_w * sum_xyw - sum_xw * sum_yw) / (sum_w * sum_x2w - sum_xw.powi(2));
    let wn0: f64 = (sum_yw * sum_x2w - sum_xw * sum_xyw) / (sum_w * sum_x2w - sum_xw.powi(2));
    let wsigma_slope: f64 = (sum_w / (sum_w * sum_x2w - sum_xw.powi(2))).sqrt();
    let wsigma_n0 = (sum_x2w / (sum_w * sum_x2w - sum_xw.powi(2))).sqrt();

    let wslope = Measure::new(vec![wslope], vec![wsigma_slope], false).unwrap();
    let wn0 = Measure::new(vec![wn0], vec![wsigma_n0], false).unwrap();

    (wslope, wn0)
}

// ------------------------- Curve fit -------------------------

fn curve_fit<F>(
    model: &F,
    x: &[f64],
    y: &[f64],
    yerr: &[f64],
    initial_point: &[f64],
    max_iterations: Option<usize>,
    tol: f64,
    scale: f64,
) -> Vec<Measure>
where
    F: Fn(&f64, &[f64]) -> f64,
{
    assert_eq!(
        x.len(),
        y.len(),
        "Expected x and y vectors to be the same length, got x.len() = {}, y.len() = {}",
        x.len(),
        y.len()
    );
    let n = x.len();
    let objective_function = |coef: &[f64]| {
        x.iter()
            .zip(y.iter())
            .zip(yerr.iter())
            .map(|((x, y), ye)| ((y - model(x, coef)) / ye).powi(2))
            .sum()
    };
    let result = nelder_mead(
        &objective_function,
        initial_point,
        max_iterations,
        tol,
        scale,
    );

    let hessian_matrix = calculate_hessian_matrix(&objective_function, &result);
    let inverse_hessian = match invert_matrix(
        &hessian_matrix
            .iter()
            .map(|x| x.iter().map(|y| y / 2.0).collect())
            .collect::<Vec<_>>(),
    ) {
        Some(inverse) => inverse,
        None => {
            eprintln!("Matriz Hessiana sin inversa, no pudieron calcularse los errores");
            vec![vec![0.0; n]; n]
        }
    };

    let rss = objective_function(&result);
    let dof = (n - result.len()) as f64;

    let covariance_matrix = inverse_hessian
        .iter()
        .map(|x| x.iter().map(|y| y * rss / dof).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    // let covariance_matrix = inverse_hessian;
    let errors: Vec<f64> = covariance_matrix
        .iter()
        .enumerate()
        .map(|(i, x)| x[i].sqrt())
        .collect();
    result
        .into_iter()
        .zip(errors)
        .map(|(v, e)| Measure::new(vec![v], vec![e], false).unwrap())
        .collect()
}

fn generate_initial_simplex(initial_point: &[f64], scale: f64) -> Vec<Vec<f64>> {
    let n = initial_point.len();
    let mut simplex = vec![initial_point.to_vec()];

    for i in 0..n {
        let mut point = initial_point.to_vec();
        point[i] += scale;
        simplex.push(point);
    }

    simplex
}

fn nelder_mead<F>(
    f: &F,
    initial_point: &[f64],
    max_iterations: Option<usize>,
    tol: f64,
    scale: f64,
) -> Vec<f64>
where
    F: Fn(&[f64]) -> f64,
{
    let initial_simplex = generate_initial_simplex(initial_point, scale);
    let n = initial_point.len();
    let mut simplex = initial_simplex.clone();
    let mut values: Vec<f64> = simplex.iter().map(|point| f(point)).collect();
    let iter: Box<dyn Iterator<Item = ()>>;
    if let Some(max) = max_iterations {
        iter = Box::new(std::iter::repeat(()).take(max));
    } else {
        iter = Box::new(std::iter::repeat(()));
    }
    for _ in iter {
        // Sort simplex vertices by function values
        let mut indices: Vec<usize> = (0..values.len()).collect();
        indices.sort_by(|&a, &b| values[a].partial_cmp(&values[b]).unwrap());

        let mut centroid = vec![0.0; n];
        for &i in &indices[0..n] {
            centroid
                .iter_mut()
                .zip(&simplex[i])
                .for_each(|(c, &s)| *c += s);
        }
        centroid.iter_mut().for_each(|c| *c /= n as f64);

        // Reflection
        let reflection: Vec<f64> = centroid
            .iter()
            .zip(&simplex[indices[n]])
            .map(|(c, s)| 2.0 * c - s)
            .collect();
        let reflection_value = f(&reflection);

        if reflection_value < values[indices[0]] && reflection_value >= values[indices[n - 1]] {
            simplex[indices[n]] = reflection.clone();
            values[indices[n]] = reflection_value;
        } else if reflection_value < values[indices[n - 1]] {
            // Expansion
            let expansion: Vec<f64> = centroid
                .iter()
                .zip(&reflection)
                .map(|(c, r)| centroid[0] + 2.0 * (r - c))
                .collect();
            let expansion_value = f(&expansion);

            if expansion_value < reflection_value {
                simplex[indices[n]] = expansion;
                values[indices[n]] = expansion_value;
            } else {
                simplex[indices[n]] = reflection.clone();
                values[indices[n]] = reflection_value;
            }
        } else {
            // Contraction
            let contraction: Vec<f64> = centroid
                .iter()
                .zip(&simplex[indices[n]])
                .map(|(c, s)| 0.5 * (s + c))
                .collect();
            let contraction_value = f(&contraction);

            if contraction_value < values[indices[n]] {
                simplex[indices[n]] = contraction;
                values[indices[n]] = contraction_value;
            } else {
                // Shrink
                for i in 1..=n {
                    simplex[indices[i]] = simplex[indices[0]]
                        .iter()
                        .zip(&simplex[indices[i]])
                        .map(|(s0, si)| 0.5 * (s0 + si))
                        .collect();
                    values[indices[i]] = f(&simplex[indices[i]]);
                }
            }
        }

        // Check convergence
        let max_diff = (values[indices[0]] - values[indices[n]]).abs();
        if max_diff < tol {
            break;
        }
    }

    simplex[0].clone()
}

fn calculate_hessian_matrix<F>(objective_function: &F, params: &[f64]) -> Vec<Vec<f64>>
where
    F: Fn(&[f64]) -> f64,
{
    let n = params.len();
    let h = 1e-6 * params.iter().fold(f64::INFINITY, |a, &b| a.min(b));

    let mut hessian_matrix = vec![vec![0.0; n]; n];

    for i in 0..n {
        let mut params1 = params.to_vec();
        let mut params2 = params.to_vec();

        params1[i] += h;
        params2[i] -= h;

        let gradient1 = calculate_gradient(&objective_function, &params1);
        let gradient2 = calculate_gradient(&objective_function, &params2);

        for j in 0..n {
            hessian_matrix[i][j] = (gradient1[j] - gradient2[j]) / (2.0 * h);
        }
    }

    hessian_matrix
}

fn calculate_gradient<F>(objective_function: &F, params: &[f64]) -> Vec<f64>
where
    F: Fn(&[f64]) -> f64,
{
    let n = params.len();
    let h = 1e-6 * params.iter().fold(f64::INFINITY, |a, &b| a.min(b));

    let mut gradient = Vec::with_capacity(n);

    for i in 0..n {
        let mut params1 = params.to_vec();
        let mut params2 = params.to_vec();

        params1[i] += h;
        params2[i] -= h;

        let f1 = objective_function(&params1);
        let f2 = objective_function(&params2);

        gradient.push((f1 - f2) / (2.0 * h));
    }

    gradient
}

fn invert_matrix(matrix: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
    let n = matrix.len();

    // Check if the matrix is square
    if matrix.iter().any(|row| row.len() != n) {
        return None; // Not a square matrix
    }

    // Create an augmented matrix with the identity matrix
    let mut augmented_matrix: Vec<Vec<f64>> = matrix
        .iter()
        .cloned()
        .zip((0..n).map(|i| {
            (0..n)
                .map(|j| if i == j { 1.0 } else { 0.0 })
                .collect::<Vec<f64>>()
        }))
        .map(|(mut row, identity_row)| {
            row.extend(identity_row);
            row
        })
        .collect();

    // Perform Gauss-Jordan elimination
    for i in 0..n {
        // Find pivot for column i
        if let Some(pivot_row) = (i..n).max_by(|&row1, &row2| {
            matrix[row1][i]
                .abs()
                .partial_cmp(&matrix[row2][i].abs())
                .unwrap()
        }) {
            augmented_matrix.swap(i, pivot_row);

            // Make the diagonal element 1
            let pivot_val = augmented_matrix[i][i];
            if pivot_val == 0.0 {
                return None; // Matrix is singular
            }
            for j in 0..2 * n {
                augmented_matrix[i][j] /= pivot_val;
            }

            // Make other elements in the column 0
            for k in 0..n {
                if k != i {
                    let factor = augmented_matrix[k][i];
                    for j in i..2 * n {
                        augmented_matrix[k][j] -= factor * augmented_matrix[i][j];
                    }
                }
            }
        } else {
            return None; // Matrix is singular
        }
    }

    // Extract the inverse matrix from the augmented matrix
    let inverse_matrix: Vec<Vec<f64>> = augmented_matrix
        .iter()
        .map(|row| row[n..].to_vec())
        .collect();

    Some(inverse_matrix)
}
