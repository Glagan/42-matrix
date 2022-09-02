use matrix::matrix::Matrix;
use matrix::vector::Vector;

#[test]
fn matrix_vector_multiplication_example_1() {
    let matrix = Matrix::from([[1., 0.], [0., 1.]]);
    let vector = Vector::from([4., 2.]);

    let expected = &vec![4., 2.];
    assert_eq!(matrix.mul_vec(&vector).all(), expected);
}

#[test]
fn matrix_vector_multiplication_example_2() {
    let matrix = Matrix::from([[2., 0.], [0., 2.]]);
    let vector = Vector::from([4., 2.]);

    let expected = &vec![8., 4.];
    assert_eq!(matrix.mul_vec(&vector).all(), expected);
}

#[test]
fn matrix_vector_multiplication_example_3() {
    let matrix = Matrix::from([[2., -2.], [-2., 2.]]);
    let vector = Vector::from([4., 2.]);

    let expected = &vec![4., -4.];
    assert_eq!(matrix.mul_vec(&vector).all(), expected);
}

#[test]
fn matrix_vector_multiplication_invalid() {
    let matrix = Matrix::from([[2., -2.], [-2., 2.]]);
    let vector = Vector::from([4., 2., 3.]);

    let expected: &Vec<f64> = &vec![];
    assert_eq!(matrix.mul_vec(&vector).all(), expected);
}

#[test]
fn matrix_matrix_multiplication_example_1() {
    let matrix = Matrix::from([[1., 0.], [0., 1.]]);
    let other_matrix = Matrix::from([[1., 0.], [0., 1.]]);

    let expected = &vec![vec![1., 0.], vec![0., 1.]];
    assert_eq!(matrix.mul_mat(&other_matrix).all(), expected);
}

#[test]
fn matrix_matrix_multiplication_example_2() {
    let matrix = Matrix::from([[1., 0.], [0., 1.]]);
    let other_matrix = Matrix::from([[2., 1.], [4., 2.]]);

    let expected = &vec![vec![2., 1.], vec![4., 2.]];
    assert_eq!(matrix.mul_mat(&other_matrix).all(), expected);
}

#[test]
fn matrix_matrix_multiplication_example_3() {
    let matrix = Matrix::from([[3., -5.], [6., 8.]]);
    let other_matrix = Matrix::from([[2., 1.], [4., 2.]]);

    let expected = &vec![vec![-14., -7.], vec![44., 22.]];
    assert_eq!(matrix.mul_mat(&other_matrix).all(), expected);
}

#[test]
fn matrix_matrix_multiplication_invalid() {
    let matrix = Matrix::from([[3., -5.], [6., 8.]]);
    let other_matrix = Matrix::from([[2., 1., 2.], [4., 2., 2.]]);

    let expected: &Vec<Vec<f64>> = &vec![];
    assert_eq!(matrix.mul_mat(&other_matrix).all(), expected);
}
