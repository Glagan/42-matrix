use matrix::matrix::Matrix;

#[test]
fn trace_example_1() {
    let e1 = Matrix::from([[1., 0.], [0., 1.]]);
    assert_eq!(e1.trace(), 2.);
}

#[test]
fn trace_example_2() {
    let e1 = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
    assert_eq!(e1.trace(), 9.);
}

#[test]
fn trace_example_3() {
    let e1 = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
    assert_eq!(e1.trace(), -21.);
}
