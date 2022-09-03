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
fn matrix_determinant_too_small() {
    let matrix = Matrix::from([[8.], [4.]]);
    assert_eq!(matrix.determinant(), 0.);
}

#[test]
fn matrix_determinant_too_big() {
    let matrix = Matrix::from([
        [8., 5., -2., 4., 2.],
        [4., 2.5, 20., 4., 2.],
        [8., 5., 1., 4., 2.],
        [28., -4., 17., 1., 2.],
        [14., 8., 11., 9., 4.],
    ]);
    assert_eq!(matrix.determinant(), 0.);
}
