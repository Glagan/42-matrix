use matrix::matrix::Matrix;
use matrix::vector::Vector;

#[test]
fn sub_vectors() {
    let mut vector_a = Vector::from([1., 2., 3.]);
    let vector_b = Vector::from([1., 2., 3.]);

    vector_a.sub(&vector_b);
    // match vector_a.sub(&vector_b) {
    //     Ok(_) => assert!(true),
    //     Err(_) => assert!(false, "Failed to sub vectors"),
    // }

    let expected = &vec![0., 0., 0.];
    assert_eq!(vector_a.all(), expected);
}

#[test]
fn sub_invalid_vectors() {
    let mut vector_a = Vector::from([1., 2., 3.]);
    let vector_b = Vector::from([1.]);

    vector_a.sub(&vector_b);
    // match vector_a.sub(&vector_b) {
    //     Ok(_) => assert!(false, "Failed to fail to sub vectors"),
    //     Err(_) => assert!(true),
    // }

    let expected = &vec![1., 2., 3.];
    assert_eq!(vector_a.all(), expected);
}

#[test]
fn sub_matrices() {
    let mut matrix_a = Matrix::from([[1., 2., 3.], [4., 5., 6.]]);
    let matrix_b = Matrix::from([[1., 2., 3.], [4., 5., 6.]]);

    matrix_a.sub(&matrix_b);
    // match matrix_a.sub(&matrix_b) {
    //     Ok(_) => assert!(true),
    //     Err(_) => assert!(false, "Failed to sub matrices"),
    // }

    let expected = &vec![vec![0., 0., 0.], vec![0., 0., 0.]];
    assert_eq!(matrix_a.all(), expected);
}

#[test]
fn sub_invalid_matrices() {
    let mut matrix_a = Matrix::from([[1., 2., 3.], [4., 5., 6.]]);
    let matrix_b = Matrix::from([[1.]]);

    matrix_a.sub(&matrix_b);
    // match matrix_a.sub(&matrix_b) {
    //     Ok(_) => assert!(false, "Failed to fail to sub matrices"),
    //     Err(_) => assert!(true),
    // }

    let expected = &vec![vec![1., 2., 3.], vec![4., 5., 6.]];
    assert_eq!(matrix_a.all(), expected);
}
