use matrix::matrix::Matrix;

#[test]
fn matrix_row_echelon_example_1() {
    let matrix = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    let expected = &vec![vec![1., 0., 0.], vec![0., 1., 0.], vec![0., 0., 1.]];
    assert_eq!(matrix.row_echelon().all(), expected);
}

#[test]
fn matrix_row_echelon_example_2() {
    let matrix = Matrix::from([[1., 2.], [3., 4.]]);
    let expected = &vec![vec![1., 0.], vec![0., 1.]];
    assert_eq!(matrix.row_echelon().all(), expected);
}

#[test]
fn matrix_row_echelon_example_3() {
    let matrix = Matrix::from([[1., 2.], [2., 4.]]);
    let expected = &vec![vec![1., 2.], vec![0., 0.]];
    assert_eq!(matrix.row_echelon().all(), expected);
}

#[test]
fn matrix_row_echelon_example_4() {
    let matrix = Matrix::from([
        [8., 5., -2., 4., 28.],
        [4., 2.5, 20., 4., -4.],
        [8., 5., 1., 4., 17.],
    ]);
    let expected = &vec![
        vec![1., 0.625, 0., 0., -12.166666666666668],
        vec![0., 0., 1., 0., -3.666666666666667],
        vec![0., 0., 0., 1., 29.500000000000004],
    ];
    assert_eq!(matrix.row_echelon().all(), expected);
}
