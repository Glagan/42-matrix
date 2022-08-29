use matrix::linear_interpolation::lerp;
use matrix::matrix::Matrix;
use matrix::vector::Vector;

#[test]
fn linear_interpolation_numbers() {
    assert_eq!(lerp(&0., &1., 0.), 0.0);
    assert_eq!(lerp(&0., &1., 1.), 1.0);
    assert_eq!(lerp(&0., &1., 0.5), 0.5);
    assert_eq!(lerp(&21., &42., 0.3), 27.299999999999997);
}

#[test]
fn linear_interpolation_vectors() {
    let e1 = Vector::from([1., 0., 0.]);
    let e2 = Vector::from([0., 1., 0.]);

    let result = lerp(&e1, &e2, 0.);
    assert_eq!(result.all(), e1.all());
    let result = lerp(&e1, &e2, 1.);
    assert_eq!(result.all(), e2.all());
    let result = lerp(&e1, &e2, 0.5);
    assert_eq!(result.all(), &[0.5, 0.5, 0.]);
}

#[test]
fn linear_interpolation_matrices() {
    let e1 = Matrix::from([[1., 0., 0.], [0., 0., 0.]]);
    let e2 = Matrix::from([[0., 1., 0.], [1., 0., 1.]]);

    let result = lerp(&e1, &e2, 0.);
    assert_eq!(result.all(), e1.all());
    let result = lerp(&e1, &e2, 1.);
    assert_eq!(result.all(), e2.all());
    let result = lerp(&e1, &e2, 0.5);
    assert_eq!(result.all(), &[[0.5, 0.5, 0.], [0.5, 0., 0.5]]);
}

#[test]
fn linear_interpolation_vector_example_1() {
    let e1 = Vector::from([2., 1.]);
    let e2 = Vector::from([4., 2.]);

    let result = lerp(&e1, &e2, 0.3);
    assert_eq!(result.all(), &[2.5999999999999996, 1.2999999999999998]);
}

#[test]
fn linear_interpolation_matrix_example_1() {
    let e1 = Matrix::from([[2., 1.], [3., 4.]]);
    let e2 = Matrix::from([[20., 10.], [30., 40.]]);

    let result = lerp(&e1, &e2, 0.5);
    assert_eq!(result.all(), &[[11.0, 5.5], [16.5, 22.0]]);
}

#[test]
fn linear_interpolation_vector_invalid_example() {
    let e1 = Vector::from([2., 1., 3.]);
    let e2 = Vector::from([4., 2.]);

    let result = lerp(&e1, &e2, 0.0);
    assert_eq!(result.size(), 0);
    let result = lerp(&e1, &e2, 1.0);
    assert_eq!(result.size(), 0);
    let result = lerp(&e1, &e2, 0.3);
    assert_eq!(result.size(), 0);
}

#[test]
fn linear_interpolation_matrix_invalid_example() {
    let e1 = Matrix::from([[2., 1., 0.], [3., 4., 0.]]);
    let e2 = Matrix::from([[20., 10.], [30., 40.]]);

    let result = lerp(&e1, &e2, 0.0);
    assert_eq!(result.shape()[0], 0);
    assert_eq!(result.shape()[1], 0);
    let result = lerp(&e1, &e2, 1.0);
    assert_eq!(result.shape()[0], 0);
    assert_eq!(result.shape()[1], 0);
    let result = lerp(&e1, &e2, 0.3);
    assert_eq!(result.shape()[0], 0);
    assert_eq!(result.shape()[1], 0);
}
