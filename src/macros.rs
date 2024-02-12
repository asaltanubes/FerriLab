/// The measure macro provides a very intuitive way for creating new measures.
///
/// It has five primary expresions available:
/// - Vector of values and vector of errors, one for each value.
/// - Vector of values and one error for all values.
/// - One value along with his error.
/// - Only a vector of values without errors.
/// - Tuples containing (value, error).
///
/// All this formats accept optionally an bool parameter, indicating whether or
/// not you want to aproximate the measure to the first significative
/// figure of the error, being set to true by default.
///
/// # Examples
///
/// ```rust
/// # use ferrilab::{measure, Measure};
/// let measure1 = measure!([1, 2, 3], [0.1, 0.2, 0.3]; false);
/// let measure2 = measure!([1, 2, 3],  0.3);
/// let measure3 = measure!(1, 0.3; true);
/// let measure4 = measure!([1, 2, 3]);
/// let measure5 = measure!((1, 0.1), (2, 0.2), (3, 0.3));
/// ```
///
/// The error is set to 0 when no error is given. It is important that vectors of values and
/// errors are the same length, otherwise it will cause an error.

#[macro_export]
macro_rules! measure {
    // value: [...], error: _, aprox: true/false/nothing
    ( [$( $val:expr ),+] $(; $aprox:literal)?) => {
        {
            let value = vec![$($val as f64,)+];
            let mut _aprox = true;
            $ ( _aprox = $aprox;)?

            match Measure::new(vec![$($val as f64,)+], vec![0.0; value.len()], _aprox) {
                Ok(measure) => measure,
                Err(e) => panic!("{}", e)
            }
        }
    };
    // value: [...], error, aprox: true/false/nothing
    ( [$( $val:expr),+], $err:literal $(; $aprox:literal)?) => {
        {
            let value = vec![$($val as f64,)+];
            let mut _aprox = true;
            $ ( _aprox = $aprox;)?
            match Measure::new(vec![$($val as f64,)+], vec![$err as f64; value.len()], _aprox) {
                Ok(measure) => measure,
                Err(e) => panic!("{}", e)
            }
        }
    };
    // value: [...], error: [...], aprox: true/false/nothing
    ( [$( $val:expr),+] , [$( $err:expr ),+] $(; $aprox:literal)?) => {
        {
            let mut _aprox = true;
                $ ( _aprox = $aprox;)?

            match Measure::new(vec![$($val as f64,)+], vec![$($err as f64,)+], _aprox) {
                Ok(measure) => measure,
                Err(e) => panic!("{}", e)
            }
        }
    };
    // value, error, aprox: true/false/nothing
    ( $val:literal , $err:literal $(; $aprox:literal)?) => {
        {
            let mut _aprox = true;
                $ ( _aprox = $aprox;)?

            match Measure::new(vec![$val as f64], vec![$err as f64], _aprox) {
                Ok(measure) => measure,
                Err(e) => panic!("{}", e)
            }
        }
    };
    // (value, error)..., aprox: true/false/nothing
    ( $( ($val:expr, $err:expr) ),+ $(; $aprox:literal)?) => {
        {
            let mut _aprox = true;
             $ ( _aprox = $aprox;)?

            match Measure::new(vec![$($val as f64,)+], vec![$($err as f64,)+], _aprox) {
                Ok(measure) => measure,
                Err(e) => panic!("{}", e)
            }
        }
    };
}

/// Internal macro to implement operations traits between measures.
#[doc(hidden)]
#[macro_export]
macro_rules! impl_op {
    ($from:ty, $for:ty) => {
        impl Add<$from> for $for {
            type Output = Measure;

            fn add(self, other: $from) -> Self::Output {
                if self.len() == 1 {
                    return Measure {
                        value: other
                            .value
                            .iter()
                            .map(|oval| self.value[0] + oval)
                            .collect(),
                        error: other
                            .error
                            .iter()
                            .map(|oerr| (self.error[0].powi(2) + oerr.powi(2)).sqrt())
                            .collect(),
                        style: Style::PM,
                    };
                }
                if other.len() == 1 {
                    return Measure {
                        value: self
                            .value
                            .iter()
                            .map(|sval| sval + other.value[0])
                            .collect(),
                        error: self
                            .error
                            .iter()
                            .map(|serr| (serr.powi(2) + other.error[0].powi(2)).sqrt())
                            .collect(),
                        style: Style::PM,
                    };
                }

                assert_eq!(
                    self.len(),
                    other.len(),
                    "Measures lengths must be equals, obtained {} and {}.",
                    self.len(),
                    other.len()
                );
                Measure {
                    value: self
                        .value
                        .iter()
                        .zip(other.value.iter())
                        .map(|(sval, oval)| sval + oval)
                        .collect(),
                    error: self
                        .error
                        .iter()
                        .zip(other.error.iter())
                        .map(|(serr, oerr)| (serr.powi(2) + oerr.powi(2)).sqrt())
                        .collect(),
                    style: Style::PM,
                }
            }
        }

        impl Sub<$from> for $for {
            type Output = Measure;

            fn sub(self, other: $from) -> Self::Output {
                if self.len() == 1 {
                    return Measure {
                        value: other
                            .value
                            .iter()
                            .map(|oval| self.value[0] - oval)
                            .collect(),
                        error: other
                            .error
                            .iter()
                            .map(|oerr| (self.error[0].powi(2) + oerr.powi(2)).sqrt())
                            .collect(),
                        style: Style::PM,
                    };
                }
                if other.len() == 1 {
                    return Measure {
                        value: self
                            .value
                            .iter()
                            .map(|sval| sval - other.value[0])
                            .collect(),
                        error: self
                            .error
                            .iter()
                            .map(|serr| (serr.powi(2) + other.error[0].powi(2)).sqrt())
                            .collect(),
                        style: Style::PM,
                    };
                }

                assert_eq!(
                    self.len(),
                    other.len(),
                    "Measures lengths must be equals, obtained {} and {}.",
                    self.len(),
                    other.len()
                );
                Measure {
                    value: self
                        .value
                        .iter()
                        .zip(other.value.iter())
                        .map(|(sval, oval)| sval - oval)
                        .collect(),
                    error: self
                        .error
                        .iter()
                        .zip(other.error.iter())
                        .map(|(serr, oerr)| (serr.powi(2) + oerr.powi(2)).sqrt())
                        .collect(),
                    style: Style::PM,
                }
            }
        }

        impl Mul<$from> for $for {
            type Output = Measure;

            fn mul(self, other: $from) -> Self::Output {
                if self.len() == 1 {
                    return Measure {
                        value: other
                            .value
                            .iter()
                            .map(|oval| self.value[0] * oval)
                            .collect(),
                        error: other
                            .iter()
                            .map(|(oval, oerr)| {
                                ((oval * self.error[0]).powi(2) + (oerr * self.value[0]).powi(2))
                                    .sqrt()
                            })
                            .collect(),
                        style: Style::PM,
                    };
                }
                if other.len() == 1 {
                    return Measure {
                        value: self
                            .value
                            .iter()
                            .map(|sval| sval * other.value[0])
                            .collect(),
                        error: self
                            .iter()
                            .map(|(sval, serr)| {
                                ((other.value[0] * serr).powi(2) + (sval * other.error[0]).powi(2))
                                    .sqrt()
                            })
                            .collect(),
                        style: Style::PM,
                    };
                }

                assert_eq!(
                    self.len(),
                    other.len(),
                    "Measures lengths must be equals, obtained {} and {}.",
                    self.len(),
                    other.len()
                );
                Measure {
                    value: self
                        .value
                        .iter()
                        .zip(other.value.iter())
                        .map(|(sval, oval)| sval * oval)
                        .collect(),
                    error: self
                        .iter()
                        .zip(other.iter())
                        .map(|((sval, serr), (oval, oerr))| {
                            ((oval * serr).powi(2) + (sval * oerr).powi(2)).sqrt()
                        })
                        .collect(),
                    style: Style::PM,
                }
            }
        }

        impl Div<$from> for $for {
            type Output = Measure;

            fn div(self, other: $from) -> Self::Output {
                if self.len() == 1 {
                    return Measure {
                        value: other
                            .value
                            .iter()
                            .map(|oval| self.value[0] / oval)
                            .collect(),
                        error: other
                            .iter()
                            .map(|(oval, oerr)| {
                                ((1.0 / oval * self.error[0]).powi(2)
                                    + (self.value[0] / oval.powi(2) * oerr.powi(2)))
                                .sqrt()
                            })
                            .collect(),
                        style: Style::PM,
                    };
                }
                if other.len() == 1 {
                    return Measure {
                        value: self
                            .value
                            .iter()
                            .map(|sval| sval / other.value[0])
                            .collect(),
                        error: self
                            .iter()
                            .map(|(sval, serr)| {
                                ((1.0 / other.value[0] * serr).powi(2)
                                    + (sval / other.value[0].powi(2) * other.error[0].powi(2)))
                                .sqrt()
                            })
                            .collect(),
                        style: Style::PM,
                    };
                }

                assert_eq!(
                    self.len(),
                    other.len(),
                    "Measures lengths must be equals, obtained {} and {}.",
                    self.len(),
                    other.len()
                );
                Measure {
                    value: self
                        .value
                        .iter()
                        .zip(other.value.iter())
                        .map(|(sval, oval)| sval / oval)
                        .collect(),
                    error: self
                        .iter()
                        .zip(other.iter())
                        .map(|((sval, serr), (oval, oerr))| {
                            ((1.0 / oval * serr).powi(2) + (sval / oval.powi(2) * oerr.powi(2)))
                                .sqrt()
                        })
                        .collect(),
                    style: Style::PM,
                }
            }
        }
    };
}

/// Internal macro to implement operations traits between every type of number
/// and a measure.
#[doc(hidden)]
#[macro_export]
macro_rules! impl_op_number {
    ($for:ty) => {
        impl<T: std::convert::Into<f64>> Add<T> for $for {
            type Output = Measure;

            fn add(self, num: T) -> Self::Output {
                let num = num.into();
                Measure {
                    value: self.value.iter().map(|val| val + num).collect(),
                    error: self.error.clone(),
                    style: Style::PM,
                }
            }
        }

        impl<T: std::convert::Into<f64>> Sub<T> for $for {
            type Output = Measure;

            fn sub(self, num: T) -> Self::Output {
                let num = num.into();
                Measure {
                    value: self.value.iter().map(|val| val - num).collect(),
                    error: self.error.clone(),
                    style: Style::PM,
                }
            }
        }

        impl<T: std::convert::Into<f64>> Mul<T> for $for {
            type Output = Measure;

            fn mul(self, num: T) -> Self::Output {
                let num = num.into();
                Measure {
                    value: self.value.iter().map(|val| val * num).collect(),
                    error: self.error.iter().map(|err| err * num.abs()).collect(),
                    style: Style::PM,
                }
            }
        }

        impl<T: std::convert::Into<f64>> Div<T> for $for {
            type Output = Measure;

            fn div(self, num: T) -> Self::Output {
                let num = num.into();
                Measure {
                    value: self.value.iter().map(|val| val / num).collect(),
                    error: self.error.iter().map(|err| err / num.abs()).collect(),
                    style: Style::PM,
                }
            }
        }
    };

    ($from:ty, $for:ty) => {
        impl Add<$from> for $for {
            type Output = Measure;

            fn add(self, measure: $from) -> Self::Output {
                Measure {
                    value: measure
                        .value
                        .iter()
                        .map(|val| val + (self as f64))
                        .collect(),
                    error: measure.error.clone(),
                    style: Style::PM,
                }
            }
        }

        impl Sub<$from> for $for {
            type Output = Measure;

            fn sub(self, measure: $from) -> Self::Output {
                Measure {
                    value: measure
                        .value
                        .iter()
                        .map(|val| val - (self as f64))
                        .collect(),
                    error: measure.error.clone(),
                    style: Style::PM,
                }
            }
        }

        impl Mul<$from> for $for {
            type Output = Measure;

            fn mul(self, measure: $from) -> Self::Output {
                Measure {
                    value: measure
                        .value
                        .iter()
                        .map(|val| val * (self as f64))
                        .collect(),
                    error: measure
                        .error
                        .iter()
                        .map(|err| err * (self as f64).abs())
                        .collect(),
                    style: Style::PM,
                }
            }
        }

        impl Div<$from> for $for {
            type Output = Measure;

            fn div(self, measure: $from) -> Self::Output {
                Measure {
                    value: measure
                        .value
                        .iter()
                        .map(|val| (self as f64) / val)
                        .collect(),
                    error: measure
                        .iter()
                        .map(|(val, err)| (self as f64).abs() * err / val.powi(2))
                        .collect(),
                    style: Style::PM,
                }
            }
        }
    };
}
