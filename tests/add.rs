use matrix::matrix::Matrix;
use matrix::vector::Vector;

#[test]
fn add_vectors() {
    let mut vector_a = Vector::from([1., 2., 3.]);
    let vector_b = Vector::from([1., 2., 3.]);

    vector_a.add(&vector_b);
    // match vector_a.add(&vector_b) {
    //     Ok(_) => assert!(true),
    //     Err(_) => assert!(false, "Failed to add vectors"),
    // }

    let expected = &vec![2., 4., 6.];
    assert_eq!(vector_a.all(), expected);
}

#[test]
fn add_invalid_vectors() {
    let mut vector_a = Vector::from([1., 2., 3.]);
    let vector_b = Vector::from([1.]);

    vector_a.add(&vector_b);
    // match vector_a.add(&vector_b) {
    //     Ok(_) => assert!(false, "Failed to fail to add vectors"),
    //     Err(_) => assert!(true),
    // }

    let expected = &vec![1., 2., 3.];
    assert_eq!(vector_a.all(), expected);
}

#[test]
fn add_matrices() {
    let mut matrix_a = Matrix::from([[1., 2., 3.], [4., 5., 6.]]);
    let matrix_b = Matrix::from([[1., 2., 3.], [4., 5., 6.]]);

    matrix_a.add(&matrix_b);
    // match matrix_a.add(&matrix_b) {
    //     Ok(_) => assert!(true),
    //     Err(_) => assert!(false, "Failed to add matrices"),
    // }

    let expected = &vec![vec![2., 4., 6.], vec![8., 10., 12.]];
    assert_eq!(matrix_a.all(), expected);
}

#[test]
fn add_invalid_matrices() {
    let mut matrix_a = Matrix::from([[1., 2., 3.], [4., 5., 6.]]);
    let matrix_b = Matrix::from([[1.]]);

    matrix_a.add(&matrix_b);
    // match matrix_a.add(&matrix_b) {
    //     Ok(_) => assert!(false, "Failed to fail to add matrices"),
    //     Err(_) => assert!(true),
    // }

    let expected = &vec![vec![1., 2., 3.], vec![4., 5., 6.]];
    assert_eq!(matrix_a.all(), expected);
}
