use matrix::matrix::Matrix;

#[test]
fn matrix_tranpose_example_1() {
    let matrix = Matrix::from([[1., 2.], [3., 1.]]);
    let expected = &vec![vec![1., 3.], vec![2., 1.]];
    assert_eq!(matrix.transpose().all(), expected);
}

#[test]
fn matrix_tranpose_example_2() {
    let matrix = Matrix::from([[1., 2., 3.], [4., 5., 6.]]);
    let expected = &vec![vec![1.0, 4.0], vec![2.0, 5.0], vec![3.0, 6.0]];
    assert_eq!(matrix.transpose().all(), expected);
}

#[test]
fn matrix_tranpose_example_3() {
    let matrix = Matrix::from([[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]]);
    let expected = &vec![
        vec![1.0, 4.0, 7.0],
        vec![2.0, 5.0, 8.0],
        vec![3.0, 6.0, 9.0],
    ];
    assert_eq!(matrix.transpose().all(), expected);
}
