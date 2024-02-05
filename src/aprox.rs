/// Rounds a value to the decimals indicated.
pub fn round(value: f64, decimal_places: i32) -> f64 {
    let multiplier = 10.0_f64.powi(decimal_places);
    (value * multiplier - 0.5).ceil() / multiplier
}

fn trucate(value: f64, decimal_places: i32) -> f64 {
    let multiplier = 10.0_f64.powi(decimal_places);
    (value * multiplier).trunc() / multiplier
}

/// Aproximate the value to the first significant figure of the error.
pub fn aprox(value: f64, error: f64) -> (f64, f64) {
    if value.is_finite() && error.is_finite() && error != 0. {
        let mut first_sigificative_figure = -(error.abs().log10().floor() as i32);
        let new_error = trucate(error, first_sigificative_figure);
        // The first significative figure of the error is 1.
        if new_error.log10() == new_error.log10().floor()
            && round(error, first_sigificative_figure) == 10.0_f64.powi(-first_sigificative_figure)
        {
            first_sigificative_figure += 1;
        }
        return (
            round(value, first_sigificative_figure),
            round(error, first_sigificative_figure),
        );
    }
    if error == 0. || error.is_nan() {
        return (value, error);
    }
    if value.is_nan() {
        return (value, aprox(1., error).1);
    }
    if error.is_infinite() {
        return (0., error);
    }
    if value.is_infinite() {
        return (value, aprox(1., error).1);
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn round_test() {
        assert_eq!(round(8.7, 0), 9.0);
        assert_eq!(round(3.2, 0), 3.0);
        assert_eq!(round(-1.2, 0), -1.0);
        assert_eq!(round(-1.5, 0), -2.0);
        assert_eq!(round(-1.49, 0), -1.0);
        assert_eq!(round(1.51, 0), 2.0);

        assert_eq!(round(1.9256, 1), 1.9);
        assert_eq!(round(1.9256, 2), 1.93);
        assert_eq!(round(1.9256, 3), 1.926);
        assert_eq!(round(1.9256, 4), 1.9256);
    }

    #[test]
    fn aprox_test() {
        assert_eq!(aprox(10.05, 0.1), (10.05, 0.1));
        assert_eq!(aprox(10.14, 0.22), (10.1, 0.2));
        assert_eq!(aprox(10.14, 0.15), (10.14, 0.15));
        assert_eq!(aprox(10.14, 0.151), (10.1, 0.2));
    }
}
