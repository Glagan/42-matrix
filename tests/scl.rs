use matrix::matrix::Matrix;
use matrix::vector::Vector;

#[test]
fn scl_vector() {
    let mut vector = Vector::from([1., 2., 3.]);
    vector.scl(2.);
    let expected = &vec![2., 4., 6.];
    assert_eq!(vector.all(), expected);
}

#[test]
fn scl_matrix() {
    let mut matrix = Matrix::from([[1., 2., 3.], [4., 5., 6.]]);
    matrix.scl(2.);
    let expected = &vec![vec![2., 4., 6.], vec![8., 10., 12.]];
    assert_eq!(matrix.all(), expected);
}
