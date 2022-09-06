use matrix::matrix::Matrix;

#[test]
fn matrix_determinant_example_1() {
    let matrix = Matrix::from([[1., -1.], [-1., 1.]]);
    assert_eq!(matrix.determinant(), 0.);
}

#[test]
fn matrix_determinant_example_2() {
    let matrix = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
    assert_eq!(matrix.determinant(), 8.);
}

#[test]
fn matrix_determinant_example_3() {
    let matrix = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
    assert_eq!(matrix.determinant(), -174.);
}

#[test]
fn matrix_determinant_example_4() {
    let matrix = Matrix::from([
        [8., 5., -2., 4.],
        [4., 2.5, 20., 4.],
        [8., 5., 1., 4.],
        [28., -4., 17., 1.],
    ]);
    assert_eq!(matrix.determinant(), 1032.);
}

#[test]
fn matrix_determinant_example_5() {
    let matrix = Matrix::from([
        [8., 5., -2., 4., 2.],
        [4., 2.5, 20., 4., 2.],
        [8., 5., 1., 4., 2.],
        [28., -4., 17., 1., 2.],
        [14., 8., 11., 9., 4.],
    ]);
    assert_eq!(matrix.determinant(), -570.);
}

#[test]
fn matrix_determinant_example_6() {
    let matrix = Matrix::from([
        [
            0.98227537, 0.6158931, 0.82725697, 0.99358585, 0.31210679, 0.53208718,
        ],
        [
            0.30188303, 0.9334737, 0.25611915, 0.48829487, 0.14969977, 0.8146814,
        ],
        [
            0.27845208, 0.37664383, 0.68020428, 0.29660365, 0.60204162, 0.92291283,
        ],
        [
            0.21552234, 0.49018346, 0.22021131, 0.32545267, 0.39536727, 0.12177177,
        ],
        [
            0.56677126, 0.35064048, 0.4436396, 0.08588831, 0.18767159, 0.16095085,
        ],
        [
            0.54082626, 0.7772932, 0.48200571, 0.17270892, 0.7642772, 0.82922992,
        ],
    ]);
    assert_eq!(matrix.determinant(), 0.03686399344413405);
}

#[test]
fn matrix_determinant_too_small() {
    let matrix = Matrix::from([[8.], [4.]]);
    assert_eq!(matrix.determinant(), 0.);
}
