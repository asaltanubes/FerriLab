//! Contains the struct Measure and all its methods and traits implementations.
use {
    crate::{
        aprox::{aprox, round},
        impl_op, impl_op_number,
    },
    std::{
        f64::consts::PI,
        fmt::Display,
        ops::{Add, Div, Mul, Sub},
    },
};

/// Essential object to store and manage measures.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Measure {
    value: Vec<f64>,
    error: Vec<f64>,
    style: Style,
}

/// Diferent style types for print measures.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Style {
    /// \[values\] ± \[errors\]
    List,
    /// value ± error, ...
    PM,
    /// value ± error
    Table,
    /// $value \pm error$
    LatexTable,
    /// $value plus.minus error$
    TypstTable,
}

#[doc(hidden)]
#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("You're only allowed to assign either one error for all values or one error for each value.")]
    InvalidErrorLen,
}

impl Measure {
    /// Constructor of the struct Measure.
    pub fn new(
        mut value: Vec<f64>,
        mut error: Vec<f64>,
        aproximate: bool,
    ) -> Result<Measure, MyError> {
        if value.len() != error.len() && error.len() != 1 {
            return Err(MyError::InvalidErrorLen);
        } else if error.len() == 1 {
            error = vec![error[0]; value.len()];
        }

        if aproximate {
            let tuples: Vec<(f64, f64)> = value
                .iter()
                .zip(error.iter())
                .map(|(val, err)| aprox(*val, *err))
                .collect();

            value = tuples.iter().map(|(val, _)| *val).collect();

            error = tuples.into_iter().map(|(_, err)| err).collect();
        }
        Ok(Measure {
            value,
            error,
            style: Style::PM,
        })
    }
    /// Length of the measure.
    pub fn len(&self) -> usize {
        self.value.len()
    }
    /// Checks if the measure is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Returns the values of a measure.
    pub fn value(&self) -> &Vec<f64> {
        &self.value
    }
    /// Returns the error of a measure.
    pub fn error(&self) -> &Vec<f64> {
        &self.error
    }
    /// Returns the style of a measure.
    pub fn style(&self) -> &Style {
        &self.style
    }
    /// Changes the style of a measure.
    pub fn change_style(self, style: Style) -> Measure {
        Measure {
            value: self.value,
            error: self.error,
            style,
        }
    }
    /// Returns a tuple (values, error)
    pub fn unpack(&self) -> (&Vec<f64>, &Vec<f64>) {
        (&self.value, &self.error)
    }
    /// Returns a vector of measures of length 1.
    pub fn list_of_measures(&self) -> Vec<Measure> {
        self.iter()
            .map(|(value, error)| Measure {
                value: vec![*value],
                error: vec![*error],
                style: Style::PM,
            })
            .collect()
    }
    /// Iterates over a measure without taking ownership as a tuple (value, error).
    pub fn iter(&self) -> MeasureIter {
        self.value.iter().zip(self.error.iter())
    }
    /// Iterates over a measure as a tuple (value, error) that allows modifying the values.
    pub fn iter_mut(&mut self) -> MeasureIterMut {
        self.value.iter_mut().zip(self.error.iter_mut())
    }
    /// Returns the value and error of a certain index.
    pub fn get(&self, index: usize) -> Option<(&f64, &f64)> {
        Some((self.value.get(index)?, self.error.get(index)?))
    }
    /// Modify the value and error of a certain index.
    pub fn set<T, U>(&mut self, index: usize, measure: (T, U))
    where
        T: std::convert::Into<f64>,
        U: std::convert::Into<f64>,
    {
        self.value[index] = measure.0.into();
        self.error[index] = measure.1.into();
    }
    /// Modify the value of a certain index.
    pub fn set_value<T: std::convert::Into<f64>>(&mut self, index: usize, value: T) {
        self.value[index] = value.into();
    }
    /// Modify the error of a certain index.
    pub fn set_error<T: std::convert::Into<f64>>(&mut self, index: usize, error: T) {
        self.error[index] = error.into();
    }

    // -------------- Operations ----------------

    /// Aproximate the measure to the first significative figure of the error.
    pub fn aprox(mut self) -> Self {
        let tuples: Vec<(f64, f64)> = self.iter().map(|(val, err)| aprox(*val, *err)).collect();

        self.value = tuples.iter().map(|(val, _)| *val).collect();

        self.error = tuples.into_iter().map(|(_, err)| err).collect();

        self
    }
    /// Aproximate the measure to the decimals indicated.
    pub fn aprox_to(mut self, decimals: i32) -> Self {
        self.value = self.value.iter().map(|val| round(*val, decimals)).collect();

        self.error = self.error.iter().map(|err| round(*err, decimals)).collect();

        self
    }
    /// Calculates the mean of a measure.
    pub fn mean(&self) -> f64 {
        self.value.iter().sum::<f64>() / (self.len() as f64)
    }
    /// Calculates the standard desviation of a measure.
    pub fn standard_deviation(&self) -> f64 {
        (self
            .value
            .iter()
            .map(|val| (val - self.mean()).powi(2))
            .sum::<f64>()
            / (self.len() as f64 - 1.0))
            .sqrt()
    }
    /// Calculates the standard error of a measure.
    pub fn standard_error(&self) -> f64 {
        self.standard_deviation() / (self.len() as f64).sqrt()
    }
    /// Calculates an estimation of a measure.
    pub fn estimation(&self) -> Measure {
        Measure {
            value: vec![self.mean(); self.len()],
            error: self
                .error
                .iter()
                .map(|err| (self.standard_error().powi(2) + err.powi(2)).sqrt())
                .collect(),
            style: Style::PM,
        }
    }
    /// Raises a measure to any number.
    pub fn pow<T: std::convert::Into<f64>>(&self, other: T) -> Measure {
        let other = other.into();
        Measure {
            value: self.value.iter().map(|val| val.powf(other)).collect(),
            error: self
                .iter()
                .map(|(val, err)| (other * val.powf(other - 1.0) * err).abs())
                .collect(),
            style: Style::PM,
        }
    }
    /// Converts grades in radians.
    pub fn rad(&self) -> Measure {
        Measure {
            value: self.value.iter().map(|val| val * PI / 180.0).collect(),
            error: self.error.iter().map(|err| err * PI / 180.0).collect(),
            style: Style::PM,
        }
    }
    /// Converts radians in grades.
    pub fn grad(&self) -> Measure {
        Measure {
            value: self.value.iter().map(|val| val * 180.0 / PI).collect(),
            error: self.error.iter().map(|err| err * 180.0 / PI).collect(),
            style: Style::PM,
        }
    }
    /// Returns the square root of a measure.
    pub fn sqrt(&self) -> Measure {
        Measure {
            value: self.value.iter().map(|val| val.sqrt()).collect(),
            error: self
                .iter()
                .map(|(val, err)| err / (2.0 * val.sqrt()))
                .collect(),
            style: Style::PM,
        }
    }
    /// Computes the absolute value of a measure.
    pub fn abs(&self) -> Measure {
        Measure {
            value: self.value.clone().iter().map(|val| val.abs()).collect(),
            error: self.error.clone(),
            style: Style::PM,
        }
    }
    /// Computes the sine of a measure in radians.
    pub fn sin(&self) -> Measure {
        let value: Vec<f64> = self.value.iter().map(|val| val.sin()).collect();
        let error = self
            .iter()
            .zip(value.iter())
            .map(|((sval, serr), value)| {
                if *value == 1.0 || *value == -1.0 {
                    ((sval + serr).sin() - sval.sin()).abs()
                } else {
                    (sval.cos() * serr).abs()
                }
            })
            .collect();

        Measure {
            value,
            error,
            style: Style::PM,
        }
    }
    /// Computes the cosine of a measure in radians.
    pub fn cos(&self) -> Measure {
        let value: Vec<f64> = self.value.iter().map(|val| val.cos()).collect();
        let error: Vec<f64> = self
            .iter()
            .zip(value.iter())
            .map(|((sval, serr), value)| {
                if *value == 1.0 || *value == -1.0 {
                    ((sval + serr).cos() - sval.cos()).abs()
                } else {
                    (sval.sin() * serr).abs()
                }
            })
            .collect();

        Measure {
            value,
            error,
            style: Style::PM,
        }
    }
    /// Computes the tangent of a measure in radians.
    pub fn tan(&self) -> Measure {
        let value: Vec<f64> = self.value.iter().map(|val| val.tan()).collect();
        let error: Vec<f64> = self
            .error
            .iter()
            .zip(value.iter())
            .map(|(serr, value)| (1.0 + value.powi(2)) * serr)
            .collect();

        Measure {
            value,
            error,
            style: Style::PM,
        }
    }
    /// Computes the arcsine of a measure in radians.
    pub fn asin(&self) -> Measure {
        let value: Vec<f64> = self.value.iter().map(|val| val.asin()).collect();
        let error: Vec<f64> = self
            .iter()
            .map(|(val, err)| {
                if *val != 1.0 && *val != -1.0 {
                    err / (1.0 - val.powi(2)).sqrt()
                } else {
                    ((val - err).asin() - val).abs()
                }
            })
            .collect();

        Measure {
            value,
            error,
            style: Style::PM,
        }
    }
    /// Computes the arccosine of a measure in radians.
    pub fn acos(&self) -> Measure {
        let value: Vec<f64> = self.value.iter().map(|val| val.acos()).collect();
        let error: Vec<f64> = self
            .iter()
            .map(|(val, err)| {
                if *val != 1.0 && *val != -1.0 {
                    err / (1.0 - val.powi(2)).sqrt()
                } else {
                    let d = if *val > 0.0 { val - err } else { val + err };

                    (d.acos() - val.acos()).abs()
                }
            })
            .collect();

        Measure {
            value,
            error,
            style: Style::PM,
        }
    }
    /// Computes the arctangent of a measure in radians.
    pub fn atan(&self) -> Measure {
        let value: Vec<f64> = self.value.iter().map(|val| val.atan()).collect();
        let error: Vec<f64> = self
            .iter()
            .map(|(val, err)| err / (1.0 + val.powi(2)))
            .collect();

        Measure {
            value,
            error,
            style: Style::PM,
        }
    }
    /// Computes the four quadrant arctangent of two measures.
    pub fn atan2(&self, other: &Measure) -> Measure {
        let value: Vec<f64> = self
            .value
            .iter()
            .zip(other.value.iter())
            .map(|(sval, oval)| sval.atan2(*oval))
            .collect();
        let error: Vec<f64> = self
            .iter()
            .zip(other.iter())
            .map(|((sval, serr), (oval, oerr))| {
                ((sval.powi(2) * oerr.powi(2)).powi(2) + (oval.powi(2) * serr.powi(2)).powi(2))
                    .sqrt()
                    / (sval.powi(2) + oval.powi(2))
            })
            .collect();

        Measure {
            value,
            error,
            style: Style::PM,
        }
    }
    /// Returns the natural logarithm of a measure.
    pub fn ln(&self) -> Measure {
        let value: Vec<f64> = self.value.iter().map(|val| val.ln()).collect();
        let error: Vec<f64> = self
            .iter()
            .map(|(val, err)| (1.0 / val).abs() * err)
            .collect();

        Measure {
            value,
            error,
            style: Style::PM,
        }
    }
    /// Returns the exponential function of a measure.
    pub fn exp(&self) -> Measure {
        let value: Vec<f64> = self.value.iter().map(|val| val.exp()).collect();
        let error: Vec<f64> = self.iter().map(|(val, err)| val.abs() * err).collect();

        Measure {
            value,
            error,
            style: Style::PM,
        }
    }
    /// Returns the diference between a value and the next one in a measure.
    pub fn delta(&self) -> Measure {
        self.iter()
            .zip(self.iter().skip(1))
            .map(|((val, err), (next_val, next_err))| {
                (next_val - val, (err.powi(2) + next_err.powi(2)).sqrt())
            })
            .collect()
    }
}

impl Style {
    /// Changes how a measure is displayed depending on its style.
    pub fn disp(&self, measure: &Measure, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Style::List => write!(f, "{:?} ± {:?}", measure.value, measure.error),

            Style::PM => {
                for i in 0..measure.len() - 1 {
                    write!(f, "{} ± {}, ", measure.value[i], measure.error[i])?;
                }
                write!(
                    f,
                    "{} ± {}",
                    measure.value[measure.len() - 1],
                    measure.error[measure.len() - 1]
                )
            }

            Style::Table => {
                if measure.len() == 1 {
                    write!(f, "{} ± {}", measure.value[0], measure.error[0])
                } else {
                    write!(f, "This style is only for one value and its error.")
                }
            }

            Style::LatexTable => {
                if measure.len() == 1 {
                    write!(f, "${} \\pm {}$", measure.value[0], measure.error[0])
                } else {
                    write!(f, "This style is only for one value and its error.")
                }
            }

            Style::TypstTable => {
                if measure.len() == 1 {
                    write!(f, "${} plus.minus {}$", measure.value[0], measure.error[0])
                } else {
                    write!(f, "This style is only for one value and its error.")
                }
            }
        }
    }
}

impl From<Measure> for Vec<f64> {
    fn from(m: Measure) -> Vec<f64> {
        m.value
    }
}

impl From<Measure> for (Vec<f64>, Vec<f64>) {
    fn from(m: Measure) -> (Vec<f64>, Vec<f64>) {
        (m.value, m.error)
    }
}

impl From<&Measure> for Vec<f64> {
    fn from(m: &Measure) -> Vec<f64> {
        m.value.clone()
    }
}

impl From<&Measure> for (Vec<f64>, Vec<f64>) {
    fn from(m: &Measure) -> (Vec<f64>, Vec<f64>) {
        (m.value.clone(), m.error.clone())
    }
}

impl From<&mut Measure> for Vec<f64> {
    fn from(m: &mut Measure) -> Vec<f64> {
        m.value.clone()
    }
}

impl From<&mut Measure> for (Vec<f64>, Vec<f64>) {
    fn from(m: &mut Measure) -> (Vec<f64>, Vec<f64>) {
        (m.value.clone(), m.error.clone())
    }
}

impl Display for Measure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.style.disp(self, f)?;
        Ok(())
    }
}

type MeasureIntoIter = std::iter::Zip<std::vec::IntoIter<f64>, std::vec::IntoIter<f64>>;
impl IntoIterator for Measure {
    type Item = (f64, f64);
    type IntoIter = MeasureIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.value.into_iter().zip(self.error)
    }
}

type MeasureIter<'a> = std::iter::Zip<std::slice::Iter<'a, f64>, std::slice::Iter<'a, f64>>;
impl<'a> IntoIterator for &'a Measure {
    type Item = (&'a f64, &'a f64);
    type IntoIter = MeasureIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.iter().zip(self.error.iter())
    }
}

type MeasureIterMut<'a> =
    std::iter::Zip<std::slice::IterMut<'a, f64>, std::slice::IterMut<'a, f64>>;
impl<'a> IntoIterator for &'a mut Measure {
    type Item = (&'a mut f64, &'a mut f64);
    type IntoIter = MeasureIterMut<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.iter_mut().zip(self.error.iter_mut())
    }
}

impl<A, B> FromIterator<(A, B)> for Measure
where
    A: std::convert::Into<f64>,
    B: std::convert::Into<f64>,
{
    fn from_iter<T: IntoIterator<Item = (A, B)>>(iter: T) -> Self {
        let mut value = Vec::new();
        let mut error = Vec::new();
        iter.into_iter()
            .map(|(a, b)| (a.into(), b.into()))
            .for_each(|(a, b)| {
                value.push(a);
                error.push(b);
            });

        Measure {
            value,
            error,
            style: Style::PM,
        }
    }
}

// Implementing Add, Sub, Mul, Div:

// Between Measure - Measure:
impl_op!(Measure, Measure);
impl_op!(Measure, &Measure);
impl_op!(&Measure, Measure);
impl_op!(&Measure, &Measure);

// Between Measure - Number:
impl_op_number!(Measure);
impl_op_number!(&Measure);

// Between Number - Measure:
impl_op_number!(Measure, f32);
impl_op_number!(&Measure, f32);
impl_op_number!(Measure, f64);
impl_op_number!(&Measure, f64);
impl_op_number!(Measure, u8);
impl_op_number!(&Measure, u8);
impl_op_number!(Measure, i8);
impl_op_number!(&Measure, i8);
impl_op_number!(Measure, u16);
impl_op_number!(&Measure, u16);
impl_op_number!(Measure, i16);
impl_op_number!(&Measure, i16);
impl_op_number!(Measure, u32);
impl_op_number!(&Measure, u32);
impl_op_number!(Measure, i32);
impl_op_number!(&Measure, i32);
impl_op_number!(Measure, u64);
impl_op_number!(&Measure, u64);
impl_op_number!(Measure, i64);
impl_op_number!(&Measure, i64);
impl_op_number!(Measure, u128);
impl_op_number!(&Measure, u128);
impl_op_number!(Measure, i128);
impl_op_number!(&Measure, i128);
