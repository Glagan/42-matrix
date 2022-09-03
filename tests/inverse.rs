use matrix::matrix::Matrix;

#[test]
fn matrix_inverse_example_1() {
    let matrix = Matrix::from([[1., 2.], [3., 4.]]);
    let expected = &vec![vec![-2., 1.], vec![1.5, -0.5]];
    let inverse = matrix.inverse();
    assert!(inverse.is_ok());
    assert_eq!(inverse.unwrap().all(), expected);
}

#[test]
fn matrix_inverse_example_2() {
    let matrix = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    let expected = &vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![0.0, 0.0, 1.0],
    ];
    let inverse = matrix.inverse();
    assert!(inverse.is_ok());
    assert_eq!(inverse.unwrap().all(), expected);
}

#[test]
fn matrix_inverse_example_3() {
    let matrix = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
    let expected = &vec![
        vec![0.5, 0.0, 0.0],
        vec![0.0, 0.5, 0.0],
        vec![0.0, 0.0, 0.5],
    ];
    let inverse = matrix.inverse();
    assert!(inverse.is_ok());
    assert_eq!(inverse.unwrap().all(), expected);
}

#[test]
fn matrix_inverse_example_4() {
    let matrix = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
    let expected = &vec![
        vec![0.6494252873563219, 0.09770114942528735, -0.6551724137931034],
        vec![
            -0.7816091954022988,
            -0.12643678160919541,
            0.9655172413793103,
        ],
        vec![
            0.14367816091954022,
            0.07471264367816091,
            -0.2068965517241379,
        ],
    ];
    let inverse = matrix.inverse();
    assert!(inverse.is_ok());
    assert_eq!(inverse.unwrap().all(), expected);
}

#[test]
fn matrix_inverse_empty() {
    let matrix = Matrix::new([0, 0]);
    let inverse = matrix.inverse();
    assert!(inverse.is_ok());
    assert_eq!(inverse.unwrap().all().len(), 0);
}

#[test]
fn matrix_inverse_point() {
    let matrix = Matrix::from([[1.]]);
    let expected = &vec![vec![1.]];
    let inverse = matrix.inverse();
    assert!(inverse.is_ok());
    assert_eq!(inverse.unwrap().all(), expected);
}

#[test]
fn matrix_inverse_singular_matrix() {
    let matrix = Matrix::from([[2., 1., 1.], [0., 2., -2.], [1., 1., 0.]]);
    let inverse = matrix.inverse();
    assert!(inverse.is_err());
}
