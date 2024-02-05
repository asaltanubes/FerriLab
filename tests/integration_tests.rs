use ferrilab::{measure, CurveFit, LinearFit, Measure};

#[test]
fn macro_test() {
    assert_eq!(
        measure!([1, 2, 3, 4]; true),
        Measure::new(vec![1., 2., 3., 4.], vec![0.; 4], true).unwrap()
    );

    assert_eq!(
        measure!([1, 2, 3.0, 4], [1, 2, 3, 4]),
        Measure::new(vec![1., 2., 3., 4.], vec![1., 2., 3., 4.], true).unwrap()
    );

    assert_eq!(
        measure!((1, 2.1), (2, 3)),
        Measure::new(vec![1., 2.], vec![2.1, 3.], true).unwrap()
    );

    assert_eq!(
        measure!([1, 2, 3, 4e-12], 1; false),
        Measure::new(vec![1., 2., 3., 4.0e-12], vec![1.; 4], false).unwrap()
    );

    assert_eq!(
        measure!(1, 2; false),
        Measure::new(vec![1.], vec![2.], false).unwrap()
    );
}

#[test]
fn unpack_test() {
    assert_eq!(
        Measure::new(vec![1.], vec![2.], false).unwrap().unpack(),
        (&vec![1.], &vec![2.])
    )
}

#[test]
fn get_and_set() {
    let mut measure1 = Measure::new(vec![1., 2., 3.], vec![2.; 3], false).unwrap();

    let mut measure2 = Measure::new(vec![4., 5., 6.], vec![1.; 3], false).unwrap();

    measure2.set(2, (0., 0.));

    measure1.set_value(0, 15.);

    measure1.set_error(1, 0.);

    assert_eq!(measure1.get(2), Some((&3., &2.)));
    assert_eq!(
        measure2,
        Measure::new(vec![4., 5., 0.], vec![1., 1., 0.], false).unwrap()
    );
    assert_eq!(
        measure1,
        Measure::new(vec![15., 2., 3.], vec![2., 0., 2.], false).unwrap()
    );
}

#[test]
fn operations() {
    let x = measure!([1, 2, 3, 4], [0.1, 0.2, 0.3, 0.4]);
    let y = measure!([0.5, 0.6, 0.7, 0.8], [0.05, 0.06, 0.07, 0.08]);

    assert_eq!(
        x.pow(2).aprox(),
        measure!([1, 4, 9, 16], [0.2, 0.8, 1.8, 3.2])
    );

    assert_eq!(
        x.sqrt().aprox(),
        measure!([1.0, 1.41, 1.73, 2.0], [0.05, 0.07, 0.09, 0.1])
    );

    assert_eq!(
        x.sin().aprox(),
        measure!([0.84, 0.91, 0.1, -0.8], [0.05, 0.08, 0.3, 0.3])
    );

    assert_eq!(
        x.cos().aprox(),
        measure!([0.54, -0.4, -0.99, -0.7], [0.08, 0.2, 0.04, 0.3])
    );

    assert_eq!(
        x.tan().aprox(),
        measure!([1.6, -2.2, -0.1, 1.2], [0.3, 1.2, 0.3, 0.9])
    );

    assert_eq!(
        y.asin().aprox(),
        measure!([0.52, 0.64, 0.78, 0.93], [0.06, 0.07, 0.1, 0.13])
    );

    assert_eq!(
        y.acos().aprox(),
        measure!([1.05, 0.93, 0.8, 0.64], [0.06, 0.07, 0.1, 0.13])
    );

    assert_eq!(
        x.atan().aprox(),
        measure!([0.79, 1.11, 1.25, 1.33], [0.05, 0.04, 0.03, 0.02])
    );

    assert_eq!(
        x.atan2(&y).aprox(),
        measure!([1.107, 1.279, 1.342, 1.373], [0.003, 0.005, 0.007, 0.009])
    );

    assert_eq!(
        x.ln().aprox(),
        measure!([0.0, 0.69, 1.1, 1.39], [0.1, 0.1, 0.1, 0.1])
    );

    assert_eq!(
        x.exp().aprox(),
        measure!([2.72, 7.4, 20.1, 55.0], [0.1, 0.4, 0.9, 2.0])
    );

    assert_eq!(
        (&x + &y).aprox(),
        measure!([1.5, 2.6, 3.7, 4.8], [0.11, 0.2, 0.3, 0.4])
    );

    assert_eq!(
        (&x - &y).aprox(),
        measure!([0.5, 1.4, 2.3, 3.2], [0.11, 0.2, 0.3, 0.4])
    );

    assert_eq!(
        (&x * &y).aprox(),
        measure!([0.5, 1.2, 2.1, 3.2], [0.07, 0.2, 0.3, 0.5])
    );

    assert_eq!(
        (&x / &y).aprox(),
        measure!([2.0, 3.3, 4.3, 5.0], [0.2, 0.4, 0.5, 0.5])
    );
}

#[test]

fn fit_test() {
    assert_eq!(
        LinearFit::new([0.7, 1.8, 2.7, 4.3], [4.6, 5.4, 6.9, 8.1]).fit(),
        (
            measure!(1.0111550917596268, 0.1158958350259736; false),
            measure!(3.8485066570708875, 0.31479109479486966; false)
        )
    );
    assert_eq!(
        LinearFit::new([0.7, 1.8, 2.7, 4.3], [4.6, 5.4, 6.9, 8.1],)
            .y_error(vec![0.1, 0.3, 0.4, 0.7])
            .fit(),
        (
            measure!(0.9963598861989915, 0.13329751086990216; false),
            measure!(3.8896028985935134, 0.15825404095614476; false)
        )
    );
    assert_eq!(
        CurveFit::new(
            |x, coefs| coefs[0] * (-x * coefs[1]).exp(),
            [0.042, 0.2, 0.33, 0.6],
            [1.6, 1.25, 0.8, 0.34]
        )
        .initial_zeros(2)
        .fit(),
        vec![
            measure!(1.8368313871324062, 0.1339378128643651; false),
            measure!(2.4591460197698325, 0.35963907104421394; false)
        ]
    )
}
