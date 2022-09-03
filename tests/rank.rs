use matrix::matrix::Matrix;

#[test]
fn matrix_rank_example_1() {
    let matrix = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    assert_eq!(matrix.rank(), 3);
}

#[test]
fn matrix_rank_example_2() {
    let matrix = Matrix::from([[1., 2., 0., 0.], [2., 4., 0., 0.], [-1., 2., 1., 1.]]);
    assert_eq!(matrix.rank(), 2);
}

#[test]
fn matrix_rank_example_3() {
    let matrix = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.], [21., 18., 7.]]);
    assert_eq!(matrix.rank(), 3);
}

#[test]
fn matrix_rank_invalid() {
    let matrix = Matrix::<f64>::new([0, 0]);
    assert_eq!(matrix.rank(), 0);
}
